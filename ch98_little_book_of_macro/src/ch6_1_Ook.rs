// This macro is an implementation of the Ook! esoteric language.

// The execution model for the language is very simple: memory is represented as an array of "cells"
// (typically at least-8 bits) of some indeterminate number(usually at least 30,000). There is pointer
// into memory which starts off at position 0. Finally, there is an execution stack (used to implement looping)
// and pointer into the program, although these last two are not exposed to the running program; they are
//  properties the runtime itself.

// The language itself is comprised of just three tokens: Ook. , Ook? , and Ook!. There are combined
// in pairs to form the eight different operations:
//

// Ook. Ook?    increment pointer
// Ook? Ook.    decrement pointer
// Ook. Ook.    increment pointed-to memory cell
// Ook! Ook!    decrement pointed-to memory cell
// Ook! Ook.    write pointed-to memory cell to standard out
// Ook. Ook!    read from standard input into pointed-to memory
// Ook! Ook?    begin loop
// Ook? Ook!    jump back to start of loop if pointed-to memory cell is not zero; otherwise, continue

// This is, in fact, the lowest possible recursion limit for which the example program provided at the
// end will actually compile. If you're wondering what could be so fantastically complex that it would
// justify a recursion limit nearly five times the default limit..  take a wild guess.



type CellType = u8;

const MEM_SIZE: usize = 100;

// The first of these will be a @start rule, which takes care of setting up the block in which the
// rest of our expansion will happen. There is nothing particularly interseting in this: we define
// some variables and helper functions, then do the bulk of the expansion.

// A few small notes:
//      We are expanding into a function largely so that we can use try! to simplify error handling.
//      The use of underscore-prefixed names is so that the compiler will not complain about unused
//          functions ro variables if, for example, the user writes an Ook! program that does no I/O

// The general form of these rule is (@e $sysms; (input)). As you can see form the @start rule, $syms is
// the collection of symbols needed to actually implement the program: input, output, the memory array, etc.
// We are using TT bundling to simplify forwarding of these symbols through later, intermediate rules.

// First, is the rule that terminates our recursion: once we have no more input, we stop.
// Next, we have a single rule for almost each opcode. For these, we strip off the opcode, emit the
// corresponding Rust code, then recurse on the input tail: a textbook TT muncher.

// Of course, we cannot actually emit an incomplete loop. This could be solved by using pushdown,
// were if not for more fundamental problems: we cannot write while memory[ptr] != {, at all, anywhere.
// This is because doing so would introduce an unbalanced brace.

// The solution to this is to actually split the input two parts: everything inside the loop, and everything
// after if. The @x rules handle the first, @s the latter.

// Next are the @x, or "extraction", rules. There are responsible for taking an input tail and extracting the
// contents of loop. The general form of these rules is: (@x $sym; $depth; $buf; $tail)
// The purpose of $sym is the same as above. $tail is the input to be parsed, whilst $buf is a push-down
// accumulation info which we will collect the opcodes that are inside the loop. But what of $depth?
// A complication to all this is that loops can be nested. Thus, we must have some way of keeping track of how
// many levels deep we currently are. We must track this accurately enough to not stop parsing too early,
// nor too late, but when the level is just right.
// Since we cannot do arithmetic in macros, and ti would be infeasible to write out explicit integer-matching rules
// (imagine the following rules all copy all copy&pasted for a non-trivial number of positive integers), we will
// instead fall back on one of the most ancient and venerable counting methods in history: counting on our fingers.
// But as macros don't have fingers, we'll use a token abacus counters instead. Specifically, we will use @s, where
// each @ represents one additional level of depth. If we keep these @s contained in a group, we can implemnt the
// three important operations we need:
//      Increment: match ($($depth:tt)*), substitute (@ $($depth)*)
//      Decrement: match (@ $($depth:tt)*), substitute ($($depth)*))
//      Compare to zero: match ()
// First is a rule to detect when we find the matching Ook? Ook! sequence that closes the loop we're parsing.
// In this case, we feed the accumulated loop contents to the previously defined @e rules.
// Note that we don't do anything with the remaining input until(that will be handled by the @s rules)
// Next, we have rules for entering and exiting nested loops. These adjust the counterand add the opcodes
// to the buffer.
// Finally, we have a rule for "everything else". Note the $op0, and $op1 captures: as far as Rust is
// concerned, our Ook! tokens are always two Rust tokens: the identifier Ook, and another token. Thus,
// we can generalise over all non-loop opcodes by matching !, ?, and . as tts.
// Here, we leave $depth untouched and just add the opcodes to the buffer.


