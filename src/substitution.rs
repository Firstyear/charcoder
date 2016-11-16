use super::traits::CharCoder;

fn apply_sub_lower(c: char, inv: &Vec<char>, outv: &Vec<char>) -> char {
    let mut index = inv.iter().position(|&x| x == c);
    match index {
        Some(i) => {
            match outv.get(i) {
                Some(d) => d.clone(),
                None => c,
            }
        },
        None => c,
    }
}

fn apply_sub(c: char, inv: &Vec<char>, outv: &Vec<char>) -> char {
    if c.is_uppercase() {
        let res_c: Vec<char> = c.to_lowercase().map(|lc| apply_sub_lower(lc, inv, outv)).collect();
        let utc = res_c.first().unwrap().to_uppercase().next().unwrap();
        utc
    } else {
        apply_sub_lower(c, inv, outv)
    }
}

pub struct Sub<'a> {
    alphabet: &'a Vec<char>,
    key: &'a Vec<char>,
}

impl<'a> Sub<'a> {
    pub fn new(alpha: &'a Vec<char>, key: &'a Vec<char>) -> Self {
        Sub {
            alphabet: alpha,
            key: key,
        }
    }
}

impl<'a> CharCoder for Sub<'a> {
    fn encode(&mut self, input: &str) -> String {
        let mut result = String::new();
        for c in input.chars() {
            result.push(apply_sub(c, self.alphabet, self.key));
        }
        result
    }

    fn decode(&mut self, input: &str) -> String {
        let mut result = String::new();
        for c in input.chars() {
            result.push(apply_sub(c, self.key, self.alphabet));
        }
        result
    }
}


#[cfg(test)]
mod tests {
    use super::Sub;
    use traits::CharCoder;

    #[test]
    fn test_substitution_rot13() {
        let alpha = vec!['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z'];
        let key =   vec!['n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm'];
        let mut s = Sub::new(
            &alpha,
            &key
        );
        assert_eq!(String::from("uryyb"), s.encode("hello") );
        assert_eq!(String::from("hello"), s.decode("uryyb") );

        assert_eq!(String::from("URYYB"), s.encode("HELLO") );
        assert_eq!(String::from("HELLO"), s.decode("URYYB") );
    }
}
