// There are two ways to expose a macro to a wider scope. The first is the #[macro_use] attribute
// This can be applied to either modules or external crates.


#[macro_use]
mod macors {
    macro_rules! X{ () => { Y!(); }}
    macro_rules! Y{ () => {} }
}

X!();

mod macors2 {

    #[macro_export] macro_rules! X2 {() => { Y2!(); }}
    #[macro_export] macro_rules! Y2 {() => {}}
}

// X! and Y! are not defined here, but are exported,
// despite 'macors2' being private.

// X2!();


#[cfg(test)]
mod tests {

    #[test]
    fn test_001() {


    }
}