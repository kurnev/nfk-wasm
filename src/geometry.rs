#[derive(Debug)]
pub struct Rectangle {
    pub x: u32, // left-top, same in DOM canvas
    pub y: u32,
    pub width: u32,
    pub height: u32,
}

impl Rectangle {
    pub fn new(x: u32, y: u32, width: u32, height: u32) -> Rectangle {
        Rectangle {
            x,
            y,
            width,
            height,
        }
    }
}

pub trait Geometry {
    fn area(&self) -> u32;
}

impl Geometry for Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

