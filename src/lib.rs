use serde::{Deserialize, Serialize};
use worker::*;

#[derive(Debug, Serialize, Deserialize)]
struct CustomResp {
    pub greeting: String,
    pub name: String,
}

#[event(fetch)]
pub async fn main(req: Request, env: Env, _ctx: worker::Context) -> Result<Response> {
    let router = Router::new();

    router.get("/", home).run(req, env).await
}

fn home(
    _: worker::Request,
    _: RouteContext<()>,
) -> std::result::Result<worker::Response, worker::Error> {
    let default_resp = CustomResp {
        greeting: "Hello".to_string(),
        name: "Nate".to_string(),
    };
    return Response::from_json(&default_resp);
}
