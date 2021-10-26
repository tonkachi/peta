#[cfg(test)]
pub mod test_env_util {
    use std::env;

    pub fn var(name: &str) -> String {
        match env::var(name) {
            Ok(v) => v,
            Err(_) => match name {
                "TAG_DB_NAME" => String::from("tag_test"),
                "TAG_DATABASE_URL" => {
                    String::from("postgres://postgres:postgres@localhost/tag_test")
                }
                _ => panic!("can not resolve {:?}", name),
            },
        }
    }
}
