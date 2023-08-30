// ------------------------------------------------ //
//                                                  //
//                                                  //
//                   DOT MEOW (🐈‍)                  //
//                                                  //
//                  By Th3Maid 2023                 //
//                                                  //
//                                                  //
// ------------------------------------------------ //

mod core;
use crate::core::core::read_meow;

fn main() {
    let result = read_meow("../samples/sample.meow", true);
    println!("{:?}", result);
}
