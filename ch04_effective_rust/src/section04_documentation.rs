use std::rc::Rc;

/// Construct a new 'Rc<T>'
///
/// Other details about construting `Rc<T>`s, maybe descrbing complicated semantics,
/// maybe additional options, all kinds of stuff.
///  # Examples
/// ```
///     use std::rc::Rc;
///     let five = Rc::new(43);
/// ```
/// # Panics
///
/// unrecoverable misuses of a functioin(i.e. programming errors) in Rust are usaully indicated
///         by panics, which will the whole current thread at very least.
/// # Errors
///
/// if your function or method retrurns a Result<T, E>, then describing the conditions under which
///         it returns Err(E) is a nice thing to do. This is slightly less important than Panics,
///         because failure is encoded into the type system, but it's still a good thing to do.
///
/// # Saftety
/// if your function is unsafe, you should explain which invariants hte caller is respoinsible for
///             upholding
///
/// # Examples
///
/// Examples. Include one or more examples of using your function or method, and your users will
/// lover you for it. These examples go inside of code block anotations, which we'll talk about
/// in moment, and can have kmore than one section.
///
/// ```
///     println!("Hello, world for section04_documentation");
/// ```
///
/// Here's the full algorithm rustdoc uses to preprocess examples:
///
/// Any leading #[!foo] attributes are left intact as create atributes
///
/// Some common allow attributes are inserted, including unused_variables, unused_assignemnts,
///         unused_mut, unused_attributes, and dead_code.
///
/// If the example does not contains extern crate, then extern crate <mycrate>; is inserted
///         (note the lack of $[macro_use])
///
///
/// Finally, if the example does not contain fn main, the remainder of the text is wrappered
///         in fn main() { your_code}
///
///
///
pub fn new<T>(value: T) -> Rc<T> {
    unimplemented!("kk");
}


/// Some documentation
///
/// ```rust
/// let x = 5;
/// let y = 6;
/// println!("{}", (x + y));
/// ```
///
///
///
/// # pub fn show() {}

pub fn show() {

}

///
/// Panic with a given message unless an expression evaluates to true.
///
/// # Examples
///
/// ```
/// # #[macro_use] extern crate  ch04_effective_rust;
/// # fn main() {
/// #    panic_unless!(1 + 1 == 2, "Match is borken");
/// # }
/// ```
/// ```rust,should_panic
/// assert!(false)
/// ```
///
/// ```rust,no_run
/// loop {
///     println!("Hello, World");
/// }
/// ```
///
/// A doc test using try!
///
/// ```
/// use std::io;
/// fn foo() -> io::Result<()> {
/// println!("documentation");
/// let mut input = String::new();
///     Ok(())
/// # }
///
/// foo();
/// println!("documentation2");
/// ```
///
#[macro_export]
macro_rules!  panic_unless {
    ($condition:expr, $($rest:expr),+) => ({ if ! $condition { panic!($($rest),+); } });
}

/// The `MyOption` type. See [the module level documentation] (index.html) for more.
enum MyOption<T> {
    /// No value
    None,

    /// Some value `T`
    Some(T)
}


#[cfg(test)]
mod tests {

    #[test]
    fn test_001() {

    }
}