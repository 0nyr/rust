// https://leetcode.com/problems/complement-of-base-10-integer/
// compile: rustc main.rs; run: ./main

struct Solution;
impl Solution {
    pub fn bitwise_complement(n: i32) -> i32 {
        // get the binary representation of n
        let mut remainders = Vec::new();
        let mut quotient = n;

        if quotient == 0 {
            return 1;
        }

        while quotient > 0 {
            remainders.push(quotient%2);
            quotient = quotient/2;
        }

        remainders.reverse();

        // get the decimal representation of the complement
        // and replace 0 with 1 and 1 with 0
        let mut res: i32 = 0;

        for i in 0..remainders.len() {
            if remainders[i] == 0 {
                res += 2_i32.pow(
                    remainders.len() as u32 - i as u32 - 1
                );
            }
        }
        return res;
    }
}

fn print_test(input: i32, result: i32, expected: i32) {
    if result == expected {
        println!("success: input: {}, result: {}, expected: {}", input, result, expected);
    } else {
        println!("failure: input: {}, result: {}, expected: {}", input, result, expected);
    }
}

fn main() {    
    // test cases
    print_test(5, Solution::bitwise_complement(5), 2);
    print_test(7, Solution::bitwise_complement(7), 0);
    print_test(10, Solution::bitwise_complement(10), 5);
    print_test(0, Solution::bitwise_complement(0), 1);
    print_test(1, Solution::bitwise_complement(1), 0);
}
