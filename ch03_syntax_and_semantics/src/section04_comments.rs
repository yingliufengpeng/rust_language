
//! # The Rust Standard Library
//!
//! The Rust Standard Library provides the essential runtime
//! functionality for building portable Rust software.

#[cfg(test)]
mod tests {

    /// Adds one to the number given.
    ///
    /// # Examples
    ///
    /// ```
    /// let five = 5;
    ///
    /// assert_eq!(60, add_one(5));
    /// # fn add_one(x: i32) -> i32 {
    /// #     x + 1
    /// # }
    /// ```
    fn add_one(x: i32) -> i32 {
        x + 1
    }

    #[test]
    fn test() {

        // Rust has two kinds of comments that you should care about: line comments and doc comments.

        // Line comments are anything after ‘//’ and extend to the end of the line.
        let x = 5; // This is also a line comment.

        // If you have a long explanation for something, you can put line comments next
        // to each other. Put a space between the // and your comment so that it’s
        // more readable.

        add_one(3);
    }
}