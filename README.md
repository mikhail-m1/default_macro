# default! macro for Rust

Sometimes you want to create nested structures, set values for some the fields and use default value for most of the other, for example:

```
use default_macro::default;

#[derive(Default)]
struct Window { title: &str, border: Border, /* 10 other fields */ }

#[derive(Default)]
struct Border { width: f64, /* 5 other fields*/ }

fn foo() {
    let w1 = Window { 
        title: "Test", 
        border: Border {  
            width: 10.0, 
            ..Border::Default}, 
        ..Window::default()
    };

    // with the macros:
    let w2 = default!( Window { 
        title: "Test", 
        border: Border {  width: 10.0} 
    });
}
```