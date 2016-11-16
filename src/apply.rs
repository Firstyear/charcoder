use super::traits::CharCoder;

// Implements some helpers for apply a CharCoder to words or lines
// rather than complete messages.

pub struct ApplyByLine<T> {
    apply: T,
}

impl<T: CharCoder> ApplyByLine<T> {
    pub fn new(apply: T) -> Self {
        ApplyByLine {
            apply: apply,
        }
    }
}

impl<T: CharCoder> CharCoder for ApplyByLine<T> {
    fn encode(&mut self, input: &str) -> String {
        let mut result = String::new();
        for l in input.lines() {
            result.push_str(self.apply.encode(l).as_str());
            result.push_str("\n");
        }
        result
    }

    fn decode(&mut self, input: &str) -> String {
        let mut result = String::new();
        for l in input.lines() {
            result.push_str(self.apply.decode(l).as_str());
            result.push_str("\n");
        }
        result
    }
}

pub struct ApplyByWord<T> {
    apply: T,
}

impl<T: CharCoder> ApplyByWord<T> {
    fn new(apply: T) -> Self {
        ApplyByWord {
            apply: apply,
        }
    }
}

impl<T: CharCoder> CharCoder for ApplyByWord<T> {
    fn encode(&mut self, input: &str) -> String {
        let mut result = String::new();
        for tok in input.split(" ") {
            result.push_str(self.apply.encode(tok).as_str());
            result.push_str(" ");
        }
        result
    }

    fn decode(&mut self, input: &str) -> String {
        let mut result = String::new();
        for tok in input.split(" ") {
            result.push_str(self.apply.decode(tok).as_str());
            result.push_str(" ");
        }
        result
    }
}


#[cfg(test)]
mod tests {
    use reverse::ReverseMessage;
    use reverse::ReverseWordOrder;
    use super::ApplyByWord;
    use super::ApplyByLine;
    use traits::CharCoder;

    #[test]
    fn reverse_applybyword_test() {
        let mut rw = ReverseMessage::new();
        let mut abw = ApplyByWord::new(rw);
        // The encode / decode may add a trailing space.
        assert_eq!(String::from("olleh !ereht "), abw.encode("hello there!"));
        assert_eq!(String::from("hello there! "), abw.decode("olleh !ereht"));
    }

    #[test]
    fn reverse_line_test() {
        let mut rw = ReverseMessage::new();
        let mut abl = ApplyByLine::new(rw);
        let forward = r#"
hello there!
this is a test!
        "#;
        let reverse = r#"
!ereht olleh
!tset a si siht
        "#;
        assert_eq!(String::from(reverse).trim(), abl.encode(forward).trim());
        assert_eq!(String::from(forward).trim(), abl.decode(reverse).trim());
    }
}
