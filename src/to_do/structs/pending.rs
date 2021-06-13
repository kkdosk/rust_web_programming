use super::base::Base;
use super::traits::create::Create;
use super::traits::edit::Edit;
use super::traits::get::Get;
use super::traits::delete::Delete;

pub struct Pending {
  pub super_struct: Base
}

impl Pending {
  pub fn new(input_title: String) -> Pending {
    let input_status: String = String::from("pending");
    let base:Base = Base::new(input_title, input_status);
    return Pending{super_struct: base}
  }
}

impl Create for Pending {}
impl Edit for Pending {}
impl Get for Pending {}
impl Delete for Pending {}