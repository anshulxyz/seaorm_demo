use entity::author;
use migration::DbErr;
use sea_orm::{ActiveModelTrait, Set};
use seaorm_demo::establish_connection;

#[tokio::main]
async fn main() -> Result<(), DbErr> {
    let db = establish_connection().await?;

    let author = author::ActiveModel {
        name: Set(String::from("FirstName LastName")),
        ..Default::default()
    };

    let author: author::Model = author.insert(&db).await?;

    println!("Authour created with ID: {}, NAME: {}", author.id, author.name);

    Ok(())
}
