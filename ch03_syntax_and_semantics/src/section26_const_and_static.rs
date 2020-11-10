#[cfg(test)]
mod tests {

    const N: i32 = 5;
    static MN: i32 = 5;
    static NAME: &'static str = "Steve"; // The type of a static value must be Sync unless the static value is mutable.

    static mut N2: i32 = 5;

    // Dropping
    // Types implementing Drop are allowed in const and static definitions. Constants are inlined
    // where they used and are dropped accordingly. static values are not dropped.

    #[test]
    fn test() {
        unsafe {
            N2 += 1;
            println!("N2: {}", N2);
        }
    }
}
