use std::env;

pub fn get_int_from_env(env_var: String) -> u32 {
    let string = env::var(&env_var)
        .unwrap_or_else(|_| panic!("{} must be set in environment as unsigned int", &env_var))
        .to_string();
    let int = string.clone().parse::<u32>();

    match int {
        Err(error) => panic!("Cannot parse {} environment var to int! {}", env_var, error),
        Ok(int) => int,
    }
}

pub fn get_float_from_env(env_var: String) -> f32 {
    let string = env::var(&env_var)
        .unwrap_or_else(|_| panic!("{} must be set in environment as float", &env_var))
        .to_string();
    let float = string.clone().parse::<f32>();

    match float {
        Err(error) => panic!(
            "Cannot parse {} environment var to float! {}",
            env_var, error
        ),
        Ok(float) => float,
    }
}

#[cfg(test)]
mod tests {
    use super::get_float_from_env;
    use super::get_int_from_env;
    use std::env;

    #[test]
    fn test_get_int_from_env_valid() {
        env::set_var("TEST_INT_VALID", "42");
        assert_eq!(get_int_from_env("TEST_INT_VALID".to_string()), 42);
        env::remove_var("TEST_INT_VALID");
    }

    #[test]
    #[should_panic(expected = "TEST_INT_NOT_SET must be set in environment as unsigned int")]
    fn test_get_int_from_env_not_set() {
        get_int_from_env("TEST_INT_NOT_SET".to_string());
    }

    #[test]
    #[should_panic(expected = "Cannot parse TEST_INT_NOT_NUMBER environment var to int!")]
    fn test_get_int_from_env_invalid_format() {
        env::set_var("TEST_INT_NOT_NUMBER", "not_a_number");
        get_int_from_env("TEST_INT_NOT_NUMBER".to_string());
        env::remove_var("TEST_INT_NOT_NUMBER");
    }

    #[test]
    fn test_get_float_from_env_valid() {
        env::set_var("TEST_FLOAT", "3.14");
        assert_eq!(get_float_from_env("TEST_FLOAT".to_string()), 3.14);
        env::remove_var("TEST_FLOAT");
    }

    #[test]
    #[should_panic(expected = "must be set in environment as float")]
    fn test_get_float_from_env_not_set() {
        get_float_from_env("TEST_FLOAT_NEW".to_string());
    }

    #[test]
    #[should_panic(expected = "Cannot parse TEST_FLOAT_NOT_A_FLOAT environment var to float!")]
    fn test_get_float_from_env_invalid_format() {
        env::set_var("TEST_FLOAT_NOT_A_FLOAT", "not_a_float");
        get_float_from_env("TEST_FLOAT_NOT_A_FLOAT".to_string());
        env::remove_var("TEST_FLOAT_NOT_A_FLOAT");
    }
}
