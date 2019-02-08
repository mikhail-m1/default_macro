#[macro_export]
macro_rules! default {
    (
        $name:ident{ $($field: ident : $value:expr),* $(,)* }
    ) => {
        $name { $($field: $value, )* .. $name::default() }
    }
}

#[cfg(test)]
mod tests {
    #[derive(Default, Debug)]
    struct Foo { a: i32, b: f64 }

    #[test]
    fn it_works() {
        check(default!(Foo{}), "Foo { a: 0, b: 0.0 }");
        check(default!(Foo{a:1}), "Foo { a: 1, b: 0.0 }");
        check(default!(Foo{a:1,}), "Foo { a: 1, b: 0.0 }");
        check(default!(Foo{a:1, b: 1.0}), "Foo { a: 1, b: 1.0 }");
    }

    fn check(foo: Foo, result: &str) {
        assert_eq!(format!("{:?}", foo), result);
    }
}
