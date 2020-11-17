//!
//! You can also set another attribute based on a cfg variable with
//! cfg_attr
//!
//!

#[cfg_attr(a, b)]
mod foo2 {

}

#[cfg(feature="foo")]
mod foo {

}

#[cfg(test)]
mod tests {



    #[test]
    fn test_001() {

        if cfg!(target_os = "macos") || cfg!(target_os = "ios") {
            println!("Think Different!");
        } else {
            println!("Think Same!!!");
        }

    }
}
