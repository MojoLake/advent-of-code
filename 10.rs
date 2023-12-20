use std::fs::File;
use std::io::{self, Read};
use std::convert::TryInto;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::cmp;

fn main() -> io::Result<()> {

    let mut file = File::open("tin")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let mut grid: Vec<Vec<char>> = Vec::new();

    for line in contents.split('\n') {
        let line: Vec<char> = line.chars().collect();
        grid.push(line);
    }

    let n = grid.len();
    let m = grid[0].len();

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

    let vector_rotations: HashMap<((i32, i32), char), (i32, i32)> = [
        (((1, 0), '7'), (0, -1)),
        (((0, 1), '7'), (-1, 0)),
        (((-1, 0), '7'), (0, 1)),
        (((0, -1), '7'), (1, 0)),
        (((0, -1), 'J'), (-1, 0)),
        (((1, 0), 'J'), (0, 1)),
        (((0, 1), 'J'), (1, 0)),
        (((-1, 0), 'J'), (0, -1)),
        (((-1, 0), 'L'), (0, 1)),
        (((1, 0), 'L'), (0, -1)),
        (((0, -1), 'L'), (1, 0)),
        (((0, 1), 'L'), (-1, 0)),
        (((0, 1), 'F'), (1, 0)),
        (((0, -1), 'F'), (-1, 0)),
        (((-1, 0), 'F'), (0, -1)),
        (((1, 0), 'F'), (0, 1)),
    ].to_vec().into_iter().collect();

    let mut q: VecDeque<(i32, i32, u32)> = VecDeque::new();

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
        let yy: usize = y as usize;
        let xx: usize = x as usize;

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

    let mut direction: Vec<Vec<(i32, i32)>> = vec![vec![(0, 0); m]; n];
    let mut vis_two: Vec<Vec<bool>> = vec![vec![false; m]; n];

    let mut y = sy;
    let mut x = sx + 1;

    println!("moi");
    grid[sy as usize][sx as usize] = 'F';
    direction[y as usize][x as usize] = (1, 0);
    println!("{}", grid[y as usize][x as usize]);

    loop {

        let yy: usize = y as usize;
        let xx: usize = x as usize;
        vis_two[yy][xx] = true;

        let mut found = false;

        for (ty, tx) in &dirs[&grid[yy][xx]] {
            let ny: i32 = y + ty;
            let nx: i32 = x + tx;

            println!("{ny}, {nx}");

            if ny < 0 || ny >= grid.len() as i32 || nx < 0 || nx >= grid[0].len() as i32 {
                continue;
            } 

            let ny: usize = ny as usize;
            let nx: usize = nx as usize;

            if !vis[ny][nx] || vis_two[ny][nx] {
                continue;
            }

            found = true;
            println!("{:?}", direction[yy][xx]);
            println!("{}", grid[ny][nx]);

            if grid[ny][nx] == '-' || grid[ny][nx] == '|' {
                direction[ny][nx] = direction[yy][xx];
            }
            else { 
                direction[ny][nx] = vector_rotations[&(direction[yy][xx], grid[ny][nx])];
            }

            y = ny as i32;
            x = nx as i32;
            break;
        }
        if !found {
            break;
        }
    }

    println!("Calculating answer...");
    let mut ans2 = 0;
    for (i, line) in grid.iter().enumerate() {
        let mut am = 0;
        for (j, c) in line.iter().enumerate() {

            // println!("{i}, {j}");

            if vis[i][j] && (grid[i][j] == '-' || grid[i][j] == '|') {
                if direction[i][j] == (0, 1) {
                    am += 1;
                }
                if direction[i][j] == (0, -1) {
                    am -= 1;
                }
            } else if !vis[i][j] {
                if am != 0 {
                    ans2 += 1;
                    println!("{i}, {j}");
                }
            }

            assert!(am < 2);
        }
    }
    println!("{ans2}");


    for i in 0..n {
        for j in 0..m {
            if !vis[i][j] {
                // print!("#");
            } else {
                // print!("{}", grid[i][j]);
            }
        }
        // println!();
    }

    println!("{ans}");

    Ok(())
}


// fn part2(vis: &Vec<Vec<bool>>, ) -> u32 {



//     let mut ans: u32 = 0;
//     ans
// }



