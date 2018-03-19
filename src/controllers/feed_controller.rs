use iron::prelude::*;
use iron::status;
//use iron::error::HttpError;
use iron::IronError;
use iron::modifiers::Redirect;

use diesel;
use diesel::prelude::*;
use iron_diesel_middleware::{DieselPooledConnection, DieselReqExt};
use params::{Params, Value};
use db;

use rustc_serialize::json;

use controllers::get_time;





pub fn title_list(req: &mut Request) -> IronResult<Response> {
    let con: DieselPooledConnection<db::MysqlConnection> = req.db_conn();

    use db::schema::title::dsl::*;
    use db::models::Title;

    match title.load::<Title>(&*con){
        Ok(title_body) => {
            let j = json::encode(&title_body).unwrap();
            Ok(Response::with((status::Ok,j)))
        }
        Err(_) => Ok(Response::with((status::Ok,"Error reading DB")))
    }
}



pub fn text_list(req: &mut Request) -> IronResult<Response> {
    let con: DieselPooledConnection<db::MysqlConnection> = req.db_conn();

    use db::schema::text::dsl::*;
    use db::models::Text;

    match text.load::<Text>(&*con){
        Ok(text_body) => {
            let j = json::encode(&text_body).unwrap();
            Ok(Response::with((status::Ok,j)))
        }
        Err(_) => Ok(Response::with((status::Ok,"Error reading DB")))
    }
}


#[derive(RustcEncodable, RustcDecodable)]
struct Post{
    p_title_id:i64,
    p_body:String,
    p_author:String,
}

use controllers::http_controller;
use rustc_serialize::json::ToJson;

pub fn insert(req: &mut Request) -> IronResult<Response> {
    let con: DieselPooledConnection<db::MysqlConnection> = req.db_conn();

    use db::schema::text;

    use db::schema::text::dsl::*;
    use db::models::Text;
    use db::models::NewText;
    use std::io::Read;

    let mut payload = String::new();


    match req.body.read_to_string( & mut payload){
        Err(e) => Ok(Response::with((status::Ok, "read failed"))),
        Ok(n) => {
            match json::decode(&payload) {
                Err(e) => Ok(Response::with((status::Ok, "bind failed"))),
                Ok(post) => {
                    let post:Post = post;
                    let text_body:Vec<Text> = text.load::<Text>(&*con).expect("Error reading DB");

                    let text_list = text_body.iter().filter(| e| {
                        e.title_id == post.p_title_id
                    });

                    let max_sequence = text_list.map(| e| {
                        e.sequence
                    }).max();

                    let new_text = NewText {
                        title_id:post.p_title_id,
                        body:&post.p_body,
                        author:&post.p_author,
                        sequence:max_sequence.map(|e| e+1).unwrap(),
                        created_at:&get_time::get_time(),
                        updated_at:&get_time::get_time(),
                    };
                    match diesel::insert_into(text::table)
                        .values(&new_text)
                        .execute(&*con) {
                        Ok(_) => {
                            Ok(Response::with((status::Ok,"insert success")))
                        },
                        Err(_) => {
                            Ok(Response::with((status::Ok, "insert failed")))
                        }
                    }

                },
            }
        },
    }



}






pub fn insert_title(req: &mut Request) -> IronResult<Response> {
    let con: DieselPooledConnection<db::MysqlConnection> = req.db_conn();

    use db::schema::title;
    use db::models::NewTitle;

    use db::schema::text;
    use db::models::NewText;


    //let params = req.get_ref::<Params>().unwrap();

    match req.get_ref::<Params>() {
        Err(e) => {Ok(Response::with((status::Ok, e.to_string()))) },
        Ok(params) => {
            match params.find(&["name"]){
                _ => {
                    Ok(Response::with((status::NotFound, "not found")))
                },
                Some(&Value::String( ref name)) => {
                    let new_title = NewTitle{
                        name: & name,
                        created_at: & get_time::get_time(),
                        updated_at: & get_time::get_time(),
                    };
                    diesel::insert_into(title::table)
                        .values( & new_title)
                        .execute( & *con)
                        .expect("INSERT failed");
                    Ok(Response::with((status::Ok, "insert")))
                }
            }
        },
    }
}