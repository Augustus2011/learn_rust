#[derive(Debug,Clone)]

struct Animal{
    #[allow(dead_code)]//allow dead code
    name:String,
    age:u8,
    r#type:String,
}

impl Animal{
    fn new(name:&str,age:u8) -> Self {
        Animal{
            name:name.to_string(),
            age,
            r#type:"duck".to_owned(),
        }
    }

    fn new_cat(name:&str,age:u8)-> Self{
        Animal{
            name:name.to_owned(),
            age,
            r#type:"cat".to_owned(),
        }
    }

    pub fn static_say(animal_type:&str) ->&str{
        match animal_type{
            "cat"=>"meaow",
            "duck"=>"quack",
            _=>"wat!",
        }
    }
    
    pub fn say(&self)->&str{
        let animal_type=self.r#type.as_str();
        Animal::static_say(animal_type)
    }
}


fn main(){
    let animal =Animal::new("foo",42u8);
    println!("animal:{:#?}",animal);
    let static_say_str=Animal::static_say("duck");
    
    println!("static_say_str:{:#?}",static_say_str);
}