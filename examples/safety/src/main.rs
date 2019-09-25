//! A look at `derive` attributes, and how `Option` and `Result can be used`

/// Shows how optionals work when used as structure fields
#[derive(Debug, Default)]
pub struct Point {
    pub name: Option<String>,
    pub x: isize,
    pub y: isize,
}

/// Shows what an error enum looks like, particularly a variant that has a static string slice as
/// its argument
#[derive(Debug)]
pub enum PointError<'pe> {
    NoName(&'pe str),
}

impl Point {
    /// Safely retrieves the name of the `Point` structure
    /// # Example:
    /// ```
    /// let constructed_point = Point {
    ///     name: Some(String::from("MyPoint")),
    ///     x: 15,
    ///     y: 45,
    /// };
    /// assert_eq!(
    ///    constructed_point.get_name().unwrap(),
    ///    String::from("MyPoint")
    /// );
    ///
    /// ```
    pub fn get_name(&self) -> Result<String, PointError> {
        if self.name.is_none() {
            return Err(PointError::NoName("No name set"));
        }
        let name = self.name.as_ref();
        Ok(name.unwrap().to_string())
    }
}

fn main() {
    let default_point: Point = Default::default();
    println!("{:#?}", default_point);

    let constructed_point = Point {
        name: Some(String::from("MyPoint")),
        x: 15,
        y: 45,
    };
    println!("{}", constructed_point.get_name().unwrap());
    println!("{:#?}", constructed_point);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default() {
        let default_point: Point = Default::default();
        assert_eq!(default_point.name, None);
        assert_eq!(default_point.x, 0);
        assert_eq!(default_point.y, 0);
    }

    #[test]
    fn test_values() {
        let constructed_point = Point {
            name: Some(String::from("MyPoint")),
            x: 15,
            y: 45,
        };
        assert_eq!(
            constructed_point.get_name().unwrap(),
            String::from("MyPoint")
        );
        assert_eq!(constructed_point.x, 15);
        assert_eq!(constructed_point.y, 45);
    }

}
