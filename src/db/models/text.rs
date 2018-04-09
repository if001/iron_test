use db::schema::text;
use chrono::prelude::*;


//#[derive(Queryable, Serialize, Deserialize, Debug)]
#[derive(Queryable, RustcEncodable, RustcDecodable, Debug)]
pub struct Text {
    pub id: i64,
    pub title_id: i64,
    pub parent_id: i32,
    pub body: String,
    pub author: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable)]
#[table_name="text"]
pub struct NewText<'a> {
    pub title_id: i64,
    pub parent_id: i32,
    pub body: &'a str,
    pub author: &'a str,
    pub created_at: &'a NaiveDateTime,
    pub updated_at: &'a NaiveDateTime,
}
