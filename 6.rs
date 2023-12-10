use std::fs::File;
use std::io::{self, Read};

fn main() -> io::Result<()> {

    let mut file = File::open("in")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let mut lines: Vec<&str> = contents.split('\n').collect();

    for i in 0..lines.len() {
        if let Some(x) = lines[i].split(':').nth(1) {
            lines[i] = x;
        }
        lines[i] = lines[i].trim();
    }

    let ans1 = part1(&lines);
    let ans2 = part2(&lines);

    println!("{ans1}, {ans2}");
    Ok(())
}

fn part1(lines: &Vec<&str>) -> i64 {
    let mut vlines: Vec<Vec<i64>> = Vec::new();
    for line in lines {
        let v: Vec<i64> = line.split_whitespace().filter_map(|s| s.parse().ok()).collect::<Vec<_>>();
        vlines.push(v);
    }

    let mut ans: i64 = 1;

    for i in 0..vlines[0].len() {
        let time = vlines[0][i];
        let rec = vlines[1][i];
        let mut am: i64 = 0;
        for x in 0..time+1 {
            if rec < time * x - x * x {
                am += 1;
            }
        }
        ans *= am;
    }
    ans
}

fn part2(lines: &Vec<&str>) -> i64 {

    let time: String = lines[0].chars()
                             .filter(|c| !c.is_whitespace())
                             .collect();
    let time: i64 = time.parse().unwrap();
    
    let rec: String = lines[1].chars()
                              .filter(|c| !c.is_whitespace())
                              .collect();
    let rec: i64 = rec.parse().unwrap();

    let mut ans: i64 = 0;

    for x in 0..time+1 {
        if rec < time * x - x * x {
            ans += 1;
        }
    }

    ans
}