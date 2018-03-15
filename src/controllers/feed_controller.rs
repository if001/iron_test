use iron::prelude::*;
use iron::status;
use iron::modifiers::Redirect;

use diesel;
use diesel::prelude::*;
use iron_diesel_middleware::{DieselPooledConnection, DieselReqExt};
use params::{Params, Value};
use db;

use rustc_serialize::json;

use controllers::get_time;



pub fn list(req: &mut Request) -> IronResult<Response> {
    let con: DieselPooledConnection<db::MysqlConnection> = req.db_conn();

//    use db::schema::users::dsl::*;
//    use db::models::User;
//    let results = users.load::<User>(&*con).expect("Error reading DB");

    use db::schema::title::dsl::*;
    use db::models::Title;

    use db::schema::text::dsl::*;
    use db::models::Text;


    let title_body = title.load::<Title>(&*con).expect("Error reading DB");

    let j_1 = json::encode(&title_body[0]).unwrap();

    let j_2:db::models::Title = json::decode(&j_1).unwrap();
    println!("{}",j_2.name);

//    let text_body = text
//        .load::<Text>(&*con).expect("Error reading DB");
//    let j_2 = json::encode(&text_body).unwrap();


    Ok(Response::with((status::Ok,j_1)))
}

pub fn insert(req: &mut Request) -> IronResult<Response> {
    let con: DieselPooledConnection<db::MysqlConnection> = req.db_conn();

    use db::schema::text;
    use db::schema::title;
    use db::models::NewText;
    use db::models::NewTitle;

    let params = req.get_ref::<Params>().unwrap();

    match params.find(&["name"]) {
        Some(&Value::String(ref name)) => {
            let new_title = NewTitle{
                name: &name,
                created_at:&get_time::get_time(),
                updated_at:&get_time::get_time(),
            };
            diesel::insert_into(title::table)
                .values(&new_title)
                .execute(&*con)
                .expect("INSERT failed");
            Ok(Response::with((status::Ok,"insert")))
        }
        _ => {
            Ok(Response::with((status::NotFound,"not found")))
        }
    }

}