/// A flight from some source to some destination
pub type Flight = (String, String);

/// Input type for the calculate endpoint.
pub type Route = Vec<Flight>;
