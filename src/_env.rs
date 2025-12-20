use std::env;

#[derive(Debug)]
struct Config {
    db_host: String,
    db_username: String,
    db_password: String,
    db_port: String,
}
impl Config {
    fn init() -> Self {
        Self {
            db_host: env::var("DB_HOST").expect("DB_HOST is not set"),
            db_username: env::var("DB_USERNAME").expect("DB_USERNAME is not set"),
            db_password: env::var("DB_PASSWORD").expect("DB_PASSWORD is not set"),
            db_port: env::var("DB_PORT").expect("DB_PASSWORD is not set"),
        }
    }
}

#[cfg(test)]
mod test_env {
    use crate::_env::Config;
    use dotenvy::dotenv;
    
    #[test]
    fn test_get_from_env() {
        dotenv().ok(); 

        let config = Config::init();
        println!("{:?}", config);
        println!("{}", config.db_host);
        println!("{}", config.db_username);
        println!("{}", config.db_password);
        println!("{}", config.db_port);
    }
}
