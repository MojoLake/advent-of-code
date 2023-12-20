use std::fs::File;
use std::io::{self, Read};

fn main() -> io::Result<()> {

    let mut file = File::open("in")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let lines: Vec<&str> = contents.split("\n").collect();
    let mut v: Vec<Vec<i64>> = Vec::new();
    for line in lines {
        let line: Vec<i64> = line.split(" ").map(|x| x.parse().unwrap()).collect();
        v.push(line);
    }

    let mut ans: i64 = 0;
    let mut ans2: i64 = 0;
    for line in v {
        let mut pyr: Vec<Vec<i64>> = vec![line; 1];
        let mut all_zero = false;

        while !all_zero {

            let pv = &pyr[pyr.len() - 1];
            let mut nv: Vec<i64> = Vec::new();
            for i in 0..pv.len() - 1 {
                nv.push(pv[i+1] - pv[i]);
            }

            all_zero = nv.iter().all(|&x| x == 0);
            println!("{:?}", nv);
            if !all_zero {
                pyr.push(nv);
            }
        }

        let mut cur = 0;
        for nv in pyr.iter().rev() {
            cur = cur + nv[nv.len() - 1];
        }
        ans += cur;

        cur = 0;
        for nv in pyr.iter().rev() {
            cur = nv[0] - cur;
        }
        ans2 += cur;
    }
    println!("{ans}");
    println!("{ans2}");
    Ok(())
}