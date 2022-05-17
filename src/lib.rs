pub fn add_numbers(number_one: i64, number_two: i64) -> i64 {
    number_one + number_two
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    fn test_add_numbers() {
        let result = add_numbers(2, 8);
        assert_eq!(result, 10);
    }
}
