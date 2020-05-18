use crate::geometry;
use crate::geometry::Geometry;
use rand::prelude::*;
use std::convert::TryInto;
use std::error::Error;
use std::fmt;

type Rectangle = geometry::Rectangle;
type Size = geometry::Size;
type Point = geometry::Point;

impl Error for SpawnObjectError {}

impl fmt::Display for SpawnObjectError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "SpawnObjectError")
    }
}

#[derive(Debug)]
pub struct SpawnObjectError {}

#[derive(Debug)]
pub struct Map {
    blocks: Vec<geometry::Rectangle>,
    tile: Size,
    resolution: Size,
}

pub struct MapGrid(pub [[u32; 20]; 30]);

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
        let map_height = tile.height * TILES_HEIGHT;

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
        if self.is_point_outside_of_map(&rect.start) || self.is_point_outside_of_map(&rect.end) {
            return true;
        }

        for block in self.blocks.iter() {
            if Map::is_point_inside_tile(block, &rect.start)
                || Map::is_point_inside_tile(block, &rect.end)
            {
                return true;
            }
        }

        false
    }

    pub fn find_space_for_rectangle(
        &self,
        rect: &Rectangle,
    ) -> Result<Rectangle, SpawnObjectError> {
        // Looks for tile that has enough space to fit rectangle above it
        // Randomly iterate over all tiles and see if it can fit somewhere
        let all_block_ids = vec![0..self.blocks.len()];
        let mut used_blocks = Vec::with_capacity(all_block_ids.len());

        loop {
            let random_tile: usize = rand::thread_rng().gen_range(0, self.blocks.len() + 1);

            if used_blocks.len() == all_block_ids.len() {
                // we have tried every possible position above
                // tiles and could not fit the rect
                return Err(SpawnObjectError {});
            }

            if used_blocks.contains(&random_tile) {
                // random generator yields id which we already tested
                continue;
            }

            used_blocks.push(random_tile);

            let width: i32 = (rect.size().width).try_into().unwrap();
            let height: i32 = (rect.size().height).try_into().unwrap();

            let position_rect_above_tile = Rectangle {
                start: Point {
                    x: self.blocks.get(random_tile).unwrap().start.x,
                    y: self.blocks.get(random_tile).unwrap().start.y + height,
                },
                end: Point {
                    x: self.blocks.get(random_tile).unwrap().start.x + width,
                    y: self.blocks.get(random_tile).unwrap().start.y,
                },
            };
            if self.does_collide_with_map(&position_rect_above_tile) {
                return Ok(position_rect_above_tile);
            } else {
                continue;
            }
        }
    }

    pub fn add_object_to_map(&self, _rect: Rectangle) -> bool {
        true
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

    fn is_point_outside_of_map(&self, point: &Point) -> bool {
        let width: i32 = self.resolution.width.try_into().unwrap();
        let height: i32 = self.resolution.height.try_into().unwrap();
        point.x > width || point.y > height
    }
}
