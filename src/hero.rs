use crate::geometry;
use crate::map;
use std::f64::consts::PI;

pub struct Hero {
    // hero is 3 tiles in height and 1 tile in width
    position: geometry::Rectangle,
    applied_forces: Vec<DirectionalForce>,
}

impl Hero {
    pub fn new() -> Hero {
        Hero {
            position: geometry::Rectangle::new(20, 90),
            applied_forces: vec![DirectionalForce {
                force: 1.0,
                angle: PI / 2.0,
            }],
        }
    }

    pub fn spawn_hero(&self, map: map::Map) {
        let f = match map.find_space_for_rectangle(&self.position) {
            Ok(rect) => println!("Found space: {:?}", rect.start),
            Err(error) => panic!("Could not spawn hero on map {}", error),
        };
    }
}

struct DirectionalForce {
    force: f64,
    angle: f64, // 0 at top, full circle is pi
}
