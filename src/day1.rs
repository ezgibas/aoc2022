//extern crate aoc_driver;
use aoc_driver::*;

pub fn main() {
    let input: String = aoc_driver::get_input("53616c7465645f5f1eeea41fccb4ffd6248098264e7ebab2748ea0d8363b7a69cfe0ebefaf9f1e2fcc848f2572e17354ca489d650efd806bd058143b7fdf1bd3", 
    2022, 1).unwrap();
    solve(input);
}

pub fn solve(input: String) -> () {
    let v: Vec<&str> = input.split_terminator("\n").collect();
    let mut cursum = 0;
    let mut maxsum = 0;
    for numstr in v {
        if(numstr.eq("")) {
            if(cursum >= maxsum) {
                maxsum = cursum;
            }
            cursum = 0;
        }
        else {
            let val: i32 = numstr.parse().unwrap();
            cursum = cursum + val;
         }
    }
    println!("{}", maxsum)
}