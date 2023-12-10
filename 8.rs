use std::fs::File;
use std::io::{self, Read};
use std::collections::HashMap;

fn main() -> io::Result<()> {

    let mut file = File::open("in")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let lines: Vec<&str> = contents.split('\n').collect();
    let instructions = lines[0];
    println!("{instructions}");

    let mut graph: HashMap<&str, (&str, &str)> = HashMap::new();
    let mut cur: Vec<&str> = Vec::new();
    for i in 2..lines.len() {
        // let line: Vec<_> = lines[i].chars().collect();
        let line = lines[i];
        let s = &line[0..3];
        let l = &line[7..10];
        let r = &line[12..15];
        println!("{s} {l} {r}");
        graph.insert(s, (l, r));
        if s.chars().collect::<Vec<_>>()[2] == 'A' {
            cur.push(&s);
        }
    }

    let mut ans: u64 = 0;
    let mut fail: bool = true;
    println!("{}", cur.len());

    for i in 0..cur.len() {
        let mut cans: u64 = 0;
        let mut found: bool = false;
        while !found {
            for c in instructions.chars() {
                cans += 1;
                if c == 'L' {
                    cur[i] = graph[&cur[i]].0;
                } else {
                    cur[i] = graph[&cur[i]].1;
                }
                if cur[i].chars().collect::<Vec<_>>()[2] == 'Z' {
                    found = true;
                    break;
                }
            }
        }
        println!("{cans}");
    }

    // while fail {
    //     for c in instructions.chars() {
    //         fail = false;
    //         ans += 1;
    //         for i in 0..cur.len() {
    //             if c == 'L' {
    //                 cur[i] = graph[&cur[i]].0;
    //             } else {
    //                 cur[i] = graph[&cur[i]].1;
    //             }
    //             if cur[i].chars().collect::<Vec<_>>()[2] != 'Z' {
    //                 fail = true;
    //             }
    //             // println!("{}", cur[i]);
    //         }
    //         if !fail {
    //             break;
    //         }
    //         // println!("{cur}");

    //     }
    // }

    // println!("{ans}");


    Ok(())
}