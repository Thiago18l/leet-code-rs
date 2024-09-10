

struct Roman {
    value: u32,
}

impl Roman {
    fn new(value: u32) -> Roman {
        Roman { value }
    }

    fn numerals(&self) -> Vec<(u32, &str)> {
        vec![
            (1000, "M"),
            (900, "CM"),
            (500, "D"),
            (400, "CD"),
            (100, "C"),
            (90, "XC"),
            (50, "L"),
            (40, "XL"),
            (10, "X"),
            (9, "IX"),
            (5, "V"),
            (4, "IV"),
            (1, "I"),
        ]
    }

    fn to_string(&self) -> String {
        let mut value = self.value;
        let mut result = String::new();
        let roman_numerals = self.numerals();

        for &(num, numeral) in roman_numerals.iter() {
            while value >= num {
                result.push_str(numeral);
                value -= num;
            }
        }

        result
    }
    fn to_int(&self, string: String) -> u32 {
      let mut result = 0;
      let mut value = string;
      let roman_numerals = self.numerals();
      for &(num, numeral) in roman_numerals.iter() {
          while value.starts_with(numeral) {
              result += num;
              value = value[numeral.len()..].to_string();
          }
      }
      result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_roman() {
        let roman = Roman::new(1990);
        assert_eq!(roman.to_string(), "MCMXC");
    }
    #[test]
    fn test_int() {
        let roman = Roman::new(1990);
        assert_eq!(roman.to_int("MCMXC".to_string()), 1990);
    }
}