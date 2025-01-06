use reg_match::reg_match;

#[test]
fn one_group() {
    let e = "1";
    let a = reg_match!(e {
        r"(?<b>\d)" => b.to_string(),
        "2" => "2".to_string(),
        _ => "0".to_string()
    });
    assert_eq!("1", a);
}

#[test]
fn two_group() {
    let input = "123abc";
    let output = reg_match!(input {
        r"(?<digits>\d+)(?<letters>.+)" => format!("{}-{}", letters, digits),
        _ => "".to_string()
    });
    assert_eq!("abc-123", output);
}

#[test]
fn nested_group() {
    let e = "123abc";
    let a = reg_match!(e {
        r"(?<b>\d+(?<c>.+))" => format!("{}-{}", b, c),
        "2" => "2".to_string(),
        _ => "0".to_string()
    });
    assert_eq!("123abc-abc", a);
}

#[test]
fn second_arm() {
    let e = "123abc";
    let a = reg_match!(e {
        "4" => "4".to_string(),
        r"(?<b>\d+(?<c>.+))" => format!("{}-{}", b, c),
        _ => "0".to_string()
    });
    assert_eq!("123abc-abc", a);
}

#[test]
fn no_default_arm() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/exp/no_default.rs");
}
