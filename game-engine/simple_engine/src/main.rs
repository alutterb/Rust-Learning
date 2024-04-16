mod world;

fn main() {
    let mut world = world::World::new();
    world.start_game_loop()
}
