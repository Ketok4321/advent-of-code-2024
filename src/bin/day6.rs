use std::io::{self, BufRead};

const DIR_VEC: [(isize, isize); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)]; // up, right, down, left

fn search_and_mark(arr: &mut Vec<Vec<char>>, mut pos: (isize, isize), dir_i: usize) {
    let dir = DIR_VEC[dir_i];
    loop {
        arr[pos.0 as usize][pos.1 as usize] = 'X';
        let new_pos = (pos.0 + dir.0, pos.1 + dir.1);
        if new_pos.0 < 0 || new_pos.0 >= arr.len() as isize || new_pos.1 < 0 || new_pos.1 >= arr[0].len() as isize {
            return;
        }
        if arr[new_pos.0 as usize][new_pos.1 as usize] == '#' {
            break;
        }
        pos = new_pos;
    }
    
    search_and_mark(arr, pos, (dir_i + 1) % 4);
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut arr = io::stdin().lock().lines().map(|s| s.unwrap().chars().collect::<Vec<_>>()).collect::<Vec<Vec<_>>>();

    let pos = arr.iter()
        .enumerate()
        .find_map(|(x, row)| {
            row.iter()
                .enumerate()
                .find_map(|(y, &value)| {
                    if value == '^' {
                        Some((x as isize, y as isize))
                    } else {
                        None
                    }
                })
        }).unwrap();
    
    search_and_mark(&mut arr, pos, 0);

    let res1 = arr.iter().map(|r| r.iter().filter(|x| x == &&'X').count()).sum::<usize>();

    println!("{}", res1);
    //println!("{}", res2);

    Ok(())
}
