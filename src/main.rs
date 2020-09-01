use std::env;

fn main() {
    let arguments: Vec<String> = env::args().collect();
    println!("Progran name: {:?}", arguments.first().unwrap());
    println!("Arguments: {:?}", arguments.split_first().unwrap().1);
}
