use rand::{Rng, thread_rng};

// define enum Direction
enum Direction {
    Up,
    Down,
    Left,
    Right,
}
// passing a mutable reference of location so that ownership doesnt switch over to the fn
fn walk(location: &mut Vec<Vec<i32>>, direction: Direction, coordinates: [i32; 2]) 
                  -> Result<(i32, i32), &'static str> {
    let mut arr = [0,0];
    let bound = location.len(); // gets number of rows for our dynamic array
    let boundi = bound as i32; // convert to i32 to check if indices in bound

    match direction { // matching enum value
        Direction::Up => arr[1] = 1,
        Direction::Down => arr[1] = -1,
        Direction::Left => arr[0] = -1,
        Direction::Right => arr[0] = 1,
    }
    // updated coordinates
    let x = coordinates[0] + arr[0];
    let y = coordinates[1] + arr[1];


    if x >= boundi || y >= boundi || x < 0 || y < 0 { // oob checking
        Err("Direction is out of bounds.")
    } else {
        // need to update to usize for indexing
        let ux = x as usize;
        let uy = y as usize;

         // do the same for original coordinates
        let ucoordx = coordinates[0] as usize;
        let ucoordy = coordinates[1] as usize;

        location[ucoordx][ucoordy] = 0; // set previous location to 0
        location[ux][uy] = 1; // update new location
        Ok((x,y))
    }
}

fn walk_result(location: &mut Vec<Vec<i32>>, direction: Direction, coordinates: [i32;2]) -> (i32, i32) {
    match walk(location, direction, coordinates) {
        Ok((x,y)) => (x,y),
        Err(_e) => (coordinates[0], coordinates[1])
    }
}

pub fn begin_adventure() {
    let n = 10; // size of our dynamic array
    let mut array: Vec<Vec<i32>> = vec![vec![0;n]; n]; // define n x n dynamic array, initializes values at 0
    // provide current location coordinates
    let mut coordinates: [i32; 2] = [2, 1];
    // set array indices of origin to 1, this is where we are starting
    array[coordinates[0] as usize][coordinates[1] as usize] = 1;

    // simulate walk
    let mut iters = 1000;
    while iters > 0 {
        // use random number generator to decide direction
        let mut rng = thread_rng();
        let number: f32 = rng.gen();

        // assing direction to move - equal chance to move any direction
        let direction = if number < 0.25 {
            Direction::Up
        } else if number < 0.5 {
            Direction::Down
        } else if number < 0.75 {
            Direction::Left
        } else {
            Direction::Right
        };
        
        let (x, y) = walk_result(&mut array, direction, coordinates);
        coordinates = [x, y];
        iters -= 1;
    }

    println!("Origin: ({}, {})", 2, 1);
    println!("Final Destination: ({}, {})", coordinates[0], coordinates[1]);
    println!("Location matrix: {:?}", array);
}