#[macro_export]
macro_rules! default {
    ( $name:ident { } ) => { $name::default() };
    ( $name:ident { $field_name:tt : $value:tt $($tail:tt)* } ) => {
        default!(@x $name { $field_name : } {$value} $($tail)* )
    };
    ( $name:expr ) => { $name };
    ( @x $name:ident {$($processed:tt)*} {$($current:tt)+} $(,)? ) => {
        $name{ $($processed)* default!($($current)+)  , ..$name::default()}
    };
    ( @x $name:ident {$($processed:tt)+} {$($current:tt)+} , $field_name:tt : $value:tt $($tail:tt)* ) => {
        default!(@x $name {$($processed)+ default!($($current)+), $field_name :} {$value} $($tail)*)
    };
    ( @x $name:ident {$($processed:tt)+} {$($current:tt)+} $value:tt $($tail:tt)*) => {
        default!( @x $name {$($processed)+} {$($current)+ $value} $($tail)* )
    };
}

#[cfg(test)]
mod tests {
    #[derive(Default, Debug)]
    struct Foo {
        a: i32,
        b: f64,
        c: Bar,
    }
    #[derive(Default, Debug)]
    struct Bar {
        d: i32,
        e: i32,
    }

    #[test]
    fn it_works() {
        check(default!(Foo {}), Foo::default());
        check(
            default!(Foo { a: 1 }),
            Foo {
                a: 1,
                ..Foo::default()
            },
        );
        check(
            default!(Foo { a: 1, b: 1.0 }),
            Foo {
                a: 1,
                b: 1.0,
                ..Foo::default()
            },
        );

        check(
            default!(Foo { a: 1 + 2, b: 1.0 }),
            Foo {
                a: 3,
                b: 1.0,
                ..Foo::default()
            },
        );
        check(
            default!(Foo { c: Bar { d: 1 } }),
            Foo {
                c: Bar {
                    d: 1,
                    ..Bar::default()
                },
                ..Foo::default()
            },
        );
    }
    fn check(foo1: Foo, foo2: Foo) {
        assert_eq!(format!("{:?}", foo1), format!("{:?}", foo2));
    }
}
