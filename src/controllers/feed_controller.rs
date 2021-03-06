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


//todo serviceとかにまとめる
pub fn response_json(status:i32, content:&str) -> String{
    [r#"{"status":"#, status.to_string().as_str(), r#","content":"#, content, "}" ].concat()
}


pub fn title_list(req: &mut Request) -> IronResult<Response> {
    let con: DieselPooledConnection<db::MysqlConnection> = req.db_conn();

    use db::schema::title::dsl::*;
    use db::models::Title;

    match title.load::<Title>(&*con){
        Ok(title_body) => {
            let j = json::encode(&title_body).unwrap();
            Ok(Response::with((status::Ok,response_json(200, &j))))
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
            Ok(Response::with((status::Ok,response_json(200, &j))))
        }
        Err(_) => Ok(Response::with((status::Ok,response_json(200,"Error reading DB"))))
    }

}


#[derive(RustcEncodable, RustcDecodable)]
struct Post{
    p_title_id:i64,
    parent_id:i32,
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
                    let new_text = NewText {
                        title_id:post.p_title_id,
                        body:&post.p_body,
                        author:&post.p_author,
                        parent_id:post.parent_id,
                        created_at:&get_time::get_time(),
                        updated_at:&get_time::get_time(),
                    };
                    match diesel::insert_into(text::table)
                        .values(&new_text)
                        .execute(&*con) {
                        Ok(_) => {
                            Ok(Response::with((status::Ok,response_json(200,"insert success"))))
                        },
                        Err(e) => {
                            Ok(Response::with((status::Ok,response_json(200, &e.to_string()))))
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


    match req.get_ref::<Params>() {
        Err(e) => {Ok(Response::with((status::Ok, response_json(200,&e.to_string())))) },
        Ok(params) => {
            match params.find(&["name"]){
                _ => {
                    Ok(Response::with((status::NotFound, response_json(200,"not found"))))
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
                    Ok(Response::with((status::Ok, response_json(200,"insert"))))
                }
            }
        },
    }
}