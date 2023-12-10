use std::fs::File;
use std::io::{self, Read};
use std::convert::TryInto;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::cmp;

fn main() -> io::Result<()> {

    let mut file = File::open("in")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let mut grid: Vec<Vec<char>> = Vec::new();

    for line in contents.split('\n') {
        let line: Vec<char> = line.chars().collect();
        grid.push(line);
    }

    let mut sy: i32 = 0;
    let mut sx: i32 = 0;
    for (i, line) in grid.iter().enumerate() {
        for (j, c) in line.iter().enumerate() {
            if *c == 'S' {
                sy = i.try_into().unwrap();
                sx = j.try_into().unwrap();
            }
        }
    }

    let dirs: HashMap<char, Vec<(i32, i32)>> = [
        ('|', vec![(1, 0), (-1, 0)]),
        ('-', vec![(0, 1), (0, -1)]),
        ('J', vec![(-1, 0), (0, -1)]),
        ('L', vec![(-1, 0), (0, 1)]),
        ('F', vec![(1, 0), (0, 1)]),
        ('7', vec![(1, 0), (0, -1)]),
    ].to_vec().into_iter().collect();

    let mut q: VecDeque<(i32, i32, u32)> = VecDeque::new();
    
    // if !['.', '-', 'F', '7'].contains(grid[sy-1][sx]) {
    //     q.push(
    // }

    let dy = [1, -1, 0, 0];
    let dx = [0, 0, 1, -1];
    let mut vis: Vec<Vec<bool>> = vec![vec![false; grid[0].len()]; grid.len()];
    vis[sy as usize][sx as usize] = true;

    for (ny, nx) in dy.iter().zip(dx.iter()) {
        let y = sy + ny;
        let x = sx + nx;

        if y < 0 || x < 0 {
            continue;
        }

        let yy: usize = y.try_into().unwrap();
        let xx: usize = x.try_into().unwrap();
        if grid[yy][xx] == '.' {
            continue;
        }

        for (ddy, ddx) in &dirs[&grid[yy][xx]] {
            let nny = y + ddy;
            let nnx = x + ddx;
            if (nny, nnx) == (sy, sx) {
                q.push_back((y, x, 1));
                vis[yy][xx] = true;
            }
            
        }
    }

    let mut ans: u32 = 0;
    while !q.is_empty() {
        let (y, x, d) = q.pop_front().unwrap();
        // q.pop_front();
        let yy: usize = y as usize;
        let xx: usize = x as usize;

        // println!("{y}, {x}");

        // if vis[yy][xx] {
        //     continue;
        // }

        for (dy, dx) in &dirs[&grid[yy][xx]] {
            let ny = y + dy;
            let nx = x + dx;

            if ny < 0 || nx < 0 || ny as usize > grid.len() || nx as usize > grid[0].len() {
                continue;
            }

            let nyy: usize = ny.try_into().unwrap();
            let nxx: usize = nx.try_into().unwrap();

            if vis[nyy][nxx] {
                continue;
            }
            vis[nyy][nxx] = true;

            ans = cmp::max(ans, d + 1); 

            q.push_back((ny, nx, d + 1));
        }
    }

    println!("{ans}");

    Ok(())
}



