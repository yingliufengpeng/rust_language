

//!
//! # Casts
//!
//! Cast are a superset of coercions: every coercion can be
//!     explicitly invoked via a cast.
//!

#[cfg(test)]
mod tests {

    #[test]
    fn test_001() {




        let i = 10;
        let b = i as u32;
        println!("{}", b);

    }

}