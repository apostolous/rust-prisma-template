mod prisma;

use prisma::PrismaClient;

async fn create() {
    let client_res = PrismaClient::_builder().build().await;
    let client = client_res.unwrap();

    client
        .pod()
        .create("Hello world!".to_string(), vec![])
        .exec()
        .await;
}

#[tokio::main]
async fn main() -> Result<(), ()> {
    create().await;

    Ok(())
}
