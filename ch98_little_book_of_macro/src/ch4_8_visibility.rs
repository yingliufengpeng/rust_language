

macro_rules! struct_name {
    ( $(pub)* struct $name:ident $($rest:tt)* ) => {  stringify!($name)};
}

// Because you cannot bind a repetition in and of itself to a variable, there is no way to store the contents
// of $(pub)* such that is can be substituted. As a result, multiple rules are needed

macro_rules! new_type_new {
    (struct $name:ident($t:ty);) => { new_type_new!{ () struct $name($t);}  };
    (pub struct $name:ident($t:ty);) => {new_type_new!{(pub) struct $name($t); } };

    (( $($vis:tt)* ) struct $name:ident($t:ty); ) => {

         as_item! {
            impl $name {
                $($vis)* fn new(value: $t) -> Self {
                    $name(value)
                }
            }
         }

    };
}

macro_rules! as_item { ($i:item) => {$i}; }

#[derive(Debug)]
pub struct Stu(i32);
new_type_new!{
    struct Stu(i32);
}

// In this case, we are using the ability to match an arbitrary sequence of tokens inside a group to match either () or (pub)
// , then substitute the contents into the output. Because the parser will not expect a tt repetition expansion in this position,
// we need to use AST coercion to get the expansion to parse correctly.


#[cfg(test)]
mod tests {
    use super::Stu;
    //
    struct A {}


    #[test]
    fn test_001() {
        println!("ok");

        let x = struct_name! {

          pub struct Stu {
                width: u32,
                height: u32,
            };
        };
        println!("{}", x);

        let r = Stu::new(3);
        println!("{:?}", r);

    }
}