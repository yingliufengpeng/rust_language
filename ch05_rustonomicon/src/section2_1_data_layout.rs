
//!
//!# Rust gives you the following ways to lay out composite data:
//!     # structs(named product types)
//!     # tuples(anoymous product types)
//!     # arrays(homogeneous product types)
//!     # enums(named sum types -- tagged unions)
//!     # unions(untagged unions)


/// A will be 32-bit aligned on a target that aligns these primitives to their
/// respective sizes.
struct A {
    a: u8,
    b: u32,
    c: u16,
}

/// The whole struct will therefore have a size that is a multiple of 32-bit.
///. It may become A1 or A11
struct A1 {
    a: u8,
    _pad1: [u8; 3], // to align `b`
    b: u32,
    c: u16,
    _pad2: [u8; 2], // to make overall size multiple of 4
}

struct A11 {
    b: u32,
    c: u16,
    a: u8,
    _pad: u8,
}



struct A2 {
    a: i32,
    b: u64,
}

struct B2 {
    a: i32,
    b: u64,
}


enum Foo {
    A(u32),
    B(u64),
    C(u8),
}

struct FooRepr {
    data: u64, // this is either a u64, u32 or u8 based on `tag`
    tag: u8,    // 0 = A, 1 = B, 2 = C
}

/// In principle enums could use fairly elaborate algorithms to store bits throughout
/// nested types with forbidden values. As such it is especially desirable that we
/// leave enum layout unspecified today.

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
