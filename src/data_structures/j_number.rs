// Copyright © 2021 Hovig Manjikian
//
// This file is part of json.
//
// json is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// json is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with json.  If not, see <https://www.gnu.org/licenses/>.

use std::fmt::{Display, Formatter};
use std::str::FromStr;

/// A number is a sequence of decimal digits with no superfluous leading zero. It may have a
/// preceding minus sign (U+002D). It may have a fractional part prefixed by a decimal
/// point (U+002E).It may have an exponent, prefixed by e(U+0065) or E(U+0045) and
/// optionally +(U+002B) or –(U+002D). The digits are the code points U+0030 through U+0039.
#[derive(Debug, Clone)]
pub struct JNumber {
    sign: Sign,
    integer_part: String,
    fractional_part: String,
    exponent: String,
    e_sign: Sign,
    e_symbol: char,
    f64_value: f64,
}

impl JNumber {
    /// returns an f64 representation of the given number
    pub fn get_f64_value(&self) -> f64 {
        self.f64_value
    }
}

impl Display for JNumber {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}{}{}{}{}{}",
               if self.sign == Sign::Negative { "-" } else { "" },
               self.integer_part,
               if self.fractional_part != "0" { "." } else { "" },
               if self.fractional_part != "0" { self.fractional_part.clone() } else { "".to_string() },
               if self.exponent != "0" { self.e_symbol.to_string() } else { "".to_string() },
               if self.e_sign == Sign::None { "" } else if self.e_sign == Sign::Positive { "+" } else { "-" },
               if self.exponent != "0" { self.exponent.clone() } else { "".to_string() }
        )
    }
}

impl FromStr for JNumber {
    type Err = String;

    /// Will create a Number form a string
    ///
    /// ```
    /// use std::str::FromStr;
    /// use json::data_structures::JNumber;
    ///
    /// let n = JNumber::from_str("-0.001").unwrap();
    /// assert_eq!("-0.001".to_string(), n.to_string());
    /// assert_eq!(-0.001_f64, n.get_f64_value());
    ///
    /// let n = JNumber::from_str("2.5e7").unwrap();
    /// assert_eq!("2.5e7".to_string(), n.to_string());
    /// assert_eq!(25000000.0_f64, n.get_f64_value());
    ///
    /// // Illegal case
    /// let n = JNumber::from_str("2.5.000");
    /// assert_eq!(Err("An illegal point at index 3".to_string()), n);
    /// ```
    // Todo: High cyclomatic complexity! Optimization needed.
    // Todo: The f64_value is not very accurate.
    // Todo: consider implementing from_str() locally i.e. without implementing the trait std::str::FromStr
    fn from_str(s: &str) -> Result<Self, String> {
        let mut sign = Sign::None;
        let mut next_is_point = false;
        let mut next_is_digit = false;
        let mut integer_part: String = "0".to_string();
        let mut fractional_part: String = "0".to_string();
        let mut exponent_part: String = "0".to_string();
        let mut e_sign = Sign::None;
        let mut e_symbol = ' ';
        let mut temp_int: String = String::new();
        let mut point = false;
        for (i, c) in s.chars().enumerate() {
            if i == 0 {
                if c == '-' {
                    sign = Sign::Negative;
                } else if c == '0' {
                    next_is_point = true;
                } else if (c as u32) > 0x0030 && (c as u32) < 0x003A {
                    temp_int.push(c);
                } else {
                    return Err(format!("Illegal symbol {} at index {}", c, i));
                }
            } else if next_is_point {
                if c != '.' {
                    return Err(format!("Illegal input! Point was expected at index {}", i));
                } else {
                    next_is_point = false;
                    point = true;
                    next_is_digit = true;
                }
            } else if next_is_digit {
                if (c as u32) >= 0x0030 && (c as u32) < 0x003A {
                    temp_int.push(c);
                    next_is_digit = false;
                } else {
                    return Err(format!("Digit was expected at index {}", i));
                }
            } else if (c as u32) >= 0x0030 && (c as u32) < 0x003A {
                temp_int.push(c);
            } else if c == '.' {
                if point || e_symbol != ' ' {
                    return Err(format!("An illegal point at index {}", i));
                }
                integer_part = temp_int.clone();
                temp_int.clear();
                point = true;
            } else if c == 'e' || c == 'E' {
                if e_symbol == ' ' {
                    e_symbol = c;
                    if point {
                        fractional_part = temp_int.clone();
                    } else {
                        integer_part = temp_int.clone();
                    }
                    temp_int.clear();
                } else {
                    return Err(format!("Illegal symbol {} at index {}", c, i));
                }
            } else if c == '+' || c == '-' {
                if e_symbol != ' ' && temp_int.len() == 0 && e_sign == Sign::None {
                    e_sign = if c == '+' { Sign::Positive } else { Sign::Negative };
                } else {
                    return Err(format!("An illegal sign at index {}", i));
                }
            } else {
                return Err(format!("Illegal first symbol {} at index {}", c, i));
            }
        }
        if temp_int.len() > 0 {
            if e_symbol != ' ' {
                exponent_part = temp_int.clone();
            } else if point {
                fractional_part = temp_int.clone();
            } else {
                integer_part = temp_int.clone();
            }
        }
        let f64_value = get_f64(&sign,
                                &integer_part,
                                &fractional_part,
                                &e_sign,
                                &exponent_part);
        Ok(JNumber {
            sign,
            integer_part,
            fractional_part,
            exponent: exponent_part,
            e_sign,
            e_symbol,
            f64_value,
        })
    }
}

