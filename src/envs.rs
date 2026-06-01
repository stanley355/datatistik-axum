pub struct Envs;

impl Envs {
    const HOST_ADDRESS: &str = "HOST_ADDRESS";
    const DATABASE_URL: &str = "DATABASE_URL";
    const APP_ENV: &str = "APP_ENV";
    const TRUSTED_DOMAINS: &str = "TRUSTED_DOMAINS";
    const BETTER_AUTH_URL: &str = "BETTER_AUTH_URL";
    const S3_ENDPOINT: &str = "S3_ENDPOINT";
    const S3_BUCKET: &str = "S3_BUCKET";
    const S3_ACCESS_KEY: &str = "S3_ACCESS_KEY";
    const S3_SECRET_KEY: &str = "S3_SECRET_KEY";

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

    pub fn trusted_domains() -> String {
        let expect = format!("{} must be set", Self::TRUSTED_DOMAINS);
        std::env::var(Self::TRUSTED_DOMAINS)
            .expect(&expect)
            .to_string()
    }

    pub fn better_auth_url() -> String {
        let expect = format!("{} must be set", Self::BETTER_AUTH_URL);
        std::env::var(Self::BETTER_AUTH_URL)
            .expect(&expect)
            .to_string()
    }

    pub fn s3_endpoint() -> String {
        let name = Self::S3_ENDPOINT;
        let expect = format!("{} must be set", name);
        std::env::var(name).expect(&expect).to_string()
    }
    pub fn s3_bucket() -> String {
        let name = Self::S3_BUCKET;
        let expect = format!("{} must be set", name);
        std::env::var(name).expect(&expect).to_string()
    }

    pub fn s3_access_key() -> String {
        let name = Self::S3_ACCESS_KEY;
        let expect = format!("{} must be set", name);
        std::env::var(name).expect(&expect).to_string()
    }

    pub fn s3_secret_key() -> String {
        let name = Self::S3_SECRET_KEY;
        let expect = format!("{} must be set", name);
        std::env::var(name).expect(&expect).to_string()
    }
}
