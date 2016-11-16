
pub trait CharCoder {
    // fn new() -> Self;
    fn encode(&mut self, input: &str) -> String;
    fn decode(&mut self, input: &str) -> String;
}

