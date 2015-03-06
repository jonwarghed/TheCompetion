use std::old_io::stdin;



fn main() {
    println!("What is your name?");
    let input = stdin().read_line().ok().expect("Failed to read line");
    println!("Hello {} !", input);
}