// This is broadly the same as oop extraction, except we don't care about the contents of the loop
// (and as such, don't need the accumulation buffer). All we need to know is when we are past the
// loop. At that point, we resume processing the input using the @e rules.
// As such, these rules are presented without further exposition.


// This is the only non-internal rule.
// It is worth noting that because this formulation simply matches all tokens provided to it,
// it is extremely dangerous. Any mistake can cause an invocation to fail all the above
// rules, thus falling down to this one and triggering an infinite recursion.
// When you are writing, modifying, or debugging a macro like this, it is wise to temporarily
// prefix rules such as this one with something, such as @entry. This prevents the infinite
// recursion case, and you are more likely to get matcher errors at the appropriate place.




macro_rules! Ook {
    (@start $($Ooks:tt)* ) => {{

        fn ook() -> ::std::io::Result<Vec<CellType>> {

            use ::std::io;
            use ::std::io::prelude::*;
            // use ::std::io::Result;

            fn _re() -> io::Error {
                io::Error::new(
                    io::ErrorKind::Other,
                    String::from("ran out of input"),
                )
            }

            fn _inc(a: &mut [u8], i: usize) {
                let c = &mut a[i];
                *c = c.wrapping_add(1);
            }
            fn _dec(a: &mut [u8], i: usize) {
                let c = &mut a[i];
                *c = c.wrapping_sub(1);
            }

            let _r = &mut io::stdin();
            let _w = &mut io::stdout();

            let mut _a: Vec<CellType> = Vec::with_capacity(MEM_SIZE);
            _a.extend( ::std::iter::repeat(0).take(MEM_SIZE) );
            let mut _i = 0;
            {
                let _a = &mut *_a;
                Ook!(@e ( _a, _i, _inc, _dec, _r, _w, _re ); ($($Ooks)*) );
            }

            Ok(_a)

        }

        ook()
    }};

    (@e $syms:tt; () ) => {};
    //
    // Increment pointer
    (@e ($a:expr, $i:expr, $inc:expr, $dec:expr, $r:expr, $w:expr, $re:expr); ( Ook. Ook? $($tail:tt)* ) ) => {

        $i = ($i + 1) % MEM_SIZE;

        Ook!(@e ($a, $i, $inc, $dec, $r, $w, $re); ($($tail)*) );
    };
    //
    // Decrement pointer
    (@e ($a:expr, $i:expr, $inc:expr, $dec:expr, $r:expr, $w:expr, $re:expr); (Ook? Ook. $($tail:tt)*) ) => {


        $i = if $i == 0 {MEM_SIZE} else {$i} - 1;

        Ook!(@e ($a, $i, $inc, $dec, $r, $w, $re); ($($tail)*) );
    };

    // Increment pointee
    (@e ($a:expr, $i:expr, $inc:expr, $dec:expr, $r:expr, $w:expr, $re:expr); (Ook. Ook. $($tail:tt)* ) ) => {

        $inc($a, $i);

        Ook!(@e ($a, $i, $inc, $dec, $r, $w, $re); ($($tail)*) );
    };
    //
     // Decrement pointee
    (@e ($a:expr, $i:expr, $inc:expr, $dec:expr, $r:expr, $w:expr, $re:expr); (Ook! Ook! $($tail:tt)* ) ) => {

        $dec($a, $i);

        Ook!(@e ($a, $i, $inc, $dec, $r, $w, $re); ($($tail)*) );
    };

     // Write to stdout
    (@e ($a:expr, $i:expr, $inc:expr, $dec:expr, $r:expr, $w:expr, $re:expr); (Ook! Ook. $($tail:tt)* )  ) => {

        ($w.write_all(&$a[$i..{$i}+1]))?;
        Ook!(@e ($a, $i, $inc, $dec, $r, $w, $re); ($($tail)*) );
    };

    // Read from stdin
    (@e ($a:expr, $i:expr, $inc:expr, $dec:expr, $r:expr, $w:expr, $re:expr); (Ook. Ook! $($tail:tt)*)  ) => {

        match $r.read(&mut $a[$i..$i + 1]) {
            Ok(0) => Err($re()),
            ok @ Ok(..) => ok,
            err @ Err(..) => er r,
        }?;

        Ook!(@e ($a, $i, $inc, $dec, $r, $w, $re); ($($tail)*) );
    };

     // Begin a loop
    (@e ($a:expr, $i:expr, $inc:expr, $dec:expr, $r:expr, $w:expr, $re:expr); (Ook! Ook? $($tail:tt)* ) ) => {

         while $a[$i] != 0 {
            Ook!(@x ($a, $i, $inc, $dec, $r, $w, $re); (); (); ($($tail)*) );
         }
         Ook!(@s ($a, $i, $inc, $dec, $r, $w, $re); (); ($($tail)*) );
    };

    (@x $syms:tt; (); ($($buf:tt)*); (Ook? Ook! $($tail:tt)*) ) => {
        // Outer-most loop is closed. Process the buffered tokens.
        Ook!(@e $syms; ( $($buf)* ) );
    };

    (@x $syms:tt; ($($depth:tt)*); ($($buf:tt)*); (Ook! Ook? $($tail:tt)* ) )=> {
        // One level deeper
        Ook!(@x $syms; (@ $($depth)*); ($($buf)* Ook! Ook? ); ($($tail)*) );
    };

    (@x $syms:tt; (@ $($depth:tt)*); ($($buf:tt)*); (Ook? Ook! $($tail:tt)* ) )=> {
        // One level higher
        Ook!(@x $syms; ($($depth)*); ($($buf)* Ook? Ook! ); ($($tail)*));
    };

    (@x $syms:tt; $depth:tt; ($($buf:tt)*); (Ook $op0:tt Ook $op1:tt $($tail:tt)* ) ) => {
        Ook!(@x $syms; $depth; ($($buf)* Ook $op0 Ook $op1); ($($tail)*) );
    };

    // End of loop
    (@s $syms:tt; (); (Ook? Ook! $($tail:tt)* ) ) => {
        Ook!(@e $syms; ($($tail)*) );
    };

    // Enter nested loop
    (@s $syms:tt; ($($depth:tt)*); (Ook! Ook? $($tail:tt)* ) ) => {
        Ook!(@s $syms; (@ $($depth)*); ($($tail)*) );
    };

    // Exit nested loop
    (@s $syms:tt; (@ $($depth:tt)*); (Ook? Ook! $($tail:tt)* ) ) => {
        Ook!(@s $syms; ($($depth)*); ($($tail)*) );
    };

    // Not a loop opcode
    (@s $syms:tt; ($($depth:tt)*); (Ook $op0:tt Ook $op1:tt $($tail:tt)* ) ) => {
        Ook!(@s $syms; ($($depth)*); ($($tail)*) );
    };

    ($($Ooks:tt)*) => {
        Ook!( @start $($Ooks)* );
    };

}


