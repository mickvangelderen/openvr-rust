
pub use openvr_sys as sys;

mod types;
mod enums;

pub use self::types::*;
pub use self::enums::*;

use std::ptr;
use std::os::raw::c_char;
use std::sync::atomic::{AtomicBool, Ordering};
use std::mem::MaybeUninit;

static INITIALIZED: AtomicBool = AtomicBool::new(false);

pub struct Context {
    system: System,
    compositor: Compositor,
}

impl Context {
    pub fn new(ty: ApplicationType) -> Result<Self, InitError> {
        if INITIALIZED.compare_and_swap(false, true, Ordering::Acquire) {
            panic!("OpenVR can only be initialized once.");
        }

        unsafe {
            let mut error = sys::EVRInitError_VRInitError_None;
            sys::VR_InitInternal(&mut error, ty as sys::EVRApplicationType);
            if let Some(error) = InitError::from_sys(error) {
                INITIALIZED.store(false, Ordering::Release);
                return Err(error);
            }
        };

        Ok(Context {
            system: System::new().unwrap(),
            compositor: Compositor::new().unwrap(),
        })
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

#[derive(Debug, Copy, Clone)]
pub struct RawProjection {
    pub l: f32,
    pub r: f32,
    pub b: f32,
    pub t: f32,
}

impl System {
    fn new() -> Result<Self, InitError> {
        let mut magic = Vec::from(b"FnTable:".as_ref());
        magic.extend(sys::IVRSystem_Version.as_ref());

        unsafe {
            let mut error = sys::EVRInitError_VRInitError_None;
            let fn_table = sys::VR_GetGenericInterface(magic.as_ptr() as *const c_char, &mut error)
                as *const sys::VR_IVRSystem_FnTable;
            if let Some(error) = InitError::from_sys(error) {
                return Err(error);
            }
            Ok(System {
                fn_table: {
                    if fn_table.is_null() {
                        panic!("Unexpected null pointer.");
                    }
                    *fn_table
                },
            })
        }
    }

    pub fn get_recommended_render_target_size(&self) -> Dimensions {
        unsafe {
            let mut width = MaybeUninit::<u32>::uninit();
            let mut height = MaybeUninit::<u32>::uninit();
            self.fn_table.GetRecommendedRenderTargetSize.unwrap()(width.as_mut_ptr(), height.as_mut_ptr());
            Dimensions { width: width.assume_init(), height: height.assume_init() }
        }
    }

    pub fn get_projection_matrix(&self, eye: Eye, z_near: f32, z_far: f32) -> [[f32; 4]; 4] {
        unsafe {
            self.fn_table.GetProjectionMatrix.unwrap()(eye.into_sys(), z_near, z_far).m
        }
    }

    pub fn get_projection_raw(&self, eye: Eye) -> RawProjection {
        unsafe {
            let mut l = MaybeUninit::<f32>::uninit();
            let mut r = MaybeUninit::<f32>::uninit();
            let mut b = MaybeUninit::<f32>::uninit();
            let mut t = MaybeUninit::<f32>::uninit();
            self.fn_table.GetProjectionRaw.unwrap()(eye.into_sys(), l.as_mut_ptr(), r.as_mut_ptr(), b.as_mut_ptr(), t.as_mut_ptr());
            RawProjection {
                l: l.assume_init(),
                r: r.assume_init(),
                b: b.assume_init(),
                t: t.assume_init(),
            }
        }
    }

    pub fn get_eye_to_head_transform(&self, eye: Eye) -> [[f32; 4]; 3] {
        unsafe {
            self.fn_table.GetEyeToHeadTransform.unwrap()(eye.into_sys()).m
        }
    }

    pub fn poll_next_event(&self) -> Option<Event> {
        unsafe {
            let mut event = MaybeUninit::<sys::VREvent_t>::uninit();
            
            if self.fn_table.PollNextEvent.unwrap()(event.as_mut_ptr(), std::mem::size_of::<sys::VREvent_t>() as u32) {
                let event = event.assume_init();
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
    fn new() -> Result<Self, InitError> {
        let mut magic = Vec::from(b"FnTable:".as_ref());
        magic.extend(sys::IVRCompositor_Version.as_ref());

        unsafe {
            let mut error = sys::EVRInitError_VRInitError_None;
            let fn_table = sys::VR_GetGenericInterface(magic.as_ptr() as *const c_char, &mut error)
                as *const sys::VR_IVRCompositor_FnTable;
            if let Some(error) = InitError::from_sys(error) {
                return Err(error);
            }
            Ok(Compositor {
                fn_table: {
                    if fn_table.is_null() {
                        panic!("Unexpected null pointer.");
                    }
                    *fn_table
                }
            })
        }
    }

    pub fn wait_get_poses(
        &self,
        poses: &mut [sys::TrackedDevicePose_t],
        predicted_poses: Option<&mut [sys::TrackedDevicePose_t]>,
    ) -> Result<(), CompositorError> {
        unsafe {
            let (pp_ptr, pp_len) = if let Some(pp) = predicted_poses {
                (pp.as_mut_ptr(), pp.len())
            } else {
                (std::ptr::null_mut(), 0)
            };
            
            match CompositorError::from_sys(self.fn_table.WaitGetPoses.unwrap()(
                poses.as_mut_ptr(),
                poses.len() as u32,
                pp_ptr,
                pp_len as u32,
            )) {
                Some(error) => Err(error),
                None => Ok(()),
            }
        }
    }

    pub fn submit(
        &self,
        eye: Eye,
        texture: &mut sys::Texture_t,
        bounds: Option<&mut sys::VRTextureBounds_t>,
        flags: SubmitFlag,
    ) -> Result<(), CompositorError> {
        unsafe {
            match CompositorError::from_sys(self.fn_table.Submit.unwrap()(
                eye as sys::EVREye,
                texture,
                if let Some(p) = bounds { p } else { ptr::null_mut() },
                flags.bits() as i32,
            )) {
                Some(error) => Err(error),
                None => Ok(()),
            }
        }
    }
}
