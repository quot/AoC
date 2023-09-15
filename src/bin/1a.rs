use std::{fs, io::stdin};

fn main() {
    let contents =
        fs::read_to_string("resources/day1a-input.txt").expect("Failed to read input file.");

    let mut max_elf = 1;
    let mut max_cals = 0;
    let mut cur_elf = 1;
    let mut cur_cals = 0;

    for cur_line in contents.lines() {
        // println!("CURLINE: {cur_line}");
        if !cur_line.is_empty() {
            cur_cals += cur_line.parse::<i32>().unwrap();
        } else {
            // println!("elf {cur_elf}: {cur_cals} cals");
            // let mut s = String::new();
            // stdin().read_line(&mut s).unwrap();

            cur_cals = 0;
            cur_elf += 1;
        }

        if cur_cals > max_cals {
            max_cals = cur_cals;
            max_elf = cur_elf;
        }
    }
    println!("Elf {max_elf}: {max_cals} calories")
}
