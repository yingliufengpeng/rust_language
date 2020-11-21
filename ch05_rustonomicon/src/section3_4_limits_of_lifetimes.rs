//!
//! # Limits of Lifetimes
//!
//!

use std::collections::HashMap;
use std::hash::Hash;

fn get_default<'m, K, V>(map: &'m mut HashMap<K, V>, key: K) -> &'m mut V
    where
        K: Clone + Eq + Hash,
        V: Default,
{
    match map.get_mut(&key) {
        Some(value) => value,
        None => {
            map.insert(key.clone(), V::default());
            map.get_mut(&key).unwrap()
        }
    }
}


#[cfg(test)]
mod tests {
    #[derive(Debug)]
    struct Foo;

    impl Foo {
        fn mutate_and_share(&mut self) -> &Self { &*self }
        fn share(&self) {}
    }

    #[test]
    fn test_001() {
        let mut foo = Foo;
        let loan = foo.mutate_and_share();
        foo.share();
        println!("{:?}", loan);
    }


    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
