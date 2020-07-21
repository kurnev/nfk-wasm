extern crate web_sys;

use wasm_bindgen::prelude::*;
mod default_map;
mod geometry;
mod hero;
mod map;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub struct GlobalState {
    state: [u32; 8000],
}

#[wasm_bindgen]
impl GlobalState {
    pub fn tick(&mut self) {
        // One tick of universe
        // Get every movable object and apply force to it
        self.state[4001] += 1;
        self.state[4002] += 1
    }

    pub fn new() -> GlobalState {
        let mut state = [0 as u32; 8000];
        let blocks = &mut state[0..4000];
        map::allocate_map(blocks);

        GlobalState { state }
    }

    pub fn blocks_ptr(&self) -> *const u32 {
        self.state[0..8000].as_ptr()
    }

    pub fn spawn_hero(&mut self) {
        let hero = hero::Hero::new();
        let hero_position = map::spawn_hero(&self.state[0..4000], &hero).unwrap();

        self.state[4001] = hero_position.x;
        self.state[4002] = hero_position.y;
        self.state[4003] = hero_position.width;
        self.state[4004] = hero_position.height;
    }
}

//fn main() {

//// TODO: move these to tests
//let map = map::Map::new(map::MapGrid {
//0: default_map::GRID,
//});
////println!("Result map is {:?}", map);

//let rect = Rectangle {
//start: Point { x: 540, y: 300 },
//end: Point { x: 580, y: 250 },
//};
//assert_eq!(map.does_collide_with_map(&rect), true);

//let rect = Rectangle {
//start: Point { x: 100, y: 285 },
//end: Point { x: 110, y: 280 },
//};
//assert_eq!(map.does_collide_with_map(&rect), false);

//let rect = Rectangle {
//start: Point { x: 220, y: 30 },
//end: Point { x: 270, y: 20 },
//};
//assert_eq!(map.does_collide_with_map(&rect), false);

//let hero = hero::Hero::new();
//hero.spawn_hero(map);
//}
