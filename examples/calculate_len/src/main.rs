//! Shows the basic differences between `String` and `str` types, and how mutalibility is enforced.

/// Returns the length of the given string
/// # Examples:
/// ```
/// let s1 = String::from("Hello");
/// assert_eq!(5, calculate_len(&s1));
///
/// ```
pub fn calculate_len(s: &String) -> isize {
    return s.len() as isize;
}

fn main() {
    let mut s1 = String::from("Hello");
    println!("'{}' has len {}", s1, calculate_len(&s1));

    s1 += ", gophers";
    println!("'{}' has len {}", s1, calculate_len(&s1));
    let s2 = ", and rustaceans!";

    let combine = |a, b| format!("{}{}", a, b);
    let s3 = combine(&s1, &s2);

    println!("'{}' has len {}", s3, calculate_len(&s3));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn s1_size() {
        let s1 = String::from("Hello");
        assert_eq!(5, calculate_len(&s1));
    }

    #[test]
    fn s1_size_after_mod() {
        let mut s1 = String::from("Hello");
        s1 += ", gophers";
        assert_eq!(14, calculate_len(&s1));
    }

    #[test]
    fn combined_size() {
        let mut s1 = String::from("Hello");
        s1 += ", gophers";
        let s2 = ", and rustaceans!";
        let combine = |a, b| format!("{}{}", a, b);
        assert_eq!(31, calculate_len(&combine(&s1, &s2)));
    }
}
