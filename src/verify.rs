
// Given some string, assert that it might be decoded!

pub struct Verify {}

impl Verify {
    pub fn new() -> Self {
        Verify {}
    }

    pub fn verify(&mut self, input: &str) -> bool {
        false
    }

    pub fn verify_fuzzy(&mut self, input: &str) -> bool {
        false
    }
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
