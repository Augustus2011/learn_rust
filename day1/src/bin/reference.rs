fn main() {
    let foo_str = "str and String";
    let bar_string = String::from("str and String");

    // String → &str
    let bar_str = bar_string.as_str();

    println!("bar_string: {bar_string}");
    println!("bar_str: {bar_str}");

    // &str → String
    assert_eq!(bar_string, foo_str.to_string());
}