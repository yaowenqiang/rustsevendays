#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_codegen;
extern crate dotenv;
use diesel::prelude::*;
use diesel::pg::PgConnection;

mod schema {
    infer_schema!("dotenv:DATABASE_URL");
}
use schema::*;

#[derive(Debug, Queryable, Identifiable, Associations, AsChangeset)]
#[has_many(photos)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub avatar: Option<String>,
}


#[derive(Debug, Queryable, Identifiable, Associations, AsChangeset)]
#[has_many(photos)]
pub struct Photo {
    pub id: i32,
    pub user_id: String,
    pub avatar: Option<String>,
}


fn main() {
    let me = users::table.find(1).first::<User>(&conn).expect("Error loading user");
    println!("{}", me);
    let my_photos = Photo::belonging_to(&me)
        .load::<Photo>(&conn)
        .expect("Error loading photos");
    println("{:?}", my_photos);

}
