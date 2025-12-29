pub struct Envs;

impl Envs {
    const HOST_ADDRESS: &str = "HOST_ADDRESS";
    const DATABASE_URL: &str = "DATABASE_URL";
    const APP_ENV: &str = "APP_ENV";

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

    pub fn app_env() -> String {
        let expect = format!("{} must be set", Self::APP_ENV);
        std::env::var(Self::APP_ENV).expect(&expect).to_string()
    }
}
