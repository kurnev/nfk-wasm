extern crate web_sys;

use crate::default_map;
use crate::geometry;
use crate::hero;
use rand::prelude::*;
use std::convert::TryInto;
use std::error::Error;
use std::fmt;


type Rectangle = geometry::Rectangle;

impl Error for SpawnObjectError {}

impl fmt::Display for SpawnObjectError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "SpawnObjectError")
    }
}

#[derive(Debug)]
pub struct SpawnObjectError {}

#[derive(Debug)]
pub struct Map<'a> {
    blocks: &'a [u32],
}


pub fn allocate_map(blocks: &mut [u32]) {
    const TILE_WIDTH: u32 = 30;
    const TILE_HEIGHT: u32 = 20;
    const TILES_ON_WIDTH: u32 = 20;
    const _TILES_ON_HEIGHT: u32 = 30;
    const _MAP_WIDTH: u32 = 600; // in px
    const _MAP_HEIGHT: u32 = 600;

    let grid = default_map::GRID;

    // convert map grid to renderable object
    for (row_id, row) in grid.iter().enumerate() {
        for (column_id, column) in row.iter().enumerate() {
            let row_id: u32 = row_id.try_into().unwrap();
            let column_id: u32 = column_id.try_into().unwrap();
            // 1 - there is a block
            // 0 - there is an empty space
            let cell_id: usize = (row_id * TILES_ON_WIDTH + column_id).try_into().unwrap();
            if column.eq(&1) {
                // x and y
                blocks[cell_id] = TILE_WIDTH * column_id;
                blocks[cell_id + 1] = TILE_HEIGHT * row_id;
                // width and height
                blocks[cell_id + 2] = TILE_WIDTH;
                blocks[cell_id + 3] = TILE_HEIGHT;
                web_sys::console::log_1(&format!("HOPA {:?} {:?}", TILE_WIDTH, cell_id).into());
            }
        }
    }
}

pub fn does_collide_with_map(blocks: &[u32], rect: &Rectangle) -> bool {
    let end_x = rect.x + rect.width;
    let end_y = rect.y + rect.height;

    if is_point_outside_of_map(&rect.x, &rect.y)
        || is_point_outside_of_map(&end_x, &end_y)
    {
        return true;
    }

    // each rectangle is held in 4 bytes
    let iter_steps = blocks.len() / 4;

    for n in 0..iter_steps {
        if is_point_inside_tile(
            &Rectangle {
                x: blocks[n],
                y: blocks[n + 1],
                width: blocks[n + 2],
                height: blocks[n + 4],
            },
            &rect.x,
            &rect.y,
        ) || is_point_inside_tile(
            &Rectangle {
                x: blocks[n],
                y: blocks[n + 1],
                width: blocks[n + 2],
                height: blocks[n + 4],
            },
            &end_x,
            &end_y,
        ) {
            return true;
        }
    }

    false
}

pub fn spawn_hero(blocks: &[u32], hero: &hero::Hero) -> Result<Rectangle, SpawnObjectError> {
    match find_space_for_rectangle(blocks, &hero.position) {
        Ok(rect) => {
            println!("Found space: {:?} {:?}", rect.x, rect.y);
            return Ok(Rectangle {
                x: rect.x,
                y: rect.y,
                width: rect.width,
                height: rect.height,
            })
        }
        Err(error) => {
            panic!("Could not spawn hero on map {}", error);
        }
    };
}

fn is_point_inside_tile(source_rect: &Rectangle, x: &u32, y: &u32) -> bool {
    let end_x = source_rect.x + source_rect.width;
    let end_y = source_rect.y + source_rect.height;
    if x >= &source_rect.x && x <= &end_x && y <= &source_rect.y && y >= &end_y {
        return true;
    }
    false
}

fn is_point_outside_of_map(x: &u32, y: &u32) -> bool {
    const MAP_WIDTH: u32 = 600; // in px
    const MAP_HEIGHT: u32 = 600;
    x > &MAP_WIDTH || y > &MAP_HEIGHT
}

fn find_space_for_rectangle(
    blocks: &[u32],
    rect: &Rectangle,
) -> Result<Rectangle, SpawnObjectError> {
    // Looks for tile that has enough space to fit rectangle above it
    // Randomly iterate over all tiles and see if it can fit somewhere

    let iter_steps = blocks.len() / 4;

    let all_block_ids = vec![0..iter_steps];
    let mut used_blocks = Vec::with_capacity(all_block_ids.len());

    loop {
        let random_tile: usize = rand::thread_rng().gen_range(0, iter_steps + 1);

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

        let position_rect_above_tile = Rectangle {
            x: rect.x,
            y: rect.y + blocks[random_tile + 1],
            width: rect.width,
            height: rect.height,
        };

        if does_collide_with_map(blocks, &position_rect_above_tile) {
            return Ok(position_rect_above_tile);
        } else {
            continue;
        }
    }
}
