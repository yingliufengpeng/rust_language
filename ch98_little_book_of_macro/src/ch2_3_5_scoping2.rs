

// finally, note that these scoping behaviours apply to functions as well, with exception of #[macro_use]
// which isn't applicable

macro_rules! X {
    () => { Y!() };

}

fn a() {
    macro_rules! Y {
        () => {"Hi";};
    }

    assert_eq!(X!(), "Hi");
    {
        assert_eq!(X!(), "Hi");
        macro_rules! Y { () => {"Bye!";}}
        assert_eq!(X!(), "Bye!");
    }
    assert_eq!(X!(), "Hi");
}


#[cfg(test)]
mod tests {
    use super::a;
    #[test]
    fn test_001() {
        a();

        macro_rules! Y { () => {"Bye2";}}
        assert_eq!(X!(), "Bye2");
    }
}