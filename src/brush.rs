

//will probably make all brushes const and just pass references to everything to avoid cloning
#[derive(Clone)]
pub struct Brush {
    color: Option<(f64, f64, f64)>,
    texture: Option<f64>
}

impl Brush {
    pub fn new() -> Self {
        Self {
            color: Some((1.0, 1.0, 1.0)),
            texture: None,
        }
    }

    pub fn new_color(color: (f64, f64, f64)) -> Self {
        Self {
            color: Some(color),
            texture: None,
        }
    }

    pub fn apply(&self, cr: &cairo::Context) {
        cr.save();

        if let Some(color) = self.color {
            cr.set_source_rgb(color.0, color.1, color.2);
            cr.fill();
        }

        //to be implemented
        // if let Some(texture) = self.texture {
            
        // }

        cr.restore();
    }
}