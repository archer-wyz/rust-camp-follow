use std::ops::Deref;

#[derive(Debug)]
struct A {
    a: i32,
}

struct B {
    a: A,
}

impl Deref for B {
    type Target = A;

    fn deref(&self) -> &Self::Target {
        &self.a
    }
}

fn test_deref(a: &A) {
    println!("{:?}", a)
}

fn main() {
    let b = B { a: A { a: 1 } };
    //test_mut_ref(&mut *b);
    let a = &*b;
    println!("{:?}", a);
    // error: deref which would be done by auto-deref
    //   --> camp-test/examples/ref.rs:29:16
    //    |
    // 29 |     test_deref(&*b);
    //    |                ^^^ help: try: `&b`
    //    |
    //    = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#explicit_auto_deref
    //    = note: `-D clippy::explicit-auto-deref` implied by `-D warnings`
    //    = help: to override `-D warnings` add `#[allow(clippy::explicit_auto_deref)]`
    //
    test_deref(&b);
    println!("{}", b.a.a);
}
