use rand::prelude::*;
use std::time::SystemTime;

pub fn measure_throughput() {
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
