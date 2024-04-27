use zero2prod2::run;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    run()?.await
}
