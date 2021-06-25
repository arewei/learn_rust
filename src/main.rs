extern crate error_chain;

use std::error::Error;

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

    test::test_hashmap();

    Ok(())
}
