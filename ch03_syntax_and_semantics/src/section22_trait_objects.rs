#[cfg(test)]
mod tests {
    use crate::Foo;

    fn do_something<T: Foo>(x: T) {
        let r = x.method();
        println!("{}", r);
    }

    fn do_something2(x: &dyn Foo) {
        let r = x.method();
        println!("{}", r);
    }

    fn show(t: *const ()) {
        println!("需要做三次强制类型转换才可以!!!");
        let pr = unsafe {&*(t as *const i32)};
        println!("{}", pr);
    }

    struct MyTraitObject {
        data: *mut (),
        vtable: *mut (),
    }

    struct FooVtable {
        // destructor: fn(*mut ()),
        size: usize,
        align: usize,
        method: fn(*const ()) -> String,
    }

    // u8
    fn call_method_on_u8(x: *const()) -> String {
        // The compiler guarantees that this functions is only
        // called with `x` pointing to a u8.

        let byte: &u8  = unsafe { &*( x as *const u8) };

        byte.method()
    }

    static FOO_FOR_U8_VTABLE: FooVtable = FooVtable {
        // destructor: unimplemented!(),
        size: 1,
        align: 1,
        // Cast to a function pointer
        method: call_method_on_u8,
    };



    // String
    fn call_method_on_string(x: *const ()) -> String {
        // The compiler guarantees that this function is only called
        // with `x` pointing to a String.
        let string = unsafe {&* {x as *const String}};
        string.method()
    }

    static FOO_FOR_STRING_VTABLE: FooVtable = FooVtable {
        size: 24,
        align: 8,
        method: call_method_on_string,
    };

    #[test]
    fn test() {

        let x = 5u8;
        let y = "Hello".to_string();

        do_something(x);
        do_something(y);

        let x = 8u8;
        let y = "Hello".to_string();
        do_something2(&x);
        do_something2(&x as & dyn Foo);

        // coercing  根据当前的类型编译器自动转换为该Foo Trait的类型
        do_something2(&y);

        let r = 33;
        let pr = &r as *const i32;
        let prr = pr as *const ();
        show(prr);

        let a = "foo".to_string();
        let x = 1u8;

        // let b: &Foo = &a;
        let b = MyTraitObject {
            // data: unsafe { &a as *const String } as *mut (),
            data: &a as *const String as *mut (),
            vtable: &FOO_FOR_STRING_VTABLE as * const FooVtable as *mut (),
        };


        let v =  unsafe {
            let p = b.vtable as *mut FooVtable;
            &*p
        };
        println!("{}", v.align);
        println!("{}", v.size);
        // println!("{}", v.method);

        let m = v.method;
        m(b.data);

        (v.method)(b.data);

        // Object Safety
        // Not every trait can be used to make a trait object. For example, vectors implement Clone, but if we
        // try to make a trait object.



        ((unsafe {
           &*(b.vtable as *mut FooVtable)
        }).method)(b.data);



        let y = MyTraitObject {
            data: &x as *const u8 as *mut (),
            vtable: &FOO_FOR_U8_VTABLE as *const FooVtable as *mut (),
        };

        let v = unsafe {

            let p = y.vtable as *mut FooVtable;
            &*p
        };

        (v.method)(y.data);

        // Only traits that are object-safe can be made into trait objects. A trait is object-safe
        // if both of these are ture:
        //    the trait doesn't require that Self: Sized
        //    all of its methods are object-safe

        // So what makes a method object-safe? Each method must require that Self: Sized or all
        // of the following:
        //      must not have any type parameters
        //      must not use Self
        // Whew! As we can see, almost all of these rules talk about Self. A good intuition is
        // "except in special circumstances, if your's method use Self, it is not object-safe."
















    }
}
