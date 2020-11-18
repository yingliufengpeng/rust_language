// #![doc(html_favicon_url = "https://example.com/favicon.ico")]
#![doc(html_playground_url = "https://playground.example.com/")]
// #![doc(issue_tracker_base_url = "https://github.com/rust-lang/rust/issues/")]
// #![doc(html_root_url = "https://docs.rs/serde/1.0")]
#![doc(html_no_source)]
#![doc(test(attr(deny(warnings))))]

//! What is rustdoc?
//! The standard Rust distribution ships with a tool called rustdoc. Its job is
//! to generate documentation for Rust projects. On a fundamental level, Rustdoc
//! takes as an argument either a crate root or a Markdown file, and produces HTML,
//! CSS, and JavaScript
//!
//!
//! When writing an example, it is rarely useful to include a complete error handling, as
//! it would significant amount of boilerplate code, you may want the following.
//!
//!
//! For conditional compilation, Rustdoc treats your crate the same way
//! the compiler does. Only things from the host target are available (or
//! from the given --target if present), and everything else is "filtered out"
//! from the crate. This can cause problems if your crate is providing different
//! things on different targets and you want your documentation to reflect
//! all the available items you provide.
//!
//! If you want to make sure an item is seen by Rustdoc regardless of what
//! platform it's targeting, you can apply #[cfg(doc)] to it. Rustdoc sets
//! this whenever it's building documentation, so anything that uses that
//! flag will make it into documentation it generates. To apply this to an
//! item with other #[cfg] filters on it, you can write something like
//! #[cfg(any(windows, doc))]. This will preserve the item either when
//! built normally on Windows, or when being documented anywhere.
//!
//! Please note that this feature is not passed to doctest.


/// foo is a function
/// # Example
/// ```
///use ch97_rust_doc::foo;
///foo();
/// ```
/// # Next Example
///
/// ```
/// # fn foo() {}
/// foo();
/// println!("Hello, World!");
/// ```
///
pub fn foo() {
    println!("foo");
}

/// bar is a function
///
/// ```python
/// a = 10
/// print(f'a is {a}')
/// ```
///
/// ```
/// let r = 10;
/// assert_eq!(r, 10);
/// ```
pub fn bar() {

}

/// zar is a function
/// # pub fn zar() {}
pub fn zar() {

}


/// First, we set `x` to five:
///
/// ```
/// let x = 5;
/// # let y = 6;
/// # println!("{}", x + y);
/// ```
///
/// Next, we set `y` to six:
///
/// ```
/// # let x = 5;
/// let y = 6;
/// # println!("{}", x + y);
/// ```
///
/// Finally, we print the sum of `x` and `y`:
///
/// ```
/// # let x = 5;
/// # let y = 6;
/// println!("{}", x + y);
/// ```
////
/// ```
/// use std::io;
/// # fn main() -> io::Result<()> {
/// let mut input = String::new();
/// io::stdin().read_line(&mut input)?;
/// # Ok(())
/// # }
/// ```
////
////
pub fn xxx() {

}


///
/// let s = "foo
/// ##bar # baz";
///
pub fn yyy() {}

#[doc = "This is a show comment"]
pub fn show() {}

#[doc(no_line)]
pub use bar::Bar;

/// bar docs
mod bar {
    /// the docs for Bar
    pub struct Bar {

    }
}

/// We have a struct here. Remember it doesn't accept negative numbers!
pub struct MyStruct(pub usize);

///
/// ``` compile_fail
/// let _r = 210;
/// let _x = ch97_rust_doc::MyStruct(-4);
///  ```
#[cfg(doctest)]
pub struct MyStructOnlyTakesSize;

/// Token struct that can only be used on Windows
#[cfg(any(windows, doc))]
#[derive(Debug)]
pub struct WindowsToken;


/// Token struct that can only be used on Unix
#[cfg(any(windows, doc))]
#[derive(Debug)]
pub struct UnixToken;


#[cfg(test)]
mod tests {
    use super::*;
    use std::any::Any;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }


    #[test]
    fn test_001() {
        println!("test_001");

        let r = WindowsToken;
        println!("{:?}", r);


        let r = UnixToken;
        println!("{:?}", r);

    }
}
