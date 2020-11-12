



#[cfg(test)]
mod tests {

    macro_rules! what_is {
        (self) => {"the keyword `self`"};
        ($i:ident) => { concat!("the identifier `", stringify!($i), "`") }
    }

    macro_rules! call_with_ident {
        ($c:ident ( $i:ident ) ) => { $c!($i)  };
    }

    macro_rules! make_mutable {
        ($i:ident) => { let mut $i = $i; };
    }

    macro_rules! make_mutable2 {
        ($i:ident) => { let mut $i = self; };
    }

    macro_rules! double_method {
        ($self_:ident, $body:expr) => {
            fn double(mut $self_) -> Self {
                $body
            }
        };
    }

    #[derive(Debug)]
    struct Dummy(i32);

    impl Dummy {
        double_method!{
            self,
            {
                self.0 *= 3;
                self
            }

        }
    }

    #[test]
    fn test_001() {

        let d = Dummy(34);
        let d = d.double();
        println!("{:?}", d);

        let x = what_is!(kk);
        println!("{}", x);

        let x = what_is!(self);
        println!("{}", x);

        let x = call_with_ident!(what_is(self));
        println!("{}", x);

        let i = 10;
        let mut i = i;
        make_mutable!(i);
        i = 30;
        println!("{}", i);

    }

}