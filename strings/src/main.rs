fn main() {
    let text = "Hello\nWorld\n!";
    let upper = text.to_uppercase();
    let stripped = upper.strip_prefix("HELLO\n").unwrap();
    println!("{}", first_line(text));
}

// String - owned string
// &str - string slice

pub fn first_line(string: &str) -> &str {
    string.lines().next().unwrap()
}
