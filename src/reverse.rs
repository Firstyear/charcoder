
use super::traits::CharCoder;

pub struct ReverseMessage;

// similar to the above, but for the whole message, so multi line works.
impl ReverseMessage {
    pub fn new() -> Self {
        ReverseMessage {}
    }
}

impl CharCoder for ReverseMessage {
    fn encode(&mut self, input: &str) -> String {
        let mut result = String::new();
        let revtok: String = input.chars().rev().map(|c| c).collect();
        result.push_str(revtok.as_str());
        result
    }

    // This is a reversable function, so we can just call encode.
    fn decode(&mut self, input: &str) -> String {
        self.encode(input)
    }
}

pub struct ReverseWordOrder;
// Reverse the word order for the whole message. This also messes with whitespace a bit.

impl ReverseWordOrder {
    fn new() -> Self {
        ReverseWordOrder {}
    }
}

impl CharCoder for ReverseWordOrder {
    fn encode(&mut self, input: &str) -> String {
        let mut result = String::new();
        for tok in input.split_whitespace().rev() {
            result.push_str(tok);
            result.push_str(" ");
        };
        result
    }

    fn decode(&mut self, input: &str) -> String {
        self.encode(input)
    }
}

#[cfg(test)]
mod tests {
    use super::ReverseMessage;
    use super::ReverseWordOrder;
    use traits::CharCoder;


    #[test]
    fn reverse_message_test() {
        let mut rm = ReverseMessage::new();
        assert_eq!(String::from("!ereht olleh"), rm.encode("hello there!"));
        assert_eq!(String::from("hello there!"), rm.decode("!ereht olleh"));
    }

    #[test]
    fn reverse_word_order_test() {
        let mut rmw = ReverseWordOrder::new();
        assert_eq!(String::from("there! hello").trim(), rmw.encode("hello there!").trim());
        assert_eq!(String::from("hello there!").trim(), rmw.decode("there! hello").trim());
    }
}
