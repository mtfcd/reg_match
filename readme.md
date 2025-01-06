# reg_match!

[中文](./ch.md)

Regex capturing is a very handy tool for extracting strings, but it can't be used directly in a match statement. This macro provides a more convenient method:

```rust
// use reg_match;

let input = "123abc";
let output = reg_match!(input {
    r"(?<digits>\d+)(?<letters>.+)" => format!("{}-{}", letters, digits),
    _ => "".to_string()
});
assert_eq!("abc-123", output);
```

By using the `reg_match!` macro, you can directly extract variables from named capture groups within the expression.

### inspired by

[structre](https://github.com/andrewbaxter/structre)
