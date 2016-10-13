

use super::traits::CharCoder;

struct Noop;

impl CharCoder for Noop {
    fn new() -> Self {
        Noop {}
    }

    fn encode(&mut self, input: &str) -> String {
        String::from(input)
    }

    fn decode(&mut self, input: &str) -> String {
        String::from(input)
    }
}




#[cfg(test)]
mod tests {
    use super::Noop;
    use traits::CharCoder;

    #[test]
    fn noop_changes_nothing() {
        let mut n = Noop::new();
        assert_eq!(String::from("hello"), n.encode("hello") );
        assert_eq!(String::from("hello"), n.decode("hello") );
    }
}

