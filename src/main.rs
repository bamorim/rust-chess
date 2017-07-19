#[macro_use] extern crate scan_rules;
mod board;
use std::io::{self, Write};

fn main() {
    let mut b = board::Board::new();
    loop {
        println!("{:}", b);
        print!("M> ");
        io::stdout().flush().unwrap();

        readln! {
            (let i, ",", let j, " ", let ni, ",", let nj) => b.move_piece(i,j,ni,nj),
            (let i, ",", let j) => println!("Possible Moves: {:?}",b.moves_for(i, j)),
            (.._) => println!("Invalid move"),
        }
    }
}
