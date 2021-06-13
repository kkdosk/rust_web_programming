use actix_web::web;
mod create;
use super::path::Path;

pub fn item_factory(app: &mut web::ServiceConfig) {
  println!("item_factory");
  let base_path: Path = Path{prefix: String::from("/item")};

  app.route(&base_path.define(String::from("/create/{title}")), web::post().to(create::create));
}