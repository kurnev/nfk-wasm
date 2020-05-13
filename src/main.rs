mod map;

use rand::prelude::*;
use std::convert::TryInto;
use std::time::SystemTime;

pub struct MapGrid([[u32; 20]; 30]);

fn main() {
    let map = Map::new(MapGrid { 0: map::GRID });
    //println!("Result map is {:?}", map);

    let rect = Rectangle {
        start: Point { x: 540, y: 300 },
        end: Point { x: 580, y: 250 },
    };
    assert_eq!(map.does_collide_with_map(&rect), true);

    let rect = Rectangle {
        start: Point { x: 100, y: 285 },
        end: Point { x: 110, y: 280 },
    };
    assert_eq!(map.does_collide_with_map(&rect), false);

    let rect = Rectangle {
        start: Point { x: 220, y: 30 },
        end: Point { x: 270, y: 20 },
    };
    assert_eq!(map.does_collide_with_map(&rect), false);

    measure_throughput();
}

#[derive(Debug)]
struct Rectangle {
    start: Point, // left-top
    end: Point,   // right-bottom
}

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Size {
    width: u32,
    height: u32,
}

trait Geometry {
    fn area(&self) -> u32;
}

impl Geometry for Rectangle {
    fn area(&self) -> u32 {
        ((self.start.x - self.end.x) * (self.start.y - self.end.y))
            .abs()
            .try_into()
            .unwrap()
    }
}

#[derive(Debug)]
struct Map {
    blocks: Vec<Rectangle>,
    tile: Size,
    resolution: Size,
}

impl Map {
    pub fn new(grid: MapGrid) -> Map {
        let mut blocks: Vec<Rectangle> = Vec::new();
        let tile: Size = Size {
            width: 30,
            height: 10,
        };
        // TODO: use real map width and height instead of hardcoded
        const TILES_WIDTH: u32 = 20;
        const TILES_HEIGHT: u32 = 30;

        let map_width = tile.width * TILES_WIDTH;
        let map_height = tile.width * TILES_HEIGHT;

        // convert map grid to renderable object
        for (row_id, row) in grid.0.iter().enumerate() {
            for (cell_id, cell) in row.iter().enumerate() {
                let cell_id: u32 = cell_id.try_into().unwrap();
                let row_id: u32 = row_id.try_into().unwrap();
                // 1 - there is a block
                // 0 - there is an empty space
                if cell.eq(&1) {
                    let start = Point {
                        x: (cell_id * tile.width).try_into().unwrap(),
                        y: ((TILES_HEIGHT - row_id) * tile.height).try_into().unwrap(),
                    };
                    let end = Point {
                        x: ((cell_id + 1) * tile.width).try_into().unwrap(),
                        y: (((TILES_HEIGHT - row_id) - 1) * tile.height)
                            .try_into()
                            .unwrap(),
                    };
                    blocks.push(Rectangle { start, end });
                }
            }
        }

        Map {
            blocks,
            tile: Size {
                width: 30,
                height: 10,
            },
            resolution: Size {
                width: map_width,
                height: map_height,
            },
        }
    }

    pub fn does_collide_with_map(&self, rect: &Rectangle) -> bool {
        for block in self.blocks.iter() {
            if Map::is_point_inside_tile(block, &rect.start)
                || Map::is_point_inside_tile(block, &rect.end)
            {
                return true;
            }
        }

        false
    }

    fn is_point_inside_tile(source_rect: &Rectangle, target_point: &Point) -> bool {
        if target_point.x >= source_rect.start.x
            && target_point.x <= source_rect.end.x
            && target_point.y <= source_rect.start.y
            && target_point.y >= source_rect.end.y
        {
            return true;
        }
        false
    }
}

fn measure_throughput() {
    let map = Map::new(MapGrid { 0: map::GRID });
    let mut count = 0;
    let mut now = SystemTime::now();

    loop {
        let start_x = rand::thread_rng().gen_range(0, 600);
        let start_y = rand::thread_rng().gen_range(0, 300);

        let end_x = rand::thread_rng().gen_range(0, 600);
        let end_y = rand::thread_rng().gen_range(0, 300);

        let rect = Rectangle {
            start: Point {
                x: start_x,
                y: start_y,
            },
            end: Point { x: end_x, y: end_y },
        };
        map.does_collide_with_map(&rect);
        count += 1;

        if now.elapsed().unwrap().as_secs() == 1 {
            println!("Completed {} operations in second", count);
            count = 0;
            now = SystemTime::now();
        }
    }
}
