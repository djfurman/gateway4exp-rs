use gateway4exp::run;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    run()?.await
}
