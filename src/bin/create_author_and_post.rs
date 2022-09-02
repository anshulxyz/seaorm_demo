use entity::{author, post};
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

    let post = post::ActiveModel {
        title: Set(String::from("Another title 2")),
        text: Set(String::from("Lorem ipsum dolor sit amet.")),
        author_id: Set(author.id),
        ..Default::default()
    };

    let post: post::Model = post.insert(&db).await?;

    println!(
        "Post created with ID: {}, TITLE: {}, AUTHOR ID: {}",
        post.id, post.title, post.author_id
    );

    Ok(())
}
