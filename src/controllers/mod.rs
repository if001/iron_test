pub mod feed_controller;
mod get_time;

use iron::prelude::*;
use iron::status;
use mount::Mount;
use router::Router;

pub fn router() -> Mount {
    let router = router!(
        index:      get   "/"               => index,
        index_q:    get   "/:name"          => index,
        insert:     post  "/insert/"        => feed_controller::insert,
        list_user:  get   "/list/"          => feed_controller::list
    );

    let mut mount = Mount::new();
    mount.mount("/", router);

    mount
}

fn index(req: &mut Request) -> IronResult<Response> {
    let ref title = req.extensions
        .get::<Router>()
        .unwrap()
        .find("title")
        .unwrap_or("hoge")
        .to_owned();

    Ok(Response::with((status::Ok,"ok")))

}