mod common;
mod simple_map;

use super::{map, map::TileType, rect::Rect, spawner, Map, Position, World};
use simple_map::SimpleMapBuilder;

trait MapBuilder {
    fn build(new_depth: i32) -> (Map, Position);
    fn spawn(map: &mut Map, ecs: &mut World, new_depth: i32);
}

pub fn build_random_map(new_depth: i32) -> (Map, Position) {
    SimpleMapBuilder::build(new_depth)
}

pub fn spawn(map: &mut Map, ecs: &mut World, new_depth: i32) {
    SimpleMapBuilder::spawn(map, ecs, new_depth);
}
