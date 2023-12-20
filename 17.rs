use std::fs::File;
use std::io::{self, Read};

fn main() -> io::Result<()> {

    let mut file = File::open("tin")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let g: Vec<Vec<u32>> = contents.lines().map(|s| s.chars().map(|x| x.to_digit(10).unwrap()).collect()).collect();
    for line in g {
        for c in line {
            print!("{c}");
        }
        println!();
    }

    Ok(())
}