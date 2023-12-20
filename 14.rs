use std::fs::File;
use std::io::{self, Read, Write};
use std::collections::HashMap;

fn main() -> io::Result<()> {

    let mut file = File::open("in")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let mut g: Vec<Vec<char>> = Vec::new();
    for line in contents.lines() {
        g.push(line.chars().collect());
    }

    let mut ans: i32 = 0;

    let mut vis: HashMap<Vec<Vec<char>>, i32> = HashMap::new();
    vis.insert(g.clone(), 0);

    let mm = 1000000000;
    let mut it = 1;
    while it <= mm {

        for i in 0..g.len() {
            for j in 0..g[0].len() {
                if g[i][j] == 'O' {
                    drop(&mut g, i as isize, -1, j as isize, 0);
                }
            }
        }

        for i in 0..g.len() {
            for j in 0..g[0].len() {
                if g[i][j] == 'O' {
                    drop(&mut g, i as isize, 0, j as isize, -1);
                }
            }
        }

        for i in (0..g.len()).rev() {
            for j in 0..g[0].len() {
                if g[i][j] == 'O' {
                    drop(&mut g, i as isize, 1, j as isize, 0);
                }
            }
        }

        for i in 0..g.len() {
            for j in (0..g[0].len()).rev() {
                if g[i][j] == 'O' {
                    drop(&mut g, i as isize, 0, j as isize, 1);
                }
            }
        }

        if vis.contains_key(&g) {
            let jump = it - vis.get(&g).unwrap();
            let mul = (mm - it) / jump;
            it += mul * jump;
        }

        vis.insert(g.clone(), it);
        it += 1;
    }

    for i in 0..g.len() {
        for j in 0..g[0].len() {
            if g[i][j] == 'O' {
                ans += (g.len() - i) as i32;
            }
        }
    }

    println!("{ans}");

    Ok(())
}

fn drop(g: &mut Vec<Vec<char>>, mut y: isize, dy: isize, mut x: isize, dx: isize) -> (isize, isize) {

    let n: isize = g.len() as isize;
    let m: isize = g[0].len() as isize;

    let ok = |i, j| i >= 0 && i < n && j >= 0 && j < m;

    while ok(y + dy, x + dx) {
        // render(g.clone());
        let uy = (y + dy) as usize;
        let ux = (x + dx) as usize;

        if g[uy][ux] != '.' {
            break;
        }

        g[y as usize][x as usize] = '.';
        g[uy][ux] = 'O';

        y = uy as isize;
        x = ux as isize;
    }
    (y, x)
}

fn render(g: Vec<Vec<char>>) -> () {
    let s = g.clone().into_iter().map(|v| v.into_iter().collect::<String>()).collect::<Vec<_>>().join("\n");
    let mut file = File::create("vis.txt").unwrap();
    file.write_all(s.as_bytes()).unwrap();
}

