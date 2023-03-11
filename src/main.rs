mod prisma;

use prisma::PrismaClient;
use prisma::{property, user};
use prisma_client_rust::{chrono::DateTime, chrono::Utc, NewClientError};

#[tokio::main]
async fn main() {
    println!("Hello, world!");
    let client: Result<PrismaClient, NewClientError> = PrismaClient::_builder().build().await;
    let prisma_service = client.unwrap();
    let current_date = DateTime::from(Utc::now());

    let result = prisma_service
        .user()
        .create("Salman".into(), "Ahmed".into(), current_date, vec![])
        .exec()
        .await
        .unwrap();

    let second_query = prisma_service
        .property()
        .create(
            user::id::equals(result.id),
            vec![
                property::name::set(Some(String::from("151--d"))),
                property::area::set(51.35.into()),
            ],
        )
        .exec()
        .await;

    match second_query {
        Ok(res) => {
            println!("Query successfully generated {}",res.id);
        }
        Err(err) => {
            println!("Error {}",err.to_string())
        }
    }
    // let second_query = prisma_service
    //     .user()
    //     .create("Naseer".into(), "Ahmad".into(), current_date, vec![])
    //     .exec()
    //     .await;

    // match second_query {
    //     Ok(dt) => {

    //     },
    //     Err(q) =>{

    //     }
    // }
}
