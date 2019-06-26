/*
	Sudoku solver written in Rust

	BSD 2-Clause License

	Copyright (c) 2019, Daniel Lorch
	All rights reserved.

	Redistribution and use in source and binary forms, with or without
	modification, are permitted provided that the following conditions are met:

	1. Redistributions of source code must retain the above copyright notice, this
	   list of conditions and the following disclaimer.

	2. Redistributions in binary form must reproduce the above copyright notice,
       this list of conditions and the following disclaimer in the documentation
       and/or other materials provided with the distribution.

	THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS"
	AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
	IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
	DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE LIABLE
	FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
	DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR
	SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER
	CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY,
	OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
	OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
*/

use std::collections::HashSet;

type SudokuPlayfield = [i32; 81];
const NO_MORE_VALUES: i32 = -1;

fn print_playfield(playfield: SudokuPlayfield) {
    for i in 0 .. 9 {
        if i % 3 == 0 {
            for _i in 1..20 {
                print!("-");
            }
            println!();
        }

        for j in 0 .. 9 {
            if j % 3 == 0 {
                print!("|")
            }

            if playfield[i*9 + j] == 0 {
                print!(" ");
            } else {
                print!("{}", playfield[i*9 + j]);
            }

            if j % 3 != 2 {
                print!(" ");
            }
        }
        println!("|");
    }

    for _i in 1 .. 20 {
        print!("-");
    }
    println!();
}

fn valid_options(index: usize, playfield: SudokuPlayfield) -> HashSet<i32> {
    let mut options = HashSet::new();

    if playfield[index] == 0 {
        for i in 0 .. 10 {
            options.insert(i);
        }

        // coordinates (row, col) of element
        let row: usize = index as usize / 9;
        let col: usize = index as usize % 9;

        for horizontal in 0 .. 9 {
            options.remove(&playfield[row * 9 + horizontal]);
        }

        for vertical in 0 .. 9 {
            options.remove(&playfield[vertical * 9 + col]);
        }

        // coordinates (square_row, square_col) of top-left element in 3x3 matrix
        let square_row = row - row % 3;
        let square_col = col - col % 3;

        for vertical in 0 .. 3 {
            for horizontal in 0 .. 3 {
                options.remove(&playfield[(square_row + vertical) * 9 + (square_col + horizontal)]);
            }
        }
    }

    return options;
}

fn find_next_index(playfield: SudokuPlayfield) -> i32 {
    let mut index: i32 = NO_MORE_VALUES;

    for i in 0 .. 81 {
        if playfield[i] == 0 {
            index = i as i32;
            break;
        }
    }

    return index;
}

// brute-force depth-first-search solver
fn solve_sudoku(input_playfield: SudokuPlayfield) -> Option<SudokuPlayfield> {
    let mut result: Option<SudokuPlayfield> = None;
    let index = find_next_index(input_playfield);

    if index == NO_MORE_VALUES {
        result = Some(input_playfield);
    } else {
        let options = valid_options(index as usize, input_playfield);
        let mut playfield = input_playfield.clone();
        let mut found_solution = false;

        for option in options {
            if !found_solution {
                playfield[index as usize] = option;
                let depth_first_search_result = solve_sudoku(playfield);

                if depth_first_search_result.is_some() {
                    result = depth_first_search_result;
                    found_solution = true;
                }
            }
        }

        if !found_solution {
            result = None; // all options exhausted -> backtrack
        }
    }

    return result; // return solution
}

fn main() {
    // Arrays in Rust are *value* types; they are allocated on the stack and copied when passing
    // them to a function. There is no need to handle ownership (borrowing etc.) for them.
    // https://github.com/aminb/rust-for-c/tree/master/arrays
    // https://doc.rust-lang.org/book/ch04-00-understanding-ownership.html

    let _playfield_easy: SudokuPlayfield =
    [
        6, 2, 5, 0, 0, 0, 7, 0, 0,
        0, 0, 8, 2, 9, 0, 5 ,0, 1,
        0, 0, 0, 6, 5, 0, 8, 4, 0,
        5, 6, 3, 0, 7, 0, 0, 0, 0,
        7, 0, 0, 9, 3, 0, 2, 0, 6,
        0, 0, 0, 0, 6, 1, 3, 0, 5,
        8, 0, 4, 7, 1, 0, 6, 0, 3,
        0, 9, 0, 0, 0, 6, 0, 0, 0,
        0, 7, 0, 0, 0, 0, 0, 5, 0
    ];

    // https://de.wikipedia.org/wiki/Sudoku
    let _playfield_wikipedia_de: SudokuPlayfield =
    [
        0, 3, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 1, 9, 5, 0 ,0, 0,
        0, 0, 8, 0, 0, 0, 0, 6, 0,
        8, 0, 0, 0, 6, 0, 0, 0, 0,
        4, 0, 0, 8, 0, 0, 0, 0, 1,
        0, 0, 0, 0, 2, 0, 0, 0, 0,
        0, 6, 0, 0, 0, 0, 2, 8, 0,
        0, 0, 0, 4, 1, 9, 0, 0, 5,
        0, 0, 0, 0, 0, 0, 0, 7, 0
    ];

    println!("Input:");
    print_playfield(_playfield_easy);

    let result = solve_sudoku(_playfield_wikipedia_de);
    match result {
        Some(solution) => {
            println!("Solution:");
            print_playfield(solution);
        },
        None => println!("No solution could be found."),
    };
}