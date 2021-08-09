use lyon::math::{point, Point};
use lyon::path::Path;
use lyon::path::{ArcFlags};
use lyon::path::builder::SvgPathBuilder;
use lyon::geom::vector;
use lyon::geom::Angle;

#[derive(Clone)]
pub enum PathCommand {
    Move(Point),
    Line(Point),
    Arc {
        center: Point,
        radius: f32,
        start_angle: f32,
        end_angle: f32,
    },
    Close,
}

impl PathCommand {
    pub fn apply(&self, path: &mut lyon::path::builder::WithSvg<lyon::path::builder::Flattened<lyon::path::path::Builder>>) {
        match self {
            Self::Move(point) => {
                path.move_to(*point);
            },
            Self::Line(point) => {
                path.line_to(*point);
            },
            Self::Arc {center, radius, start_angle, end_angle} => {
                let radius = *radius;

                let total_angle = end_angle - start_angle;

                // You need to draw 2 arcs if you are doing a complete circle
                if total_angle == 2.0 * std::f32::consts::PI {
                    path.move_to(point(radius + center.x, center.y));
                    path.arc_to(vector(radius, radius), Angle {radians: 0.0}, ArcFlags::default(), point(-radius + center.x, center.y));
                    path.arc_to(vector(radius, radius), Angle {radians: 0.0}, ArcFlags::default(), point( radius + center.x, center.y));
                }

                let mut arc_flags = ArcFlags {
                    sweep: true, // which way to draw (false => Clockwise, true => Counter Clockwise)
                    large_arc: false,
                };

                // determines which direction the arc starts drawing, should draw clockwise when the total angle is negative
                // sweep goes clockwise when false
                if total_angle < 0.0 {
                    arc_flags.sweep = false;
                }

                if total_angle.abs() > std::f32::consts::PI {
                    arc_flags.large_arc = true;
                }

                let start_x = radius * start_angle.cos() + center.x;
                let start_y = radius * start_angle.sin() + center.y;

                let stop_x = radius * end_angle.cos() + center.x;
                let stop_y = radius * end_angle.sin() + center.y;

                path.move_to(point(start_x, start_y));
                path.arc_to(vector(radius, radius), Angle {radians: 0.0}, arc_flags, point(stop_x, stop_y));
            },
            Self::Close => {
                path.close();
            },
        }
    }
}

#[derive(Clone)]
pub struct ShapePath {
    path_history: Vec<PathCommand>
}

impl ShapePath {
    fn create_path() -> lyon::path::builder::WithSvg<lyon::path::builder::Flattened<lyon::path::path::Builder>> {
        Path::svg_builder().flattened(0.001)
    }

    pub fn new() -> Self {
        Self {
            path_history: Vec::new(),
        }
    }

    pub fn move_to(&mut self, point: Point) {
        self.path_history.push(PathCommand::Move(point));
    }

    pub fn line_to(&mut self, point: Point) {
        self.path_history.push(PathCommand::Line(point));
    }

    pub fn arc_to(&mut self, center: Point, radius: f32, start_angle: f32, end_angle: f32) {
        self.path_history.push(PathCommand::Arc {center, radius, start_angle, end_angle});
    }

    pub fn close(&mut self) {
        self.path_history.push(PathCommand::Close);
    }

    pub fn build_path(&self) -> lyon::path::Path {
        let mut path = Self::create_path();

        for command in &self.path_history {
            command.apply(&mut path);
        }

        path.build()
    } 
}