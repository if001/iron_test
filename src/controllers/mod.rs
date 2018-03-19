pub mod feed_controller;
mod get_time;
mod http_controller;

use iron::prelude::*;
use iron::status;
use mount::Mount;
use router::Router;

pub fn router() -> Mount {
    let router = router!(
        index:      get   "/"               => index,
        index_q:    get   "/:name"          => index,
        insert:     post  "/insert/"        => feed_controller::insert,
        title_list: get   "/title_list/"    => feed_controller::title_list,
        text_list:  get   "/text_list/"     => feed_controller::text_list
    );
    //router.post("/set", move |r| set_greeting(r, &mut greeting_clone.lock().unwrap()));

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