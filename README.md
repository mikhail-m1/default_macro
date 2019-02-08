# default! macro for Rust

example:

```
use default_macro::default;

#[derive(Default)]
struct Foo { a: i32, b: f64 }

default!(Foo{}) // Foo { a: 0, b: 0.0 }
default!(Foo{a:1}) // Foo { a: 1, b: 0.0 }
default!(Foo{a:1,}) // Foo { a: 1, b: 0.0 }"
default!(Foo{a:1, b: 1.0}) // Foo { a: 1, b: 1.0 }"
```
