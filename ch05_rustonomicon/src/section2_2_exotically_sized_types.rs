
//!
//!  Dynamically Sized Types (DSTs)
//!
//!  There are two major DSTs exposed by the language
//!     trait objects: dyn MyTrait
//!     slices: [T], str, and others
//!
//!  Zero Sized Types(ZSTs)
//!     One of the most extreme examples of this is Sets and Maps. Given a
//!         Map<key, Value>, it is common to implement a Set<Key> as just
//!         a thin wrapper around Map<Key, UselessJunk>. In Many language,
//!         this would necessitate allocating spaces for UselessJunk and doing
//!         to store and load. UselessJunk only to discard it.  Proving this unnecessary
//!         would be a difficult analysis for the compiler.
//!
//!   Empty Types
//!     *const () (or equivalent) works reasonably well for void*, and can be made into
//!         a reference without any safety problems. It still doesn't prevent you from
//!         trying to read or write values, but at least it compiles to a no-op instead of UB.
//!
//!  Extern Types
//!

enum Void {
    // No variant = Emtpy
}

struct Nothing; // No fields = no size

// All fields have no size = no size
struct LotsOfNothing {
    foo: Nothing,
    qux: (), // empty tuple has no size
    bar: [u8; 0], // empty array has no size
}

#[derive(Debug)]
struct MySuperSliceabel<T: ?Sized> {
    info: u32,
    data: T
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn it_works() {
        let r = MySuperSliceabel {
            info: 3,
            data:[3, 3, 4],
        };

        let rr: &MySuperSliceabel<[u8]> = &r;
        println!("{:?}", rr);

        let res: Result<u32, Void> = Ok(0);
        // Err doesn't exits anymore, so Ok is actually irrefutable.
        // let Ok(num) = res;
        // println!("{}", num);
    }
}