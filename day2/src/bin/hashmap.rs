use std::collections::HashMap;
fn main(){
    let mut foo_hashmap=HashMap::new();
    foo_hashmap.insert("name","foo");
    foo_hashmap.insert("age","42");

    let mut foo_hashmap=HashMap::from(
        [
            ("name","foo"),
            ("age","42"),
        ]
    );

    let maybe_name=foo_hashmap.get("name");
    let maybe_name=foo_hashmap.get("name").copied();
    let maybe_name=foo_hashmap.get::<str>("name");

    match maybe_name{
        Some(name)=>println!("hello {name}"),
        None => panic!("who!"),
    };

    let unwrapped_name = maybe_name.unwrap_or(&"who!?");

    // And assign back by return after matched.
    let hi = match unwrapped_name {
        &"foo" => format!("3️⃣ unwrapped_name:{unwrapped_name}"), // Will return unwrapped_name.
        _ => panic!("who!?"),                                    // `_` aka `default` in js.
    };

    println!("{hi}");

    
}