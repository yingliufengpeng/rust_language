#[cfg(test)]
mod tests {

    #[inline(always)]
    fn super_fast_fn() -> i32 {
        32
    }

    #[test]
    fn check() {
        assert_eq!(2, 1 + 1);

        let answer = super_fast_fn();

        println!("{}", answer);
    }
}
