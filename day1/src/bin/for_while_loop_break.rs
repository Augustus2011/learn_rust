fn main(){
   let mut count=0;
   
   for _i in 0..8{
    count+=1;
   }

   println!("1. count = {count}");

   for i in 0..=9{
    count+=i;
   }

   println!("2. count = {count}",count=count);

   for e in ["a","b","c"] {
    println!("3. {e}");
   }

   for (i,e) in ["a","b","c"].iter().enumerate(){
    println!("4. {i}={e}");
   }

   while count < 50 {
    count += 1;
}

    println!("5. count = {0}", count);

    loop{
        count+=1;
        if count>=100{
            break;
        }
    }
    println!("6. count = {}",count);
   
    'outer: loop {
        count += 1;
        if count >= 200 {
            break;
        } else {
            'inner: loop {
                count += 1;
                if count >= 150 {
                    break 'outer;
                }
            }
        }
    }

    println!("7. count = {}", count);
}
