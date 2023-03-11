mod prisma;

use prisma::PrismaClient;
use prisma_client_rust::{NewClientError,chrono::DateTime, chrono::Utc};


#[tokio::main]
async fn main() {
    println!("Hello, world!");
    let client: Result<PrismaClient, NewClientError> = PrismaClient::_builder().build().await;
    let prisma_service = client.unwrap();
    let current_date = DateTime::from(Utc::now());

    let result = prisma_service.user()
    .create( "Salman".into(),"Ahmed".into(),current_date,vec![]).exec().await;
    match result {
        Ok(data) =>{
            println!("Data was added successfully {} {}",data.id, data.first_name)
        },
        Err(query_error) =>{
            panic!("There was a query error {}",query_error.to_string())
        }
    }
}
