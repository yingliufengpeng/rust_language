mod chinese;
mod french;
mod phrases;

mod section01_variable_bindings;
mod section02_functions;
mod section03_primitive_types;
mod section04_comments;
mod section05_if;
mod section06_loops;
mod section07_vectors;
mod section08_ownership;
mod section09_references_and_borrowing;
mod section10_lifetimes;
mod section11_mutablity;
mod section12_struct;
mod section13_enums;
mod section14_match;
mod section15_patterns;
mod section16_method_syntax;
mod section17_strings;
mod section18_generics;
mod section19_traits;
mod section20_drop;
mod section21_if_let;
mod section22_trait_objects;
mod section23_closures;
mod section24_universal_function_call_syntax;
mod section25_crates_and_modules;
mod section26_const_and_static;
mod section27_attributes;
mod section28_type_aliases;


pub fn foo() {

}

trait Foo {
    fn method(&self) -> String;
}

impl Foo for u8 {
    fn method(&self) -> String {
        println!("u8 for method");
        format!("u8: {}", *self)
    }
}

impl Foo for String {
    fn method(&self) -> String {
        println!("string for method");
        format!("string: {}", *self)
    }
}
