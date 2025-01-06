# reg_match!

正则表达式的捕获是一个很好用的提取字符串的工具,但是不用直接用在`match`语句中. 这个宏提供了一个稍微方便一点的方法:

```Rust
let input = "123abc";
let output = reg_match!(input {
    r"(?<digits>\d+)(?<letters>.+)" => format!("{}-{}", letters, digits),
    _ => "".to_string()
});
assert_eq!("abc-123", output);
```

使用`reg_match!`宏,可以直接把表达式中的命名捕获组中的变量取出.


### inspired by

[structre](https://github.com/andrewbaxter/structre)
