#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("cxx-demo/include/add.h");

        fn add(x: i32, y: i32) -> i32;
    }
}

fn main() {
    let res = ffi::add(1, 2);
    println!("res = {}", res);
}
