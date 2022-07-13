use migration::DbErr;
use sea_orm::EntityTrait;
use seaorm_demo::establish_connection;
use entity::post;

#[tokio::main]
async fn main() -> Result<(), DbErr>{
	let db = establish_connection().await?;

	let posts: Vec<post::Model> = post::Entity::find().all(&db).await?;

	println!("All the posts in db:");
	for post in posts {
		println!("ID: {}, TITLE: {}", post.id, post.title);
	}

	Ok(())
}