use iron::prelude::*;
use iron::status;
use iron_auth::AuthWrapper;
use iron_auth::AuthReqExt;

use api::transport::prelude::*;
use api::Router;
use api::transport::users::PostUser;

fn post_user(req: &mut Request) -> IronResult<Response> {
    let body: PostUser = match req.parse_body() {
        Ok(body) => body,
        Err(err) => return Ok(err.into()),
    };
    Ok(Response::with((status::Ok, body.email)))
}

fn get_users(req: &mut Request) -> IronResult<Response> {
    let s = req.session();
    println!("{:?}", s);
    Ok(Response::with((status::Ok, "get users")))
}

pub fn engage(router: &mut Router) {
    router.post("users", post_user);
    router.get("users", AuthWrapper::wrap(get_users));
}
