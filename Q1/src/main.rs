use std::fmt;

struct Game;

impl Game {
    fn bat(&mut self, runs: i32) {
        unimplemented!();
    }

    fn score(&self) -> i32 {
        unimplemented!();
    }
}

impl fmt::Display for Game {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        unimplemented!();
    }
}

fn main() {}
