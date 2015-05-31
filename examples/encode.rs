extern crate ron;

#[allow(dead_code)]
#[derive(Serialize)]
enum Foo<'a> {
    Var1(u8),
    Var2([&'a str; 2]),
}

#[derive(Serialize)]
struct Bar<'a> {
    q: Option<(f32, bool)>,
    w: Foo<'a>,
}

fn main() {
    let bar = Bar {
        q: Some((0.4, true)),
        w: Foo::Var2(["a", "b"])
    };

    let s = ron::to_str(&bar).unwrap();
    println!("{}", s);
}
