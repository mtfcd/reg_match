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

#[derive(PartialEq, Debug)]
struct Email {
    username: String,
    domain: String,
}

#[derive(PartialEq, Debug)]
enum Account {
    Phone(String),
    Email(Email),
    Invalid,
}

fn match_account(input: &str) -> Account {
    reg_match!(input {
        r"(?<name>.+)@(?<domain>.+)" => Account::Email(Email{username: name.to_string(), domain: domain.to_string()}),
        r"(?<phone>\d{6,11})" => Account::Phone(phone.to_string()),
        _ => Account::Invalid
    })
}

#[test]
fn match_enum() {
    assert_eq!(
        Account::Email(Email {
            username: "abc".to_string(),
            domain: "email.com".to_string()
        }),
        match_account("abc@email.com")
    );
    assert_eq!(
        Account::Phone("66668888".to_string()),
        match_account("66668888")
    );
    assert_eq!(
        Account::Invalid,
        match_account("abc123")
    );
}
