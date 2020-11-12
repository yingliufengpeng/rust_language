#![feature(trace_macros)]
#![feature(log_syntax)]

// macro_rules! Y {
//     () => {};
// }

// The following code will work as expected;
// X!(); // X is defined, but Y is not
#[macro_use(X)]  // 只导入了X宏,没有导入Y的宏
extern crate ch98_macro_export_use;

// Finally, when importing macros from an external crate, you can control which
// macros you import. You can use this to limit namespace pollution, or to override
// specific macros,

macro_rules! Y {
    () => {};
}

X!(); // X is defined so is Y

// When exporting macros, it is often useful to refer to non-macro symbols in the defining crate.
// Because creates can be renamed, there is a special substitution variable available: $create. This
// will always expand to an absolute path prefix to the containing create (eg::macs)


mod captures;
mod captures2;
mod ch2_3_4_debugging;
mod ch2_3_5_scoping;
mod ch2_3_5_scoping1;
mod ch2_3_5_scoping2;
mod ch2_3_6_import_export;
mod ch3_macros_practical_introduction;


#[cfg(test)]
mod tests {
    macro_rules! three  {
        () => (3);
    }

    macro_rules! four {

        () =>  (1 + three!() );
    }

    macro_rules! gibbersih {
        ( 4 fn [ 'spang "Who"] ) => { println!("gibberish");  };
    }

    #[test]
    fn test_001() {

        let x = four!();
        println!("{}", x);

        let x = four![];
        println!("{}", x);

        let x = four!{};
        println!("{}", x);

        gibbersih!(4 fn [ 'spang "Who" ]);
    }


}