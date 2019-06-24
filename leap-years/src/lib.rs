pub struct Year {
    value: u32,
}

impl Year {
    pub fn new(value: u32) -> Year {
        Year {
            value
        }
    }
    pub fn is_leap(&self) -> Result<bool, String> {
        if self.value < 1583 {
            return Err(String::from("Not gregorian year"));
        }
        if self.value % 400 == 0 {
            return Ok(true);
        }
        if self.value % 100 == 0 {
            return Ok(false);
        } else {
            return Ok(self.value % 4 == 0);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn is_not_leap_when_not_divisible_by_4() {
        assert!(!Year::new(2001).is_leap().unwrap());
    }
    #[test]
    fn is_leap_when_divisible_by_4() {
        assert!(Year::new(2004).is_leap().unwrap());
    }
    #[test]
    fn is_not_leap_when_divisible_by_100() {
        assert!(!Year::new(1900).is_leap().unwrap());
    }
    #[test]
    fn is_leap_when_divisible_by_400() {
        assert!(Year::new(2000).is_leap().unwrap());
    }
    #[test]
    #[should_panic(expected = "Not gregorian year")]
    fn returns_err_when_before_1583() {
        Year::new(1500).is_leap().unwrap();
    }
}
