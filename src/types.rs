#[derive(Debug)]
pub struct Event {
    pub tracked_device_index: u32,
    pub event_age_seconds: f32,
}

// pub enum EventData {
// }

#[derive(Debug)]
pub struct Dimensions {
    pub width: u32,
    pub height: u32,
}
