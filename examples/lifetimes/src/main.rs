//! An example that shows how `lifetimes` work

/// Returns the string with the longest value. If the strings are equal, the string literal
/// "Neither" is returned.
/// # Here's an example:
/// ```
/// let s1 = "Java";
/// let s2 = "C++";
/// assert_eq!("Java", longest_str(s1, s2));
/// ```
pub fn longest_str<'long>(a: &'long str, b: &'long str) -> &'long str {
    match a {
        _ if a.len() > b.len() => a,
        _ if b.len() > a.len() => b,
        _ => "Neither",
    }
}

fn main() {
    let s1 = "Java";
    let s2 = "C++";

    println!("{}", longest_str(s1, s2));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_longest_str() {
        let s1 = "Java";
        let s2 = "C++";

        assert_eq!("Java", longest_str(s1, s2));
    }

    #[test]
    fn test_neither() {
        let s1 = "Java";
        let s2 = "Rust";

        assert_eq!("Neither", longest_str(s1, s2));
    }
}
