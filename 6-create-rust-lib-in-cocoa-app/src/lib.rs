#[no_mangle]
pub extern fn square(num: i32) -> i32 {
    num * num
}

#[no_mangle]
pub extern fn cube(num: i32) -> i32 {
    square(num) * num
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cube() {
        assert_eq!(27, cube(3));
    }

    #[test]
    fn test_square() {
        assert_eq!(16, square(4));
    }
}
