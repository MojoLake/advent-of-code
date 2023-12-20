use std::fs::File;
use std::io::{self, Read};
use std::cmp::{max, min};

fn main() -> io::Result<()> {

    let mut file = File::open("in")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let mut v: Vec<Vec<char>> = Vec::new();

    for line in contents.split('\n') {
        v.push(line.chars().collect());
    }

    let n = v.len();
    let m = v.last().expect("no last element!").len();

    let mut rows: Vec<i64> = vec![0; n];
    let mut cols: Vec<i64> = vec![999999; m];

    let mut coords: Vec<(isize, isize)> = Vec::new();

    for (i, row) in v.iter().enumerate() {
        let mut found = false;
        
        for (j, c) in row.iter().enumerate() {
            if *c == '#' {
                coords.push((i as isize, j as isize));
                cols[j] = 0;
                found = true;
            }
        }

        if !found {
            rows[i] = 999999;
        }
    }

    for i in 1..rows.len() {
        rows[i] += rows[i-1];
    }
    
    for j in 1..cols.len() {
        cols[j] += cols[j-1];
    }

    let mut ans1: i64 = 0;
    for i in 0..coords.len() - 1 {
        let (y1, x1) = coords[i];
        for j in i+1..coords.len() {
            let (y2, x2) = coords[j];

            ans1 += (y1 - y2).abs() as i64 + (x1 - x2).abs() as i64 + pfx(&rows, min(y1, y2), max(y1, y2)) + pfx(&cols, min(x1, x2), max(x1, x2));
        }
    }
    println!("{ans1}");

    Ok(())
}


fn pfx(p: &Vec<i64>, a: isize, b: isize) -> i64 {
    if a == 0 { p[b as usize] } else { p[b as usize] - p[(a-1) as usize] }
}