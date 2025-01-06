use reg_match::reg_match;

fn main() {
    let e = "123abc";
    let a = reg_match!(e {
        "4" => "4".to_string(),
        r"(?<b>\d+(?<c>.+))" => format!("{}-{}", b, c),
        //_=>"a".to_string()
    });
    assert_eq!("123abc-abc", a);
}
