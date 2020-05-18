mod default_map;
mod geometry;
mod hero;
mod map;

type Rectangle = geometry::Rectangle;
type Point = geometry::Point;

fn main() {
    let map = map::Map::new(map::MapGrid {
        0: default_map::GRID,
    });
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

    let hero = hero::Hero::new();
    hero.spawn_hero(map);
}
