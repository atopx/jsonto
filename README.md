# jsonto
json string to frequently used programming languages type, it's lightweight and fast

# example

```rust
fn main() {
    let input_str = r#"{
        "code": "country",
        "type": "PROPERTY_INT",
        "value": "国家"
    }"#;

    let mut options = jsonto::Options::default();
    options.output_mode = jsonto::OutputMode::Rust;
    let result = jsonto::codegen("property", input_str, options).unwrap();
    println!("-----auto genaration-----\n{}", result);
}
```
