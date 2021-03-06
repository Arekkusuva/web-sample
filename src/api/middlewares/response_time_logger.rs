use iron::prelude::*;
use iron::{status, typemap, BeforeMiddleware, AfterMiddleware};
use time::precise_time_ns;

use api::middlewares::logger::LoggerReqExt;

pub struct ResponseTimeLoggerMiddleware;

impl typemap::Key for ResponseTimeLoggerMiddleware { type Value = u64; }

impl BeforeMiddleware for ResponseTimeLoggerMiddleware {
    fn before(&self, req: &mut Request) -> IronResult<()> {
        req.extensions.insert::<ResponseTimeLoggerMiddleware>(precise_time_ns());
        Ok(())
    }
}

impl AfterMiddleware for ResponseTimeLoggerMiddleware {
    fn after(&self, req: &mut Request, res: Response) -> IronResult<Response> {
        let delta = precise_time_ns() - *req.extensions.get::<ResponseTimeLoggerMiddleware>().unwrap();
        let logger = req.get_logger();
        info!(logger, "Request with response status: {}, took: {}", res.status.unwrap_or(status::NotFound), delta as f64 / 1000000.0);
        Ok(res)
    }
}
