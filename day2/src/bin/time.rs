use std::time::{SystemTime, UNIX_EPOCH, Duration};

fn main() {
    // Get current system time.
    let now = SystemTime::now();

    // And it will return a Result.
    let duration_since_result = now.duration_since(UNIX_EPOCH);
    println!("duration_since_result:{:?}", duration_since_result);

    // 1️⃣ We can unwrap it to get inner value. 😃
    let duration_since:Duration = duration_since_result.unwrap();
    println!("1️⃣ duration_since:{:?}", duration_since);

    // 2️⃣ Or use match to handle Result<Ok(Duration),Err(())>
    let duration = match now.duration_since(UNIX_EPOCH) {
        // Handle happy case.
        Ok(duration) => duration,

        // Handle error case.
        Err(err) => panic!("{:?}", err),
    };
    println!("2️⃣ duration:{:?}", duration);

    // But what if result is error? 😱
    let duration_since_result = Err(());

    // // 💥 👇 It will panic and crash with no reason. 😭
    // let duration_since:Duration = duration_since_result.unwrap();
    // println!("💥 duration_since:{:?}", duration_since);

    // 3️⃣ You can fallback with unwrap_or.
    let duration:Duration = duration_since_result.unwrap_or(Duration::new(0u64,0u32));
    println!("3️⃣ duration:{:?}", duration);

    // 4️⃣ Or panic with a reason, we will use `expect` instead. 🫣
    let _duration:Duration = duration_since_result.expect("4️⃣ 🔥 Expect some number.");
}