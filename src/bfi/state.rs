#[derive(PartialEq)]
pub enum State {
    Ready,
    Running,
    Ended,
    Error
}