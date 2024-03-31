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
