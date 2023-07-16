mod lib_other_func;
#[no_mangle]
pub extern "C" fn my_add(x: i32, y: i32) -> i32 {
    x + y
}
#[no_mangle]
pub extern "C" fn my_sub(x: i32, y: i32) -> i32 {
    x - y
}

#[no_mangle]
pub extern "C" fn my_multiply(x: i32, y: i32) -> i32 {
    lib_other_func::lib_other_func::my_multiply_internal(x, y)
}

#[no_mangle]
pub extern "C" fn my_divide(x: i32, y: i32) -> i32 {
    lib_other_func::lib_other_func::my_divide_internal(x, y)
}
// test
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(my_add(2, 2), 4);
    }
    #[test]
    fn it_works2() {
        assert_eq!(my_sub(2, 2), 0);
    }
}
