#[cfg(test)]
mod tests {

    // Rust understand a few of these types, but they have some restriction
    //. There are three:
    //  We can only manipulate an instance of an unsized type via a pointer.
    //      An &[T] works fine, but a [T] does not.
    //  Variables and arguments cannot have dynamically sized types
    //  Only the last filed in struct may have a dynamically sized types; the
    //      other fields must not. Enum variants must not have dynamically
    //      sized types ad data.


    struct Foo<T: ?Sized> {
        foo: T,
    }

    #[test]
    fn test() {
        let a = Foo { foo: 3};
        let b = Foo { foo: "44"};


    }
}
