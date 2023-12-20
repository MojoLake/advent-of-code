use std::fs::File;
use std::io::{self, Read};
use std::collections::VecDeque;
use std::convert::TryInto;

fn main() -> io::Result<()> {

    let mut file = File::open("in")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let mut ans1 = 0;
    let mut v: [VecDeque<(&str, u32)>; 256] = vec![VecDeque::new(); 256].try_into().expect("failed vec to arr!");

    for s in contents.split(',') {

        ans1 += hash(s);

        let parts: Vec<_> = s.split(|c| ['-', '='].contains(&c)).collect();

        let lab = parts[0];
        let b = hash(lab);
        let b = b as usize;
        
        if s.contains('-') {
            v[b].retain(|&x| x.0 != lab);
        } else {
            let fl: u32 = parts[1].parse().unwrap();

            let mut it = 999;
            for (i, (l, f)) in v[b].iter().enumerate() {
                if l == &lab {
                    it = i;
                    break;
                }
            }

            if it == 999 {
                v[b].push_back((lab, fl));
            } else {
                v[b][it] = (lab, fl);
            }
        }
    }

    let mut ans2: u32 = 0;
    for i in 0..256 {
        for (j, (lab, f)) in v[i].iter().enumerate() {
            // println!("{i}, {lab}");
            ans2 += (i + 1) as u32 * (j + 1) as u32 * f;
        }
    }

    println!("{ans1}");
    println!("{ans2}");

    Ok(())
}

fn hash(s: &str) -> u32 {
    let mut r: u32 = 0;
    for c in s.as_bytes() {
        r += *c as u32;
        r *= 17;
        r %= 256;
    }
    r
}