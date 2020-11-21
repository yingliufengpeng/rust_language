
//!
//!  # Higher-Rank Trait Bounds
//!
//!
//!
//!


struct Closure<F> {
    data: (u8, u16),
    func: F,
}

impl <F> Closure<F>
    where F: Fn(&(u8, u16)) -> &u8,
{
    fn call(&self) -> &u8 {
        (self.func)(&self.data)
    }

}

fn do_it(data: &(u8, u16)) -> &u8  {
    &data.0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_001() {

        let clo = Closure { data: (0, 1), func: do_it};

        println!("{}", clo.call());
    }

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
