use std::{io, io::prelude::*};

fn main() {
    let mut x = 0;
    let mut y = 0;
    let mut num_trees = 0;

    let mut forest: Vec<_> = io::stdin()
        .lock()
        .lines()
        .map(|line| line.unwrap())
        .collect();

    loop {
        x += 3;
        y += 1;
        if y >= forest.len() {
            break;
        }
        x = x % forest[y].len();
        if &forest[y][x..x + 1] == "#" {
            num_trees += 1;
            forest[y] = format!("{}X{}", &forest[y][..x], &forest[y][x + 1..]);
        } else {
            forest[y] = format!("{}O{}", &forest[y][..x], &forest[y][x + 1..]);
        }
    }
    for line in forest {
        println!("{}", line);
    }
    println!("Number of trees: {}", num_trees);
}
