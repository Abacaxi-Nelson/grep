use std::env;
use std::fs;

fn main() {
    let arguments: Vec<String> = env::args().collect();
    println!("Exe: {}", arguments.first().unwrap());
    println!("Arguments: {:?}", arguments.split_first().unwrap().1);

    // dumb rule, when we deal with object, we only store reference.alloc
    // thus, we dont copy arguments, we still point to the original argument
    let query = &arguments[1];
    let filename = &arguments[2];
    println!("Search: {}", query);
    println!("File: {}", filename);

    // read file passed in args
    let contents = fs::read_to_string(filename)
        // says if ok -> return content to 'contents' var
        // or panic with this error message
        .expect(&format!("Hum, i cannot read this file: {}", filename));

    println!("Text content:\n{}", contents);

    // search feature
    let mut lines_results = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
            lines_results.push(line);
        }
    }
    println!("lines_results: {:?}", lines_results);
}
