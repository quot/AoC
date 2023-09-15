use std::fs;

fn main() {
    let contents =
        fs::read_to_string("resources/day2a-input.txt").expect("Failed to read input file.");

    let mut top_cals: [i32; 3] = [0; 3];
    let mut cur_cals = 0;

    for cur_line in contents.lines() {
        // println!("CURLINE: {cur_line}");
        if !cur_line.is_empty() {
            cur_cals += cur_line.parse::<i32>().unwrap();
        } else {
            if cur_cals > top_cals[2] {
                top_cals[0] = top_cals[1];
                top_cals[1] = top_cals[2];
                top_cals[2] = cur_cals;
            } else if cur_cals > top_cals[1] {
                top_cals[0] = top_cals[1];
                top_cals[1] = cur_cals;
            } else if cur_cals > top_cals[0] {
                top_cals[0] = cur_cals;
            }
            cur_cals = 0;
        }
    }

    let mut top_total = 0;
    for cur_top in top_cals {
        println!("{cur_top}");
        top_total += cur_top;
    }
    println!("{top_total} total top calories")
}
