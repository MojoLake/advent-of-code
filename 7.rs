use std::fs::File;
use std::io::{self, Read};
use std::cmp::Ordering;
use std::collections::HashMap;


fn main() -> io::Result<()> {

    let mut file = File::open("in")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let lines: Vec<&str> = contents.split('\n').collect();
    let mut hands: Vec<(&str, i64)> = Vec::new();

    for line in lines {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if let Ok(x) = parts[1].parse::<i64>() {
            hands.push((parts[0], x));
        } else {
            println!("Parsing failed!");
        }

        // println!("{}, {}", parts[0], parts[1]);
    }

    hands.sort_by(|a, b| comp(a.0, b.0));
    for (a, b) in &hands {
        println!("{a}, {b}");
    }
    let mut ans: i64 = 0;

    for i in 0..hands.len() {
        ans += (i as i64 + 1) * hands[i].1;
    }

    println!("{ans}");

    Ok(())
}
// 251551323


fn comp(a: &str, b: &str) -> Ordering {

    let values: HashMap<char, usize> = HashMap::from([
        ('2', 2),
        ('3', 3),
        ('4', 4),
        ('5', 5),
        ('6', 6),
        ('7', 7),
        ('8', 8),
        ('9', 9),
        ('T', 10),
        ('J', 1),
        ('Q', 12),
        ('K', 13),
        ('A', 14),
    ]);

    let mut fa: Vec<i64> = vec![0; 15];
    let mut fb: Vec<i64> = vec![0; 15];

    let mut aj = 0;
    let mut bj = 0;

    for c in a.chars() {
        if c == 'J' {
            aj += 1;
        } else {
            fa[values[&c]] += 1;
        }
    }
    for c in b.chars() {
        if c == 'J' {
            bj += 1;
        } else {
            fb[values[&c]] += 1;
        }
    }

    let mut va: Vec<i64> = Vec::new();
    let mut vb: Vec<i64> = Vec::new();
    for i in 2..15 {
        if fa[i] != 0 {
            va.push(fa[i]);
        }
        if fb[i] != 0 {
            vb.push(fb[i]);
        }
    }
    va.sort_by(|a, b| b.cmp(a));
    vb.sort_by(|a, b| b.cmp(a));
    
    if va.is_empty() {
        va.push(5);
    } else {
        va[0] += aj;
    }

    if vb.is_empty() {
        vb.push(5);
    } else {
        vb[0] += bj;
    }

    for i in 0..5 {
        if va.len() <= i {
            assert!(vb.len() >= 1);
            break;
        }
        if va[i] > vb[i] {
            return Ordering::Greater;
        }
        if va[i] < vb[i] {
            return Ordering::Less;
        }
    }

    let a: Vec<_> = a.chars().collect();
    let b: Vec<_> = b.chars().collect();

    for i in 0..5 {
        if values[&a[i]] > values[&b[i]] {
            return Ordering::Greater;
        }
        if values[&a[i]] < values[&b[i]] {
            return Ordering::Less;
        }
    }

    Ordering::Equal
}