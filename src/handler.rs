use log::warn;

use warp::reply::{json, Json};

use crate::{logic, model::Route};

pub fn calculate(route: Route) -> Json {
    let flight = logic::calculate(route);
    match flight {
        Ok(flight) => json(&flight),
        Err(err) => {
            // in a production microservice one would specify precisely how to handle errors.
            // At the moment I am coding on an assignment, that does not include strategies for
            // error handling. It often happens, that tasks are a little underspecified, but I
            // think in order to complete the assignment, I can pretend I have talked some strategy
            // over with some team, and we decided on logging and returning the error as json.
            warn!("Invalid flight route {:?}", err);
            json(&err)
        }
    }
}
