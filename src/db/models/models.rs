//use db::schema::posts;
//use chrono::prelude::*;
//
//
////#[derive(Queryable, Serialize, Deserialize, Debug)]
//#[derive(Queryable, RustcEncodable, RustcDecodable, Debug)]
//pub struct Posts {
//    pub id: i32,
//    pub title: String,
//    pub body: String,
//    pub created_at: NaiveDateTime,
//    pub updated_at: NaiveDateTime,
//}
//
//#[derive(Insertable)]
//#[table_name="posts"]
//pub struct NewPosts<'a> {
//    pub title: &'a str,
//    pub body: &'a str,
//    pub created_at: &'a NaiveDateTime,
//    pub updated_at: &'a NaiveDateTime,
//}
