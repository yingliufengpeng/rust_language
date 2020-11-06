
#[derive(Clone, Debug)]
struct A {
    v: i32,
}

#[derive(Copy, Clone, Debug)]
struct Generate<T>(fn() -> T);

fn main() {
    println!("Hello, world!");

    let a = A {v: 3};
    let b = a.clone();
    println!("{:?}", a);

    let g = Generate(|| 3);
    let g2 = g;


}
