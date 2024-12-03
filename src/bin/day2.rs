use std::io::{self, BufRead};

fn is_diff_ok(dir: bool, a: isize, b: isize) -> bool {
    let d = a - b;
    (dir && d >= 1 && d <= 3 ) || (!dir && d <= -1 && d >= -3)
}

fn res2(r: &[isize], dir: bool) -> bool {
    if r.len() > 2 && !is_diff_ok(dir, r[0], r[1]) && !is_diff_ok(dir, r[0], r[2]) {
        for i in 2..r.len() {
            if !is_diff_ok(dir, r[i - 1], r[i]) {
                return false
            }
        }
        return true
    }

    for i in 1..r.len() {
        if !is_diff_ok(dir, r[i - 1], r[i]) {
            if i != r.len() - 1 && !is_diff_ok(dir, r[i - 1], r[i + 1]) {
                return false
            }
            for i in i+2..r.len() {
                if !is_diff_ok(dir, r[i - 1], r[i]) {
                    return false
                }
            }
            return true
        }
    }

    true
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut records: Vec<Vec<isize>> = Vec::new();

    for line in io::stdin().lock().lines() {
        match line {
            Ok(content) => {
                records.push(content.split_whitespace().map(str::parse::<isize>).collect::<Result<Vec<isize>, _>>()?);
            },
            Err(error) => eprintln!("Error reading input: {}", error),
        }
    }
    
    let res1 = records.iter().filter(|r| r.windows(2).all(|p| p[0] - p[1] >= 1 && p[0] - p[1] <= 3) || r.windows(2).all(|p| p[1] - p[0] >= 1 && p[1] - p[0] <= 3)).count();
    let res2 = records.iter().filter(|r| res2(r, false) || res2(r, true)).count();

    println!("{}", res1);
    println!("{}", res2);

    Ok(())
}
