extern crate proc_macro;
extern crate syn;

#[macro_use]
extern crate quote;

use proc_macro::TokenStream;

#[proc_macro_derive(HelloWorld2, attributes(HelloWorldName))]
pub fn hello_world(input: TokenStream) -> TokenStream {
    // Construct a string representation of the type definition
    let s = input.to_string();

    // Parse the string representation
    let ast = syn::parse_derive_input(&s).unwrap();

    // Build the impl
    let gen = impl_hello_world(&ast);

    // Return the generated impl
    gen.parse().unwrap()

}

fn impl_hello_world(ast: &syn::DeriveInput) ->  quote::Tokens {
    let name = &ast.ident;
    // Check if derive(HelloWorld2) was specified for a struct

    if let syn::Body::Struct(_) = ast.body { // body is a struct, it's applied for pattern match
        // Yes, this is a struct
        quote! {
            impl HelloWorld for #name {
                fn hello_world() {
                    println!("Hello, World! My name is {}", stringify!(#name));
                }
            }
        }
    } else {
        // Nope. This is an Enum. We cannot handle these!
        panic!("#[derive(HelloWorld2)] is only defined for structs, not for enums!");
    }

}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
