mod envs;

fn main() {
    dotenvy::dotenv().ok();
    println!("Hello, world!");
}
