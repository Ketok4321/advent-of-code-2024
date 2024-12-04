use std::io::{self, BufRead};

fn xmas1_check(arr: &Vec<Vec<char>>, mut x: isize, mut y: isize, dx: isize, dy: isize) -> bool {
    for c in "XMAS".chars() {
        if x < 0 || y < 0 || x >= arr.len() as isize || y >= arr[0].len() as isize {
            return false;
        }
        if arr[x as usize][y as usize] != c {
            return false;
        }

        x += dx;
        y += dy;
    }

    return true
}

fn xmas2_check(arr: &Vec<Vec<char>>, x: usize, y: usize) -> bool {
    if x as isize - 1 < 0 || y as isize - 1 < 0 || x + 1 >= arr.len() || y + 1 >= arr[0].len() {
        return false;
    }

    let tl = arr[x - 1][y + 1];
    let tr = arr[x + 1][y + 1];
    let bl = arr[x - 1][y - 1];
    let br = arr[x + 1][y - 1];

    return ((tl == 'M' && br == 'S') || (tl == 'S' && br == 'M')) && ((tr == 'M' && bl == 'S') || (tr == 'S' && bl == 'M'));
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let arr = io::stdin().lock().lines().map(|s| s.unwrap().chars().collect::<Vec<_>>()).collect::<Vec<Vec<_>>>();

    let size_x = arr.len();
    let size_y = arr[0].len();

    let mut res1 = 0;
    let mut res2 = 0;

    for x in 0..size_x {
        for y in 0..size_y {
            match arr[x][y] {
                'X' => {
                    for a in -1..1+1 {
                        for b in -1..1+1 {
                            if a == 0 && b == 0 { continue; }

                            res1 += xmas1_check(&arr, x as isize, y as isize, a, b) as usize;
                        }
                    }
                },
                'A' => {
                    res2 += xmas2_check(&arr, x, y) as usize;
                },
                _ => ()
            }
        }
    }

    println!("{}", res1);
    println!("{}", res2);

    Ok(())
}
