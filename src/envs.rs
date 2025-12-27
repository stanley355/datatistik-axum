pub struct Envs;

impl Envs {
    const HOST_ADDRESS: &str = "HOST_ADDRESS";
    const DATABASE_URL: &str = "DATABASE_URL";

    pub fn host_address() -> String {
        let expect = format!("{} must be set", Self::HOST_ADDRESS);
        std::env::var(Self::HOST_ADDRESS)
            .expect(&expect)
            .to_string()
    }

    pub fn database_url() -> String {
        let expect = format!("{} must be set", Self::DATABASE_URL);
        std::env::var(Self::DATABASE_URL)
            .expect(&expect)
            .to_string()
    }
}
