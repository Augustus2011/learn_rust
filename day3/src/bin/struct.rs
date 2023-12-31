fn main(){

    let animal =(("name","foo"),("age",42)); //tuple
    println!("{0:?}:{1:?}",animal.0 .0, animal.0 .1);
    println!("{0:?}:{1:?}",animal.1 .0 ,animal.1 .1);

    struct Animal{
        name:String,
        age:u8,
    }

    let animal=Animal{
        name:"foo".to_string(),
        age:42u8,
        
    };
    
    println!("name:{:?}",animal.name);
    println!("age:{:?}",animal.age);
    

}