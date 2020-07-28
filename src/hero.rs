use crate::geometry;
use std::f64::consts::PI;

pub struct Hero {
    // hero is 3 tiles in height and 1 tile in width
    pub position: geometry::Rectangle,
    pub applied_forces: Vec<DirectionalForce>,
}

impl Hero {
    pub fn new() -> Hero {
        Hero {
            position: geometry::Rectangle::new(0, 0, 30, 90),
            applied_forces: vec![DirectionalForce {
                force: 1.0,
                angle: PI / 2.0,
            }],
        }
    }
}

// Force is applied to center of the object
pub struct DirectionalForce {
    pub force: f64,
    pub angle: f64, // 0 at top, full circle is pi
}
