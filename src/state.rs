use std::time::SystemTime;
mod hero;
mod map;

pub struct GlobalState {
    time_passed: time::SystemTime,
    heroes: Vec<Hero>,
    map: MapGrid,
}
