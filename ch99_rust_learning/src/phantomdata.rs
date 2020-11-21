

#[cfg(test)]
mod tests {
    use std::marker::PhantomData;

    struct A<'a, T: 'a> {
        v: &'a T
    }

    struct B<'a, T: 'a> {
        ptr: *const T,
        _maker: PhantomData<&'a T>
    }

    #[test]
    fn test_001() {


    }


}