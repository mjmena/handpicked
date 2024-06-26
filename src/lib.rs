#[derive(Clone, Debug)]
pub struct TouchPoint {
    pub id: i32,
    pub x: i32,
    pub y: i32,
    pub color: String,
}

impl std::fmt::Display for TouchPoint {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{} ({},{})", self.id, self.x, self.y)
    }
}

#[derive(Clone, PartialEq)]
pub enum State {
    Preparing,
    Selecting,
    Revealing,
    Resetting,
}

impl std::fmt::Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            State::Preparing => write!(f, "Preparing"),
            State::Selecting => write!(f, "Selecting"),
            State::Revealing => write!(f, "Revealing"),
            State::Resetting => write!(f, "Resetting"),
        }
    }
}
