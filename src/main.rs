use rand;
use std::env;
use std::fs;

fn main() {
    let contents = fs::read_to_string("george_mac.txt").expect("read File failed");
    let line_count = contents.lines().count();

    println!("There are {} lines in the file", line_count);

    let args: Vec<String> = env::args().collect();
    let mut search_term = "";
    if args.len() > 1 {
        search_term = &args[1];

        let mut prev_line = " ";
        for line in contents.lines() {
            if line.contains(search_term) {
                print!("{}\n{}\n\n", prev_line, line);
            }
            prev_line = &line;
        }
    } else {
        let rand_number = rand;
        let rand_line = contents.lines().nth(rand_number).unwrap();

    }
}
