use entity::post;
use migration::DbErr;
use sea_orm::{ActiveModelTrait, Set};
use seaorm_demo::establish_connection;

#[tokio::main]
async fn main() -> Result<(), DbErr> {
    let db = establish_connection().await?;

    let post = post::ActiveModel {
        title: Set(String::from("Another title 2")),
        text: Set(String::from("Lorem ipsum dolor sit amet.")),
        ..Default::default()
    };

    let post: post::Model = post.insert(&db).await?;

    println!("Post created with ID: {}, TITLE: {}", post.id, post.title);

    Ok(())
}
