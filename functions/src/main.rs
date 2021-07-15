fn main() {
    last_char(String::from("HELLO"));
}

fn last_char(string: String) -> char {
    if string.is_empty() {
        return 's';
    }
    string.chars().next_back().unwrap()
}
