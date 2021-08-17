use chrono::prelude::*;
use chrono::{Duration, NaiveTime};

//
// Will keep track of how long it has been between frame renders
//

pub struct FrameTiming {
    last_frame: NaiveTime
}

impl FrameTiming {
    pub fn new() -> Self {
        Self {
            last_frame: Utc::now().time()
        }
    }

    pub fn update_frame_time(&mut self) {
        self.last_frame = Utc::now().time();
    }

    pub fn delta_frame_time(&self) -> Duration {
        Utc::now().time().signed_duration_since(self.last_frame)
    }
}