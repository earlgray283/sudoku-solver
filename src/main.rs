mod checker;
mod converter;

use std::time::Instant;
use checker::Checker;
use converter::Convertor;
use proconio::{input, marker::Chars};

const N: usize = 9;

fn main() {
    println!("Input 9x9 Grid with [0-9] or '?' >> ");
    input! {m: [Chars; N]}
    
    let mut m = m.to_vec_i32();

    let start = Instant::now();
    let res = dfs(&mut m, (0, 0));
    let stop = Instant::now();

    println!("took {:?}", stop.duration_since(start));

    if !res {
        println!("answer not found");
        return;
    }
    if !m.check_all((0, 0)) {
        println!("WA!w");
        return;
    }
    for i in 0..N {
        for j in 0..N {
            print!("{:3} ", m[i][j]);
        }
        println!();
    }
}

fn dfs(m: &mut Vec<Vec<i32>>, pos: (usize, usize)) -> bool {
    if pos.0 >= N {
        if dfs(m, (0, pos.1 + 1)) {
            return true;
        }
    } else if pos.1 >= N {
        return true;
    } else if m[pos.1][pos.0] != -1 {
        if dfs(m, (pos.0 + 1, pos.1)) {
            return true;
        }
    } else if m[pos.1][pos.0] == -1 {
        for i in 1..=9 {
            m[pos.1][pos.0] = i;
            if !m.check_all(pos) {
                continue;
            }

            if dfs(m, (pos.0 + 1, pos.1)) {
                return true;
            }
        }

        m[pos.1][pos.0] = -1;
    }    

    false
}
