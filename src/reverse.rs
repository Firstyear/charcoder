
use super::traits::CharCoder;

struct ReverseWords;

impl CharCoder for ReverseWords {
    fn new() -> Self {
        ReverseWords {}
    }

    fn encode(&mut self, input: &str) -> String {
        let mut result = String::new();
        for tok in input.split(" ") {
            let revtok: String = tok.chars().rev().map(|c| c).collect();
            result.push_str(revtok.as_str());
            // This will likely mis-format some things because we assume the spacing
            result.push_str(" ");
        }
        result
    }

    // This is a reversable function, so we can just call encode.
    fn decode(&mut self, input: &str) -> String {
        self.encode(input)
    }
}

// struct ReverseLines;
// takes hello there! to !ereht olleh

struct ReverseMessage;

// similar to the above, but for the whole message, so multi line works.
impl CharCoder for ReverseMessage {
    fn new() -> Self {
        ReverseMessage {}
    }

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

// struct ReverseMessageWordOrder;
// Reverse the word order for the whole message.

// struct ReverseLineWordOrder;
// Reverse the word order, so hello there! -> there! hello, but per line.

#[cfg(test)]
mod tests {
    use super::ReverseWords;
    use super::ReverseMessage;
    use traits::CharCoder;

    #[test]
    fn reverse_words_test() {
        let mut rw = ReverseWords::new();
        // The encode / decode may add a trailing space.
        assert_eq!(String::from("olleh !ereht "), rw.encode("hello there!"));
        assert_eq!(String::from("hello there! "), rw.decode("olleh !ereht"));
    }

    #[test]
    fn reverse_message_test() {
        let mut rm = ReverseMessage::new();
        assert_eq!(String::from("!ereht olleh"), rm.encode("hello there!"));
        assert_eq!(String::from("hello there!"), rm.decode("!ereht olleh"));
    }
}
