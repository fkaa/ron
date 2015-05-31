#![feature(custom_derive, plugin)]
#![plugin(serde_macros)]

extern crate serde;
extern crate serde_macros;
extern crate ron;

#[derive(Deserialize)]
enum Foo<'a> {
    Var1(u8),
    Var2([&'a str; 2]),
}

#[derive(Deserialize)]
struct Bar<'a> {
    q: Option<(f32, bool)>,
    w: Foo<'a>,
}

fn main() {
    let s =
"Bar (\n
    Some(0.4, true),\n
    Foo::Var2([\"a\", \"b\"])\n
)";
    let bar: Bar = ron::from_str(&s).unwrap();
    println!("{}", s);
}
