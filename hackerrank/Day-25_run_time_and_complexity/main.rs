/* 
This program is a solution to the problem 
"Day 25: Running Time and Complexity" on HackerRank.

NOTE: Remember to send EOF at the end of the input. 
In Linux, press Ctrl+D while in the terminal,
after providing the inputs.
*/

use std::io::{self, BufRead};

fn read_lines_from_stdin() -> io::Result<Vec<String>> {
    let stdin = io::stdin();
    let handle = stdin.lock();
    let lines: Vec<String> = handle.lines().collect::<Result<_, _>>()?;
    Ok(lines)
}

fn is_prime(n: i32) -> bool {
    if n == 1 {
        return false;
    }
    if n == 2 {
        return true;
    }
    if n % 2 == 0 {
        return false;
    }
    let mut i = 3;
    while i*i <= n {
        if n % i == 0 {
            return false;
        }
        i += 2;
    }
    return true;
}

fn main() -> io::Result<()> {
    let lines = read_lines_from_stdin()?;
    let n: i32 = lines[0].trim().parse().unwrap();
    for i in 1..=n as usize {
        let x: i32 = lines[i].trim().parse().unwrap();
        if is_prime(x) {
            println!("Prime");
        } else {
            println!("Not prime");
        }
    }
    
    Ok(())
}