#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
include!(concat!(env!("OUT_DIR"), "\\bindings.rs")); 
fn main(){
    unsafe {
        putpixel(1,2,3);
    }
    println!("{}", concat!(env!("OUT_DIR"), "\\bindings.rs"));
}