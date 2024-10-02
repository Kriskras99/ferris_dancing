pub trait JsonDeserialize<'a> {
    fn from_tape(json: &'a str) -> Self;
}