use rand;
use rand::Rng;
use std::env;
use std::fs;

fn main() {
    let contents = fs::read_to_string("george_mac.txt").expect("read File failed");
    // let line_count = contents.lines().count();
    //
    // println!("There are {} lines in the file", line_count);

    let args: Vec<String> = env::args().collect();

    let contents = contents
        .lines()
        .filter(|line| !line.is_empty())
        .filter(|line| !line.contains("CHAPTER"))
        .collect::<Vec<&str>>()
        .join("\n");

    let contents = contents
        .split(".")
        .flat_map(|sen| sen.split(";"))
        .collect::<Vec<&str>>();

    if args.len() > 1 {
        let search_term = &args[1];

        // let mut prev_line = " ";
        // for line in contents.lines() {
        //     if line.contains(search_term) {
        //         print!("{}\n{}\n\n", prev_line, line);
        //     }
        // }
        // prev_line = &line;
        let found_lines = contents
            .into_iter()
            .filter(|line| line.contains(search_term))
            .collect::<Vec<&str>>();

        let rand_number = rand::thread_rng().gen_range(0..found_lines.len());

        println!("{}.\n", found_lines[rand_number]);
    } else {
        // let mut rand_section = contents.lines().skip(rand_number).take(4).collect::<Vec<&str>>();
        //
        // rand_section[0] = rand_section[0].split(".").last().unwrap();
        //
        // rand_section[3] = rand_section[3].split(".").next().unwrap();
        //
        // println!("{}\n", rand_section.join("\n"));
        //
        //
        let contents = contents
            .into_iter()
            .filter(|line| !line.is_empty())
            .filter(|line| !line.contains("CHAPTER"))
            .collect::<Vec<&str>>();

        let rand_number = rand::thread_rng().gen_range(0..contents.len());

        println!("{}.\n", contents[rand_number]);
    }
}

fn read_file(path: &str) {
    fs::read_to_string(path).expect("read File failed");
}

fn generate_random_tweet(sentences: Vec<&str>) {
    //TODO
}

fn generate_tweet_from_word(sentences: Vec<&str>, word: &str){
    //TODO
}