impl PartialEq for JNumber {
    fn eq(&self, other: &Self) -> bool {
        self.f64_value == other.f64_value
    }
}

fn get_f64(sign: &Sign,
           integer_part: &str,
           fractional_part: &str,
           e_sign: &Sign,
           exponent_part: &str) -> f64 {
    let mut result: f64 = integer_part.parse::<f64>().unwrap();
    result += fractional_part.parse::<f64>().unwrap() / 10.0_f64.powi(fractional_part.len() as i32);
    if exponent_part != "0" {
        if *e_sign == Sign::Negative {
            result *= 10.0_f64.powi(-1 * (exponent_part.parse::<i32>().unwrap()));
        } else {
            result *= 10.0_f64.powi(exponent_part.parse::<i32>().unwrap());
        }
    }
    if *sign == Sign::Negative {
        result *= -1.0;
    }
    result
}

/// An enum that represents the signs for numbers
#[derive(Debug, Clone, PartialEq)]
pub enum Sign {
    None,
    Positive,
    Negative,
}


#[cfg(test)]
mod test {
    use std::str::FromStr;
    use crate::data_structures::JNumber;

    #[test]
    fn test_zero() {
        let n = JNumber::from_str("0").unwrap();
        assert_eq!("0".to_string(), n.to_string());
        assert_eq!(0_f64, n.get_f64_value());

        let n = JNumber::from_str("-0").unwrap();
        assert_eq!("-0".to_string(), n.to_string());
        assert_eq!(-0_f64, n.get_f64_value());
    }

    #[test]
    fn test_legal_negative_int() {
        let n = JNumber::from_str("-300").unwrap();
        assert_eq!("-300".to_string(), n.to_string());
        assert_eq!(-300_f64, n.get_f64_value());
    }

    #[test]
    fn test_legal_positive_int() {
        let n = JNumber::from_str("5898499948554533445").unwrap();
        assert_eq!("5898499948554533445".to_string(), n.to_string());
        assert_eq!(5898499948554533445_f64, n.get_f64_value());
    }

    #[test]
    fn test_legal_negative_decimal() {
        let n = JNumber::from_str("-0.0016387").unwrap();
        assert_eq!("-0.0016387".to_string(), n.to_string());
        assert_eq!(-0.0016387_f64, n.get_f64_value());
    }

    #[test]
    fn test_legal_positive_decimal() {
        let n = JNumber::from_str("340.600").unwrap();
        assert_eq!("340.600".to_string(), n.to_string());
        assert_eq!(340.600_f64, n.get_f64_value());
    }

    #[test]
    fn test_legal_negative_exponent() {
        let n = JNumber::from_str("-0.0016387e7").unwrap();
        assert_eq!("-0.0016387e7".to_string(), n.to_string());
        assert_eq!(-0.0016387e7_f64, n.get_f64_value());

        let n = JNumber::from_str("-0.0016387E-3").unwrap();
        assert_eq!("-0.0016387E-3".to_string(), n.to_string());
        assert_eq!(-0.0016387e-3_f64, n.get_f64_value());

        let n = JNumber::from_str("-0.0016387E+3").unwrap();
        assert_eq!("-0.0016387E+3".to_string(), n.to_string());
        assert_eq!(-0.0016387e+3_f64, n.get_f64_value());
    }

    #[test]
    fn test_legal_positive_exponent() {
        let n = JNumber::from_str("340.6001e5").unwrap();
        assert_eq!("340.6001e5".to_string(), n.to_string());
        assert_eq!(340.6001e5_f64, n.get_f64_value());

        let n = JNumber::from_str("340.6001E-5").unwrap();
        assert_eq!("340.6001E-5".to_string(), n.to_string());
        assert_eq!(340.6001e-5_f64, n.get_f64_value());

        let n = JNumber::from_str("340.6001E+2").unwrap();
        assert_eq!("340.6001E+2".to_string(), n.to_string());
        assert_eq!(340.6001e+2_f64, n.get_f64_value());
    }

    #[test]
    fn test_illegal_numbers() {
        let n = JNumber::from_str("00");
        assert_eq!(Err("Illegal input! Point was expected at index 1".to_string()), n);

        let n = JNumber::from_str("+0");
        assert_eq!(Err("Illegal symbol + at index 0".to_string()), n);

        let n = JNumber::from_str(".30");
        assert_eq!(Err("Illegal symbol . at index 0".to_string()), n);

        let n = JNumber::from_str("1.3.0");
        assert_eq!(Err("An illegal point at index 3".to_string()), n);

        let n = JNumber::from_str("1-30");
        assert_eq!(Err("An illegal sign at index 1".to_string()), n);

        let n = JNumber::from_str("-13e2.3");
        assert_eq!(Err("An illegal point at index 5".to_string()), n);

        let n = JNumber::from_str("13e2e3");
        assert_eq!(Err("Illegal symbol e at index 4".to_string()), n);

        let n = JNumber::from_str("-1123.35E2E3");
        assert_eq!(Err("Illegal symbol E at index 10".to_string()), n);
    }
}
