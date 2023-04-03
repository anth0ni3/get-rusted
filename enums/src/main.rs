//Noraml Enum
enum WebEvent {
    PageLoad,
    PageUnload,
    KeyPress(char),
    Paste(String),
    Click { x: i64, y: i64 },
}

//Option Enum
enum Option<T> {
    Some(T),
    None,
}

//Result Enum
enum Result<T, E> {
    Ok(T),
    Err(E),
}

fn main() {
    let quit = WebEvent::KeyPress('q');

    let something = Some(1);
}
