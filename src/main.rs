extern crate diesel;
extern crate rocket;
extern crate rocket_contrib;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use rocket::response::{status::Created, Debug};
use rocket::serde::{json::Json, Deserialize, Serialize};
use rocket::{get, launch, post, routes};

mod models;
pub mod schema;

use rocket_dyn_templates::{context, Template};
use std::env;
pub fn establish_connection_pg() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

#[derive(Serialize, Deserialize)]
struct NewPost {
    title: String,
    body: String,
}

type Result<T, E = Debug<diesel::result::Error>> = std::result::Result<T, E>;

#[post("/post", format = "json", data = "<post>")]
fn create_post(post: Json<NewPost>) -> Result<Created<Json<NewPost>>> {
    use self::schema::posts::dsl::*;
    use models::Post;
    let mut connection = establish_connection_pg();

    let new_post = Post {
        id: 1,
        title: post.title.to_string(),
        body: post.body.to_string(),
        published: true,
    };

    diesel::insert_into(posts)
        .values(&new_post)
        .execute(&mut connection)
        .expect("Error saving new post");

    Ok(Created::new("/").body(post))
}

#[get("/posts")]
fn index() -> Template {
    use self::models::Post;

    let connection = &mut establish_connection_pg();
    let results = self::schema::posts::dsl::posts
        .load::<Post>(connection)
        .expect("Error loading posts");
    Template::render("posts", context! {posts: &results, count: results.len()})
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![create_post])
        .mount("/", routes![index])
        .attach(Template::fairing())
}
