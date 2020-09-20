
use colored::*;

#[no_mangle]
pub extern "C" fn hello() {
    println!("{}", "hello world!".green());
}
