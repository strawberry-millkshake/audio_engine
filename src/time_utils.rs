use std::time::SystemTime;


fn get_now(now: SystemTime) -> u64 {
    match now.elapsed() {
        Ok(elapsed) => elapsed.as_secs(),
        Err(e) => {
            println!("Error: {e:?}");
            0
        }
    }
}


// // ~~~ SECOND COUNTER ~~~

// let now = SystemTime::now();
// let mut second_counter = 0;


// if second_counter != get_now(now) {
//     second_counter += 1;
// }