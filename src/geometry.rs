use std::convert::TryInto;

#[derive(Debug)]
pub struct Rectangle {
    pub start: Point, // left-top
    pub end: Point,   // right-bottom
}

impl Rectangle {
    pub fn new(width: u32, height: u32) -> Rectangle {
        Rectangle {
            start: Point {
                x: 0,
                y: height.try_into().unwrap(),
            },
            end: Point {
                x: width.try_into().unwrap(),
                y: 0,
            },
        }
    }
}

#[derive(Debug)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug)]
pub struct Size {
    pub width: u32,
    pub height: u32,
}

pub trait Geometry {
    fn area(&self) -> u32;
    fn size(&self) -> Size; // width and length
}

impl Geometry for Rectangle {
    fn area(&self) -> u32 {
        ((self.start.x - self.end.x) * (self.start.y - self.end.y))
            .abs()
            .try_into()
            .unwrap()
    }

    fn size(&self) -> Size {
        Size {
            width: (self.end.x - self.start.x).try_into().unwrap(),
            height: (self.start.y - self.end.y).try_into().unwrap(),
        }
    }
}

#[derive(Debug)]
pub struct Vector {
    start: Point,
    end: Point,
}