#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_002() {

        let a = Ook!(
            Ook. Ook. Ook! Ook!
        );
        println!("{:?}", a);
    }

    #[test]
    fn test_001() {
        let _ = Ook!(
            Ook. Ook?  Ook. Ook.  Ook. Ook.  Ook. Ook.
            Ook. Ook.  Ook. Ook.  Ook. Ook.  Ook. Ook.
            Ook. Ook.  Ook. Ook.  Ook! Ook?  Ook? Ook.
            Ook. Ook.  Ook. Ook.  Ook. Ook.  Ook. Ook.
            Ook. Ook.  Ook. Ook.  Ook. Ook.  Ook. Ook.
            Ook. Ook?  Ook! Ook!  Ook? Ook!  Ook? Ook.
            Ook! Ook.  Ook. Ook?  Ook. Ook.  Ook. Ook.
            Ook. Ook.  Ook. Ook.  Ook. Ook.  Ook. Ook.
            Ook. Ook.  Ook! Ook?  Ook? Ook.  Ook. Ook.
            Ook. Ook.  Ook. Ook.  Ook. Ook.  Ook. Ook?
            Ook! Ook!  Ook? Ook!  Ook? Ook.  Ook. Ook.
            Ook! Ook.  Ook. Ook.  Ook. Ook.  Ook. Ook.
            Ook. Ook.  Ook. Ook.  Ook. Ook.  Ook. Ook.
            Ook! Ook.  Ook! Ook.  Ook. Ook.  Ook. Ook.
            Ook. Ook.  Ook! Ook.  Ook. Ook?  Ook. Ook?
            Ook. Ook?  Ook. Ook.  Ook. Ook.  Ook. Ook.
            Ook. Ook.  Ook. Ook.  Ook. Ook.  Ook. Ook.
            Ook. Ook.  Ook! Ook?  Ook? Ook.  Ook. Ook.
            Ook. Ook.  Ook. Ook.  Ook. Ook.  Ook. Ook?
            Ook! Ook!  Ook? Ook!  Ook? Ook.  Ook! Ook.
            Ook. Ook?  Ook. Ook?  Ook. Ook?  Ook. Ook.
            Ook. Ook.  Ook. Ook.  Ook. Ook.  Ook. Ook.
            Ook. Ook.  Ook. Ook.  Ook. Ook.  Ook. Ook.
            Ook. Ook.  Ook! Ook?  Ook? Ook.  Ook. Ook.
            Ook. Ook.  Ook. Ook.  Ook. Ook.  Ook. Ook.
            Ook. Ook.  Ook. Ook.  Ook. Ook.  Ook. Ook.
            Ook. Ook?  Ook! Ook!  Ook? Ook!  Ook? Ook.
            Ook! Ook!  Ook! Ook!  Ook! Ook!  Ook! Ook.
            Ook? Ook.  Ook? Ook.  Ook? Ook.  Ook? Ook.
            Ook! Ook.  Ook. Ook.  Ook. Ook.  Ook. Ook.
            Ook! Ook.  Ook! Ook!  Ook! Ook!  Ook! Ook!
            Ook! Ook!  Ook! Ook!  Ook! Ook!  Ook! Ook.
            Ook! Ook!  Ook! Ook!  Ook! Ook!  Ook! Ook!
            Ook! Ook!  Ook! Ook!  Ook! Ook!  Ook! Ook!
            Ook! Ook.  Ook. Ook?  Ook. Ook?  Ook. Ook.
            Ook! Ook.  Ook! Ook?  Ook! Ook!  Ook? Ook!
            Ook. Ook.  Ook. Ook.  Ook. Ook.  Ook. Ook.
            Ook. Ook.  Ook. Ook.  Ook. Ook.  Ook. Ook.
            Ook. Ook.  Ook. Ook.  Ook! Ook.
        );


        //
        // if let Result::Ok(x) = v {
        //     println!("{:?}", x);
        //
        //     // let s = String::from_utf8(x).expect("Found invalid UTF-8");
        //     //
        //     //
        //     // println!("{:?}", s);
        // }
        //
        // // let r = Result::<i32, i32>::Ok(3);
        // //
        // // if let Result::Ok(vv) = r {
        // //     println!("{}", vv);
        // // }
        //
        // let bytes = vec![0x41, 0x42, 0x43];
        // let s = String::from_utf8(bytes).expect("Found invalid UTF-8");
        // println!("{}", s);


    }
}

