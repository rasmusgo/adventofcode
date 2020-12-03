use std::{io, io::prelude::*};

fn main() {
    let mut forest: Vec<_> = io::stdin()
        .lock()
        .lines()
        .map(|line| line.unwrap())
        .collect();

    let mut product: i64 = 1;
    let slopes = [[1, 1], [3, 1], [5, 1], [7, 1], [1, 2]];
    for [dx, dy] in &slopes {
        let mut x = 0;
        let mut y = 0;
        let mut num_trees = 0;
        loop {
            x += dx;
            y += dy;
            if y >= forest.len() {
                break;
            }
            x = x % forest[y].len();
            if &forest[y][x..x + 1] == "#" || &forest[y][x..x + 1] == "X" {
                num_trees += 1;
                forest[y] = format!("{}X{}", &forest[y][..x], &forest[y][x + 1..]);
            } else {
                forest[y] = format!("{}O{}", &forest[y][..x], &forest[y][x + 1..]);
            }
        }
        product *= num_trees;
        println!(
            "Number of trees using slope right {}, down {}: {}",
            dx, dy, num_trees
        );
    }
    println!("Product of number of trees: {}", product);
    for line in forest {
        println!("{}", line);
    }
}
