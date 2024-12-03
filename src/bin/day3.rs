use std::io::{self, Read};
use regex::Regex;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let re = Regex::new(r"mul\((\d+),(\d+)\)|do\(\)|don't\(\)").unwrap();

    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Failed to read from stdin");

    let mut res1 = 0;
    let mut res2 = 0;

    let mut enabled = true;

    for c in re.captures_iter(&input) {
        match &c[0] {
            "do()" => enabled = true,
            "don't()" => enabled = false,
            _ => {
                let m = c[1].parse::<usize>()? * c[2].parse::<usize>()?;
                res1 += m;
                if enabled { res2 += m }
            }
        }
    }

    println!("{}", res1);
    println!("{}", res2);

    Ok(())
}
