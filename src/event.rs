pub enum Event {
    Key(char),
    ReadLine(String),
    Enter,
    Backspace,
    Delete,
    Tab,
    Forward,
    Back,
    Next,
    Prev,
    Up,
    Down,
    Left,
    Right,
    Quit,

    Fn1,
    Fn2,
    Fn3,
}
