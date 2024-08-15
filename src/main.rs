use Newsletter::run;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    run(std::net::TcpListener)?.await
}
