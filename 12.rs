use std::fs::File;
use std::io::{self, Read};


fn main() -> io::Result<()> {

    let mut file = File::open("in")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let mut v: Vec<Vec<char>> = Vec::new();
    let mut nums: Vec<Vec<u32>> = Vec::new();

    let mut ans: u32 = 0;

    for line in contents.lines() {
        v.push(line.split(' ').next().unwrap().chars().collect());
        nums.push(line.split(' ').last().unwrap().split(',').map(|x| x.parse().unwrap()).collect());

        let mut v2 = String::new();
        ans += part1(v.last().unwrap(), nums.last().unwrap(), &mut v2, 0);

        // println!("{:?}", v.last().unwrap());
        // println!("{:?}", nums.last().unwrap());
    }

    println!("{ans}");

    Ok(())
}


fn part1(v: &Vec<char>, nums: &Vec<u32>, v2: &mut String, ind: usize) -> u32 {

    if ind == v.len() {
        // println!("sup");
        // println!("{v2}");
        let mut vv: Vec<&str> = v2.split('.').collect();
        vv.retain(|s| !s.is_empty());
        // println!("{:?}", vv);
        // println!("moi");
        // println!("{:?}", v);
        if vv.len() != nums.len() {
            return 0;
        }
        for i in 0..vv.len() {
            if nums[i] as usize != vv[i].len() {
                return 0;
            }
        }
        return 1;
    }

    let mut ret: u32 = 0;

    if v[ind] == '?' {
        v2.push('#');
        ret += part1(v, nums, v2, ind + 1);
        v2.pop();
        v2.push('.');
        ret += part1(v, nums, v2, ind + 1);
        v2.pop();
    } else {
        v2.push(v[ind]);
        ret += part1(v, nums, v2, ind + 1);
        v2.pop();
    }

    ret
}