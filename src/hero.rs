use crate::geometry;
use crate::physics;
use std::f64::consts::PI;

pub struct Hero {
    // hero is 3 tiles in height and 1 tile in width
    pub position: geometry::Rectangle,
    pub applied_forces: Vec<physics::DirectionalForce>,
    pub cursorAngle: f64,
}

impl physics::Physics for Hero {
    fn get_next_tick_position(&self) -> geometry::Rectangle {}
}

impl Hero {
    pub fn new() -> Hero {
        Hero {
            position: geometry::Rectangle::new(0, 0, 30, 60),
            applied_forces: vec![physics::DirectionalForce {
                force: 1.0,
                angle: PI / 2.0,
                attenuation: 1.0, // g force baby
            }],
            cursorAngle: 180.0,
        }
    }
}
