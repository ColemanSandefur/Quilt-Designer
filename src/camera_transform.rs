use cairo::Context;

static MOVE_SPEED: f64 = 5.0;

struct KeysPressed {
    pub up: bool,
    pub right: bool,
    pub down: bool,
    pub left: bool,
}

impl KeysPressed {
    pub fn new() -> Self {
        Self {
            up: false,
            right: false,
            down: false,
            left: false,
        }
    }
}

pub struct CameraTransform {
    pub offset: (f64, f64),
    scale: f64,
    keys_pressed: KeysPressed
}

impl CameraTransform {
    pub const MIN_ZOOM: f64 = 0.1;

    pub fn new() -> Self {
        Self {
            offset: (0.0, 0.0),
            scale: 1.0,
            keys_pressed: KeysPressed::new(),
        }
    }

    pub fn apply_transformation(&self, cr: &Context) {
        cr.translate(self.offset.0, self.offset.1);
        cr.scale(self.scale, self.scale);
    }

    pub fn apply_zoom(&self, cr: &Context) {
        cr.scale(self.scale, self.scale);
    }

    pub fn apply_offset(&self, cr: &Context) {
        cr.translate(self.offset.0, self.offset.1);
    }

    pub fn get_zoom(&self) -> f64 {
        self.scale
    }

    pub fn start_move_left(&mut self) {self.keys_pressed.left = true;}
    pub fn start_move_right(&mut self) {self.keys_pressed.right = true;}
    pub fn start_move_up(&mut self) {self.keys_pressed.up = true;}
    pub fn start_move_down(&mut self) {self.keys_pressed.down = true;}

    pub fn stop_move_left(&mut self) {self.keys_pressed.left = false;}
    pub fn stop_move_right(&mut self) {self.keys_pressed.right = false;}
    pub fn stop_move_up(&mut self) {self.keys_pressed.up = false;}
    pub fn stop_move_down(&mut self) {self.keys_pressed.down = false;}

    pub fn move_with_keys_pressed(&mut self, delta_time: &chrono::Duration) -> bool {
        let time_passed = delta_time.num_milliseconds() as f64 / 4.0;
        let mut changed = false;

        if self.keys_pressed.left {
            self.offset.0 -= MOVE_SPEED * time_passed;
            changed = true;
        }

        if self.keys_pressed.right {
            self.offset.0 += MOVE_SPEED * time_passed;
            changed = true;
        }

        if self.keys_pressed.up {
            self.offset.1 -= MOVE_SPEED * time_passed;
            changed = true;
        }

        if self.keys_pressed.down {
            self.offset.1 += MOVE_SPEED * time_passed;
            changed = true;
        }

        changed
    }

    pub fn get_scale(&self) -> f64 {
        self.scale
    }

    pub fn set_scale(&mut self, value: f64) -> f64 {
        self.scale = match value {
            v if v <= CameraTransform::MIN_ZOOM => CameraTransform::MIN_ZOOM,
            v if v > CameraTransform::MIN_ZOOM => v,
            _ => CameraTransform::MIN_ZOOM,
        };

        self.scale
    }

    pub fn get_offset(&self) -> (f64, f64) {
        self.offset
    }
}