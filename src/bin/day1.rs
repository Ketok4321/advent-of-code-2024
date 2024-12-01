use std::io::{self, BufRead};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut list1: Vec<isize> = Vec::new();
    let mut list2: Vec<isize> = Vec::new();

    for line in io::stdin().lock().lines() {
        match line {
            Ok(content) => {
                let s = content.split_whitespace().collect::<Vec<_>>();
                assert!(s.len() == 2);
                list1.push(s[0].parse::<isize>()?);
                list2.push(s[1].parse::<isize>()?);
            },
            Err(error) => eprintln!("Error reading input: {}", error),
        }
    }

    list1.sort();
    list2.sort();

    let res1 = list1.iter().zip(list2.iter()).fold(0, |acc, (a, b)| acc + (a - b).abs());
    let res2 = list1.iter().fold(0, |acc, n| acc + (*n as usize) * list2.iter().filter(|&x| x == n).count());

    println!("{}", res1);
    println!("{}", res2);

    Ok(())
}
