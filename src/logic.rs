use serde_derive::Serialize;

use crate::model::{Flight, Route};

#[derive(Debug, Serialize)]
pub enum Error {}

pub fn calculate(route: Route) -> Result<Flight, Error> {
    unimplemented!()
}
