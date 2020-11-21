

//!
//! # Subtyping and Variance
//!
//! # Variance
//!     & and &mut are type constructors that take two inputs: a lifetime, and a type to point to.
//!
//!
//!
//!



trait Animal {
    fn snuggle(&self);
    fn eat(&mut self);
}

trait Cat: Animal {
    fn meow(&self);
}

trait Dog: Animal {
    fn bark(&self);
}

fn love(pet: &dyn Animal) {
    pet.snuggle();
}

struct DDog {


}

fn evil_feeder<T>(input: &mut T, val: T) {
    *input = val;
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_001() {
        let mut mr_snuggles: &'static str = "meow! :3";  // mr. snuggles forever!!
        // {
        //     let spike = String::from("bark! >:V");
        //     let spike_str: &str = &spike;                // Only lives for the block
        //     evil_feeder(&mut mr_snuggles, spike_str);    // EVIL!  in the caller we pass in &mut &'static str and &'spike str
        // }
        // println!("{}", mr_snuggles);                     // Use after free?
    }

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
