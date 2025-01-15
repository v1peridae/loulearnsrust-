fn main() {
}

fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn div(a:f64, b:f64) -> Result<f64, String> {
    if b == 0.0 {
        return Err("Division by zero".to_string());
    }
    Ok(a / b)
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_add(){
        assert_eq!(add(1,2), 3);
    }

    #[test]
    fn test_div(){
        assert_eq!(div(10.0, 2.0), Ok(5.0));
    }

    #[test]
    fn test_div_by_zero(){
        assert_eq!(div(10.0, 0.0), Err("Division by zero".to_string()));
    }

    #[test]
    #[should_panic(expected = "Panic thingy")]
    fn test_panic(){
        panic!("Panic thingy");
    }

    macro_rules! assert_result {
        ($a:expr, $b:expr, $tolerance:expr) => {
            assert!(
                ($a as f64 - $b as f64).abs() < $tolerance,
                "{} is not within {} of {}",
                $a,
                $tolerance,
                $b
            );
        }
    }

    #[test]
    fn test_float_comp() {
        let result = 0.1 + 0.2;
        assert_result!(result, 0.3, 0.0001);
    }
}