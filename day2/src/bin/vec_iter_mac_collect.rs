use std::vec;

fn main(){
    let array_of_foo=["foo","bar"]; //array of strings
    let mut vec_of_foo = vec!["foo","bar"];

    println!("array_of_foo: {array_of_foo:#?}");
    println!("vec_of_foo: {vec_of_foo:#?}");

    vec_of_foo.push("baz"); //like stack
    println!("vec_of_foo:{vec_of_foo:#?}");
    vec_of_foo.pop();

    println!("vec_of_foo:{vec_of_foo:#?}");

    let hello_vec = vec_of_foo
        .iter() // Must `iter()` before you can map, filter,...
        .map(|e| format!("hello {e}")) // Say hi to `closure` |e| aka (e)=> in js.
        .collect::<Vec<_>>(); // `collect` inferred type from iterate.
        //             👆 `_` is inferred type (let compiler desire).

    println!("hello_vec: {hello_vec:#?}");

    let indexed_vec = vec_of_foo
    .iter()
    .enumerate() // To access index we need `enumerate`.
    .map(|(i, e)| (i, e)) // Say hi to `Tuple` type.
    .collect::<Vec<(usize, &&str)>>(); // i is `usize`, e is &&str.

    println!("indexed_vec: {indexed_vec:#?}");


}
