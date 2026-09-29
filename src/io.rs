use std::fs;
use std::error::Error;
use std::io::{Error as IoError, ErrorKind, Write};
use std::io;

pub fn read_sudoku_from_file(filename: &str) -> Result<Vec<Vec<u8>>, Box<dyn Error>> {
    
    let raw = fs::read_to_string(filename)?;
    let mut sudoku: Vec<Vec<u8>> = vec![];

    for line in raw.lines() {
        let row: Result<Vec<u8>, IoError> = line
        .chars()
        .map(|c| {
            c.to_digit(10)
                .map(|digit| digit as u8)
                .ok_or_else(|| IoError::new(ErrorKind::InvalidData, "Char not converted to digit"))
        })
        .collect();
        let row = row?;

        sudoku.push(row);
    }

    Ok(sudoku)
}

pub fn print_sudoku(sudoku: &Vec<Vec<u8>>) {
    let mut outer_index = 0;
    for line in sudoku {
        if outer_index % 3 == 0 && outer_index != 0 {
            println!("------+-------+------");
        }
        let mut inner_index = 0;
        for int in line {
            if inner_index % 3 == 0 && inner_index != 0 {
                print!("| ");
            }
            print!("{} ", int);
            inner_index += 1;
            if inner_index == 9 {println!();}
        }
        outer_index += 1;
    }
}

pub fn _write_sudoku_to_file(sudoku: &Vec<Vec<u8>>, filename: &str) -> io::Result<()> {
    let mut new_file = fs::File::create(filename)?;

    for line in sudoku {    
        for int in line {
            let temp = int.to_string();
            write!(new_file, "{}", temp)?;
        }
        new_file.write(b"\n")?;
    }

    Ok(())
}