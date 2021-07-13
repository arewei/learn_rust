extern crate error_chain;

use std::error::Error;
// use std::sync::Arc;
// use std::thread;

mod phrases;
mod algorithms;
mod test;


fn main() -> Result<(), Box<dyn Error>> {
    // let _b = test::test3();
    //
    // algorithms::main1();
    // phrases::greetings::hello();
    //
    // let a = [0, 1, 2, 3, 4];
    // let middle = &a[1..4];
    // println!("{:?}", middle);
    //
    // let origin = test::Point3d::default();
    // println!("{:?}", origin);
    //
    // test::test2();
    //
    // let _a = test::test_file();
    //
    // let _b = test::test_read_write();
    //
    // test::test4();
    //
    // test::test_string();

    // test::test_hashmap();

    // match test::double_arg(std::env::args()) {
    //     Ok(n) => println!("{}", n),
    //     Err(err) => println!("Error: {}", err),
    // }

    // 创建一个线程
    // let new_thread = thread::spawn(move || {
    //     // 再创建一个线程
    //     thread::spawn(move || {
    //         loop {
    //             println!("I am a new thread.");
    //         }
    //     })
    // });

    // 等待新创建的线程执行完成
    // new_thread.join().unwrap();
    // println!("Child thread is finish!");

    // 睡眠一段时间，看子线程创建的子线程是否还在运行
    // thread::sleep_ms(10);

    // let numbers: Vec<_> = (0..20u32).collect();
    // let shared_numbers = Arc::new(numbers);
    //
    // for _ in 0..10 {
    //     let child_numbers = shared_numbers.clone();
    //
    //     let thread = thread::spawn(move || {
    //         let local_numbers = &child_numbers[..];
    //
    //         println!("share value in new thread: {:?}, address: {:p}", local_numbers, &*local_numbers);
    //     });
    //
    //     thread.join().unwrap();
    // }

    test::find_file();

    Ok(())
}
