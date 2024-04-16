use std::io::{self, Write};

#[derive(Debug)]
struct Grid<Tile> {
    width: usize,
    height: usize,
    tiles: Vec<Tile>,
}

// Tile types used with grid must implement clone
// so that in our new function we can safely duplicate default_value
impl<Tile> Grid<Tile> where Tile: Clone {
    fn new(width: usize, height: usize, default_value: Tile) -> Self {
        let tiles = vec![default_value; width * height];
        Grid { width, height, tiles }
    }

    fn get(&self, x: usize, y: usize) -> Option<&Tile> {
        if x < self.width && y < self.height {
            self.tiles.get(y * self.width + x)
        } else {
            None
        }
    }

    fn set(&mut self, x: usize, y: usize, value: Tile) {
        if x < self.width && y < self.height {
            let index = y * self.width + x;
            self.tiles[index] = value;
        }
    }
}
#[derive(Debug)]
struct Dungeon {
    tiles: Grid<Tile>
}

impl Dungeon {
    fn new(width: usize, height: usize) -> Self {
        let tile = Tile { tile_type: TileType::Floor, position: (0, 0) };
        let tiles = Grid::new(width, height, tile); // Defaulting all tiles to Floor
        Dungeon { tiles }
    }
}

#[derive(Debug, Clone)]
struct Tile {
    tile_type: TileType,
    position: (usize, usize)
}

#[derive(Debug,Clone,PartialEq)]
enum TileType {
    Wall,
    Floor,
    Door
}

//------
#[derive(Debug)]
struct Entity {
    entity_type: EntityType,
    position: (usize, usize)
}

#[derive(Debug)]
enum EntityType {
    Player,
    Monster,
    Item
}

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right

}

//-------

#[derive(Debug)]
pub struct World {
    dungeon: Dungeon,
    entities: Vec<Entity>
}

impl World {
    pub fn new() -> Self {
        // ------------------------------ Dungeon Initialization ------------------- //
        let dungeon_width = 10;
        let dungeon_height = 10;
        let mut dungeon = Dungeon::new(dungeon_width, dungeon_height);

        // set dungeon walls
        for x in 0..dungeon_width {
            let tile_top_row = Tile { tile_type: TileType::Wall, position: (x as usize, 0) };
            let tile_bottom_row = Tile { tile_type: TileType::Wall, position: (x as usize, dungeon_height-1) };
            dungeon.tiles.set(x,0, tile_top_row);
            dungeon.tiles.set(x,dungeon_height-1, tile_bottom_row);
        }

        for y in 0..dungeon_height {
            let tile_left_column = Tile { tile_type: TileType::Wall, position: (0, y as usize) };
            let tile_right_column = Tile { tile_type: TileType::Wall, position: (dungeon_width-1, y as usize) };
            dungeon.tiles.set(0, y, tile_left_column);
            dungeon.tiles.set(dungeon_width-1, y, tile_right_column);
        }

        // ------------------------------ Entity Initialization ------------------- //
        let mut player = Entity {
            entity_type: EntityType::Player,
            position: (1, 1)
        };

        let mut monster = Entity {
            entity_type: EntityType::Monster,
            position: (5, 5)
        };

        let mut potion = Entity {
            entity_type: EntityType::Item,
            position: (3, 3)
        };

        let entities = vec![player, monster, potion];

        // ------------------------------------------------------------------------ //
        World {
            dungeon,
            entities
        }
    }

    pub fn start_game_loop(&mut self) {
        let stdin = io::stdin();
        let mut input = String::new();

        loop {
            println!("Enter move (WASD): ");
            io::stdout().flush().unwrap();
            stdin.read_line(&mut input).unwrap();

            let direction = match input.trim() {
                "w" => Direction::Up,
                "a" => Direction::Left,
                "s" => Direction::Down,
                "d" => Direction::Right,
                _ => { println!("Invalid input, defaulting to up");
                    Direction::Up
                }
            };
            self.update(&direction);
            input.clear();
        }
    }

    fn update(&mut self, direction: &Direction) {
        // update game entities and world state
        for i in 0..self.entities.len() {
            match self.entities[i].entity_type {
                EntityType::Player => {
                    let (new_x, new_y) = self.determine_new_position(self.entities[i].position, direction);

                    if let Some(tile) = self.dungeon.tiles.get(new_x, new_y) {
                        if tile.tile_type == TileType::Floor {
                            self.entities[i].position = (new_x, new_y);
                            println!("Player moved to {:?}", self.entities[i].position);
                        }
                    }
                }
                EntityType::Monster => {
                    // monster logic
                }
                EntityType::Item => {
                    // item logic
                }
            }
        }
    }

    fn determine_new_position(&self, position: (usize, usize), direction: &Direction) -> (usize, usize) {
        match direction {
            Direction::Up => {
                (position.0, position.1 + 1)
            }
            Direction::Down => {
                (position.0, position.1 - 1)
            }
            Direction::Left => {
                (position.0 - 1, position.1)
            }
            Direction::Right => {
                (position.0 + 1, position.1)
            }
        }
    }

    fn handle_interactions(&self) {
        // handle interactions between entities
    }

}