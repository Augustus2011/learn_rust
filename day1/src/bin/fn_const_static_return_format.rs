const COUNT:&str="count"; //slice of str
static mut TOTAL:u32=0; //unsigned 32 interger :[0,.....2^32]
    
fn add(a:i32,b:i32)->i32{
    a+b //return a+b ,no semicolon
}
fn main(){
    assert!(add(1,2)==3);
    let result = format!("{COUNT}= {}",add(1,9));
    println!(" 1. {result}");

    unsafe {
        TOTAL = add(3,4) as u32; //cast output i32 to u32
        assert_eq!(TOTAL,7);
    }    
}