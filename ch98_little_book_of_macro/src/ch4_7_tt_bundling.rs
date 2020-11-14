
macro_rules! log {
    ($e:tt) => { println!("item is {:?}", "i"); };
}

macro_rules! call_a_or_b_tail {
    ((a: $a:expr, b: $b:expr), call a: $($tail:tt)*) => {{
        $a(stringify!($($tail)*))
    }};
    ((a: $a:expr, b: $b:expr), call b: $($tail:tt)*) => {{
        $b(stringify!($($tail)*))
    }};

    ($ab:tt, $_skip:tt $($tail:tt)*) => {
        {
              // let r = $_skip;
              // println!("{:?}", r);
              log!($_skip);
              call_a_or_b_tail!($ab, $($tail)*)
        }

    };
}

fn compute_len(s: &str) -> Option<usize> {
    println!("tail: {:?}", s);

    Some(s.len())
}

fn show_tail(s: &str) -> Option<usize> {
    println!("tail: {:?}", s);
    None
}




#[cfg(test)]
mod tests {

    #[test]
    fn test_001() {
        // let r = call_a_or_b_tail!((a: super::compute_len, b: super::show_tail), "tttt" call b: "tttt");
        // println!("{:?}", r);
        //
        // let r = call_a_or_b_tail!((a: super::compute_len, b: super::show_tail), "tttt" call a: "tttt");
        // println!("{:?}", r);

        let r = call_a_or_b_tail!(
             (a: super::compute_len, b: super::show_tail),
            the recursive part that skips over all these
            tokens doesn't much care whether we will call a
            or call b: only the terminal rules care.
        );
        println!("{:?}", r);

        let r = call_a_or_b_tail!(
            (a: super::compute_len, b: super::show_tail),
            and now, to justify the existence of two paths
            we will also call a: its input should somehow
            be self-referential, so let's make it return
            some ninety one!
        );
        println!("{:?}", r);



    }

}

// In particularly complex recursive macros, a large number of arguments may be
// needed in order to carry identifiers and expressions to successive layers.
// However, depending on the implementation there may be may intermediate layers
// which need to forward these arguments, but do not need to use them.

// As such, it can be very useful to bundle al such arguments together into a single TT
// by placing them in a group. This allows layers which do not need to use the arguments
// to simply capture and substitute a single tt, rather than having to exactly capture and substitute
// the entire argument group.










