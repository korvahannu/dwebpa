use std::io::stdin;

pub fn ask_question(question: String) -> char {
    println!("{}", question);
    let mut c : String = String::new();
    stdin().read_line(&mut c).unwrap();
    c.as_str().chars().nth(0).unwrap()
}