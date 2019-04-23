pub mod enums;
pub mod types;

pub use self::enums::*;
pub use self::types::*;

pub use openvr_sys as sys;

use std::mem;
use std::ptr;
use std::os::raw::c_char;
use std::sync::atomic::{AtomicBool, Ordering};

static INITIALIZED: AtomicBool = AtomicBool::new(false);

pub struct Context {
    system: System,
    compositor: Compositor,
}

impl Context {
    pub fn new(ty: ApplicationType) -> Result<Self, Unchecked<InitError>> {
        if INITIALIZED.compare_and_swap(false, true, Ordering::Acquire) {
            panic!("OpenVR can only be initialized once.");
        }

        let error = unsafe {
            let mut error = sys::EVRInitError_VRInitError_None;
            sys::VR_InitInternal(&mut error, ty as sys::EVRApplicationType);
            Unchecked(error)
        };

        if error == InitError::None.into_unchecked() {
            Ok(Context {
                system: System::new().unwrap(),
                compositor: Compositor::new().unwrap(),
            })
        } else {
            INITIALIZED.store(false, Ordering::Release);
            Err(error)
        }
    }

    #[inline]
    pub fn system(&self) -> &System {
        &self.system
    }

    #[inline]
    pub fn compositor(&self) -> &Compositor {
        &self.compositor
    }
}

impl Drop for Context {
    fn drop(&mut self) {
        unsafe {
            sys::VR_ShutdownInternal();
        }
        INITIALIZED.store(false, Ordering::Release);
    }
}

pub struct System {
    fn_table: sys::VR_IVRSystem_FnTable,
}

impl System {
    fn new() -> Result<Self, Unchecked<InitError>> {
        let mut magic = Vec::from(b"FnTable:".as_ref());
        magic.extend(sys::IVRSystem_Version.as_ref());

        unsafe {
            let mut error = sys::EVRInitError_VRInitError_None;
            let fn_table = sys::VR_GetGenericInterface(magic.as_ptr() as *const c_char, &mut error)
                as *const sys::VR_IVRSystem_FnTable;
            let error = Unchecked(error);
            if error == InitError::None.into_unchecked() {
                Ok(System {
                    fn_table: {
                        if fn_table.is_null() {
                            panic!("Unexpected null pointer.");
                        }
                        *fn_table
                    },
                })
            } else {
                Err(error)
            }
        }
    }

    pub fn get_recommended_render_target_size(&self) -> Dimensions {
        unsafe {
            let mut width: u32 = mem::uninitialized();
            let mut height: u32 = mem::uninitialized();
            self.fn_table.GetRecommendedRenderTargetSize.unwrap()(&mut width, &mut height);
            Dimensions { width, height }
        }
    }

    pub fn get_projection_matrix(&self, eye: Eye, z_near: f32, z_far: f32) -> [[f32; 4]; 4] {
        unsafe {
            self.fn_table.GetProjectionMatrix.unwrap()(eye as u32, z_near, z_far).m
        }
    }

    pub fn get_projection_raw(&self, eye: Eye) -> [f32; 4] {
        unsafe {
            let mut out: [f32; 4] = mem::uninitialized();
            self.fn_table.GetProjectionRaw.unwrap()(eye as u32, &mut out[0], &mut out[1], &mut out[2], &mut out[3]);
            out
        }
    }

    pub fn get_eye_to_head_transform(&self, eye: Eye) -> [[f32; 4]; 3] {
        unsafe {
            self.fn_table.GetEyeToHeadTransform.unwrap()(eye as u32).m
        }
    }

    pub fn poll_next_event(&self) -> Option<Event> {
        unsafe {
            let mut event: sys::VREvent_t = mem::uninitialized();
            if self.fn_table.PollNextEvent.unwrap()(&mut event, mem::size_of_val(&event) as u32) {
                Some(Event {
                    tracked_device_index: event.trackedDeviceIndex,
                    event_age_seconds: event.eventAgeSeconds,
                })
            } else {
                None
            }
        }
    }
}

pub struct Compositor {
    pub fn_table: sys::VR_IVRCompositor_FnTable,
}

impl Compositor {
    fn new() -> Result<Self, Unchecked<InitError>> {
        let mut magic = Vec::from(b"FnTable:".as_ref());
        magic.extend(sys::IVRCompositor_Version.as_ref());

        unsafe {
            let mut error = sys::EVRInitError_VRInitError_None;
            let fn_table = sys::VR_GetGenericInterface(magic.as_ptr() as *const c_char, &mut error)
                as *const sys::VR_IVRCompositor_FnTable;
            let error = Unchecked(error);
            if error == InitError::None.into_unchecked() {
                Ok(Compositor {
                    fn_table: {
                        if fn_table.is_null() {
                            panic!("Unexpected null pointer.");
                        }
                        *fn_table
                    }
                })
            } else {
                Err(error)
            }
        }
    }

    pub fn wait_get_poses(
        &self,
        poses: &mut [sys::TrackedDevicePose_t],
        predicted_poses: Option<&mut [sys::TrackedDevicePose_t]>,
    ) -> Result<(), Unchecked<CompositorError>> {
        unsafe {
            let (pp_ptr, pp_len) = if let Some(pp) = predicted_poses {
                (pp.as_mut_ptr(), pp.len())
            } else {
                (std::ptr::null_mut(), 0)
            };
            let error = Unchecked(self.fn_table.WaitGetPoses.unwrap()(
                poses.as_mut_ptr(),
                poses.len() as u32,
                pp_ptr,
                pp_len as u32,
            ));
            if error == CompositorError::None.into_unchecked() {
                Ok(())
            } else {
                Err(error)
            }
        }
    }

    pub fn submit(
        &self,
        eye: Eye,
        texture: &mut sys::Texture_t,
        bounds: Option<&mut sys::VRTextureBounds_t>,
        flags: SubmitFlag,
    ) -> Result<(), Unchecked<CompositorError>> {
        unsafe {
            let error = Unchecked(self.fn_table.Submit.unwrap()(
                eye as sys::EVREye,
                texture,
                if let Some(p) = bounds { p } else { ptr::null_mut() },
                flags as sys::EVRSubmitFlags,
            ));
            if error == CompositorError::None.into_unchecked() {
                Ok(())
            } else {
                Err(error)
            }
        }
    }
}
