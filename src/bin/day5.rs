use std::io::{self, BufRead};
use std::collections::HashMap;
use std::cmp::Ordering;

fn is_correct(rules: &HashMap<usize, Vec<usize>>, update: &Vec<usize>) -> bool {
    for (i, page) in update.iter().enumerate() {
        if let Some(r) = rules.get(page) {
            for prev in &update[..i] {
                if r.contains(prev) {
                    return false;
                }
            }
        }
    }

    return true;
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut rules: HashMap<usize, Vec<usize>> = HashMap::new();
    let mut updates: Vec<Vec<usize>> = Vec::new();

    for line in io::stdin().lock().lines() {
        match line {
            Ok(content) => {
                let s = content.split('|').collect::<Vec<_>>();
                match s.as_slice() {
                    [""] => (), // empty line
                    [_] => { // update
                        let s = content.split(',').map(str::parse::<usize>).collect::<Result<Vec<_>, _>>()?;
                        updates.push(s);
                    },
                    [before, after] => { // rule
                        rules.entry(before.parse::<usize>()?).or_insert(Vec::new()).push(after.parse::<usize>()?);
                    },
                    _ => panic!(),
                }
            },
            Err(error) => eprintln!("Error reading input: {}", error),
        }
    }

    let mut res1 = 0;
    let mut res2 = 0;

    for up in updates {
        if is_correct(&rules, &up) {
            res1 += up[up.len() / 2];
        } else {
            let mut sorted = up;
            sorted.sort_by(|a, b| {
                if rules.get(a).map(|r| r.contains(b)) == Some(true) {
                    Ordering::Less
                } else if rules.get(b).map(|r| r.contains(a)) == Some(true) {
                    Ordering::Greater
                } else {
                    Ordering::Equal
                }
            });

            res2 += sorted[sorted.len() / 2];
        }
    }

    println!("{}", res1);
    println!("{}", res2);

    Ok(())
}
