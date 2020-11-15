macro_rules! as_expr { ($e:expr) => {$e} }
macro_rules! as_item { ($i:item) => {$i} }
macro_rules! as_pat { ($p:pat) => {$p} }
macro_rules! as_stmt { ($s:stmt) => {$s} }
// Note that this specific set of macros is determined by what macros
// are allowed to expand to, not what they are able to capture. This is
// because macros cannot appear in type position, you cannot have an as_ty!
// macro.



#[cfg(test)]
mod tests {
    #[test]
    fn test_001() {
        let r = as_expr!((3 + 4));
        println!("{}", r);

        as_item! {
            #[derive(Debug)]
            struct A(i32);
        }
        ;

        let a = A(34);
        println!("{:?}", a);

        as_stmt! {
        let r = 20
        };
        println!("{}", r);
    }
}