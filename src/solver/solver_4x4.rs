// use crate::solver::toroid::{SphericalNode, ToroidList};
use crate::solver::io::{print_sudoku_4x4, read_sudoku_from_file, write_sudoku_to_file};

// 4x4 Sudoku has 16 pieces, each has four possible spots it can go into.
// 16 columns representing the pieces, 16 locations, 16 for column relation, 16 for box relation
// 64 total columns
const DIVIDER: usize = 16;
// each piece can go into 4 spots, so 16 * 4 = 64 rows
// 1111222233334444 is the piece ordering, the grid is represented in row major order

pub fn generate_empty_sudoku() -> Vec<Vec<u8>> {
    let mut sudoku: Vec<Vec<u8>> = vec![];

    for i in 0..64 {
        let mut current_row = vec![0; 64];

        current_row[i/4] = 1; // Setting the piece representation
        current_row[i%16 + 1 * DIVIDER] = 1; // Setting the piece location
        current_row[i%4 + 4*(i/16) + 2 * DIVIDER] = 1; // Setting Column representation
        
        // Setting box representation
        let k;
        if i % 4 > 1 {
            k = 1;
        } else {
            k = 0;
        }

        current_row[2 * (i/8) + k + 3*DIVIDER] = 1;

        sudoku.push(current_row);

    }

    _ = write_sudoku_to_file(&sudoku, "empty4x4.txt");

    sudoku
}

pub fn read_and_write() {
    let sudoku = read_sudoku_from_file("src/inputs/test4x4.txt");

    match sudoku {
        Ok(v) => {
            print_sudoku_4x4(&v);
        },
        Err(_) => {panic!()}
    }
}



/*

1           1000
1           1000
1           0100
1           0100
01          1000
01          1000
01          0100
01          0100
001         0010
001         0010
001         0001
001         0001
0001        0010
0001        0010
0001        0001
0001        0001

*/