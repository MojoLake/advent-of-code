use std::fs::File;
use std::io::{self, Read};

fn main() -> io::Result<()> {

    let mut file = File::open("in")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let mut ans: u32 = 0;
    let mut ans2: u32 = 0;
    for thing in contents.split("\n\n") {
        let mut g: Vec<Vec<char>> = thing.lines().map(|s| s.chars().collect()).collect();
        let mut g2 = transpose(&g);
        let mut r = calc(&g, 0);
        if r != 0 {
            ans += 100 * r;
            let rr = part2(&mut g, r);
            if rr != 0 {
                ans2 += 100 * rr;
            } else {
                let rrr = part2(&mut g2, 0);
                assert!(rrr != 0);
                ans2 += rrr;
            }
            continue;
        }
 
        r = calc(&g2, 0);
        // assert!(r != 0);
        ans += r;

        let rr = part2(&mut g2, r);
        if rr != 0 {
            ans2 += rr;
        } else {
            let rrr = part2(&mut g, 0);
            // println!("{r}");
            // for line in thing.lines() {
            //     println!("{line}");
            // }
            for v in &g2 {
                for c in v {
                    print!("{c}");
                }
                println!();
            }
            println!();
            assert!(rrr != 0);
            ans2 += 100 * rrr;
        }
    }
    println!("{ans}");
    println!("{ans2}");

    Ok(())
}

fn part2(g: &mut Vec<Vec<char>>, r: u32) -> u32 {


    for i in 0..g.len() {
        for j in 0..g[0].len() {
            // if i == 15 && j == 8 {
            //     println!("{}", g[i][j]);
            // }
            if g[i][j] == '#' { g[i][j] = '.' } else { g[i][j] = '#' };
            // if i == 15 && j == 8 {
            //     println!("{}", g[i][j]);
            // }
            let rr = calc(&g, r);
            // if i == 15 && j == 8 {
            //     println!("{rr}");
            // }
            if rr != 0 {
                if g[i][j] == '#' { g[i][j] = '.' } else { g[i][j] = '#' };
                return rr as u32;
            }
            if g[i][j] == '#' { g[i][j] = '.' } else { g[i][j] = '#' };
        }
    }
    0
}

fn calc(g: &Vec<Vec<char>>, r: u32) -> u32 {

    for i in 1..g.len() {
        let mut fail = false;
        for k in 0..i {
            let l = i - (k + 1);
            let r = i + k;

            if r >= g.len() {
                continue;
            }

            if i == 16 {
                // println!("{l}, {r}");
                // println!("{:?}", g[l]);
                // println!("{:?}", g[r]);
            }
            if g[l] != g[r] {
                fail = true;
            }
        }
        if !fail && i as u32 != r{
            return i as u32;
        }
    }
    0
}

fn transpose(g: &Vec<Vec<char>>) -> Vec<Vec<char>> {

    let mut g2: Vec<Vec<char>> = vec![vec!['0'; g.len()]; g[0].len()];

    for i in 0..g.len() {
        for j in 0..g[0].len() {
            g2[j][i] = g[i][j];
        }
    }
    g2
}