fn main() {
    println!("Hello, world!");
    let text: &str = "HEllo\nWorld\n!";
    println!("{}", first_lines(text));
}


pub fn first_lines(string: &str) -> &str {
    string.lines().next().unwrap()
}
