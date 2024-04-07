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

#[derive(Debug,Clone)]
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

    pub fn update(&mut self) {
        // update game entities and world state
    }
}