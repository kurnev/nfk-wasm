use crate::geometry;

// Force is applied to center of the object
pub struct DirectionalForce {
    pub force: f64,
    pub angle: f64,       // 0 at right, full circle is pi
    pub attenuation: f64, // how fast force weakens     % / 100
}

pub trait Physics {
    fn get_next_tick_position(&self) -> geometry::Rectangle;
}
