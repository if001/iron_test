use db::schema::title;
use chrono::prelude::*;


//#[derive(Queryable, Serialize, Deserialize, Debug)]
#[derive(Queryable, RustcEncodable, RustcDecodable, Debug)]
pub struct Title {
    pub id: i64,
    pub name: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable)]
#[table_name="title"]
pub struct NewTitle<'a> {
    pub name: &'a str,
    pub created_at: &'a NaiveDateTime,
    pub updated_at: &'a NaiveDateTime,
}
