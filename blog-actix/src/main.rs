use dotenv::dotenv;
use std::env;

fn main() -> std::io::Result<()> {
    // Calling dotenv().ok() sets environment variables based on the contents of the .env file in
    // the current directory and ignores any error that might result.
    dotenv().ok();

    env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let app = blog_actix::Blog::new(8998);
    app.run(database_url)
}