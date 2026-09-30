// use crate::solver::toroid;

// // Empty sudoku can be represented by 324 columns (81 pieces, 81 places, 81 row relations, 81 box relations)
// const DIVIDER: usize = 81;
// // each row will have a 1 in the location and in the piece
// // they will also have a 1 representing the row they are in as well as the column
// // first 9 pieces are 1's, next 9 are 2's, and so on.
// // the board is from the top left in row major order
// // rest are "empty"

// // Since any piece can only go in their respective row, that gives us 81 x 9 rows
// // Pieces are related to pieces of the same number and locations are related if in same box, row, or column

// fn generate_default_matrix() -> Vec<Vec<u8>> {

    // let mut result: Vec<Vec<u8>> = vec![];
    
    // for i in 0..81 {
        // let mut new_row: Vec<u8> = vec![0; 324];
        // new_row[i] = 1;

        // for j in 0..9 {
            // let mut temp = new_row.clone();

            // // Add row dependency
            // temp[DIVIDER + ((i%9)*9) + j] = 1;

            // // Add column dependency
            // temp[(2 * DIVIDER) + (i/9)*9 + j] = 1;

            // // Add box dependency
            // let k = (j / 3) * 3;
            // temp[(3 * DIVIDER) + (i/9)*9 + k] = 1;
            // temp[(3 * DIVIDER) + (i/9)*9 + k + 1] = 1;
            // temp[(3 * DIVIDER) + (i/9)*9 + k + 2] = 1;

            // result.push(temp);
        // }
    // }
    // result
// }

// fn create_quadly_linked_matrix(input: &[Vec<u8>]) -> node::QuadList {
    // let column_count = input.first().map_or(0, |row| row.len());
    // let data_node_count = input.iter().flatten().filter(|&&value| value == 1).count();
    // let mut list = node::QuadList::with_capacity(1 + column_count + data_node_count);

    // for column in 0..column_count {
        // list.add_column(format!("c{column}"));
    // }

    // for row in input {
        // let mut first_in_row: Option<node::NodeIndex> = None;
        // let mut previous_in_row: Option<node::NodeIndex> = None;

        // for (column, &value) in row.iter().enumerate() {
            // if value != 1 {
                // continue;
            // }

            // let node_index = list.add_data_node(column + 1);

            // if let Some(previous) = previous_in_row {
                // list.link_right(previous, node_index);
            // } else {
                // first_in_row = Some(node_index);
            // }

            // previous_in_row = Some(node_index);
        // }

        // if let (Some(first), Some(last)) = (first_in_row, previous_in_row) {
            // list.link_right(last, first);
        // }
    // }

    // list
// }

// use std::{collections::HashSet, env::temp_dir};

// fn apply_partial_set_to_generic(sudoku: Vec<Vec<u8>>, partial_solution: &mut HashSet::<Vec<Relation>>) -> Result<Vec<Vec<Relation>>, String> {
    // let mut converted: Vec<Vec<Relation>> = generate_default_matrix();

    // for line in sudoku {
        // let mut row = 0;
        // for value in line {
            // if value != 0 {
                
            // }
            // index += 1;
        // }
    // }

    // Ok(converted)
// }

// fn naive_algorthim_x(arr: Vec<Vec<u8>>, partial_solution: &mut HashSet::<Vec<u8>>) -> Vec<Vec<u8>> {
    // todo!()
// }

// pub fn solve_sudoku_naive(sudoku: Vec<Vec<u8>>) -> Result<Vec<Vec<u8>>, String> {
    // let mut partial_solution = HashSet::<Vec<u8>>::new();
    // let binary_array = convert_to_binary_array(sudoku, &mut partial_solution).unwrap();
    // let result = naive_algorthim_x(binary_array, &mut partial_solution);
    // Ok(result)
// }

// pub fn solve_sudoku_dlx(sudoku: Vec<Vec<u8>>) {

// }