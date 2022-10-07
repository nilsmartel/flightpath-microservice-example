// we alias the HashMap as Map, because there might value in using a different Map implementation.
use std::collections::HashMap as Map;

use crate::model::{Flight, Route};

// #[derive(Debug, Serialize)]
pub enum Error {
    UnclosedPath,
}

/// Compiles all flights into one source and one destination.
/// runs in O(1)
pub fn calculate(route: Route) -> Result<Flight, Error> {
    let mut sources: Map<String, String> = Map::new();

    // keys that can be reached via their value
    let mut destinations: Map<String, String> = Map::new();
    
    for (source, dest) in route {
        loop {
            // does some path lead to source?
            if let Some(newsource) = destinations.get(&source) {
                // then this path also leads to our destination!

                destinations.remove(&source);
                destinations[&dest] = newsource.clone();

                sources[newsource] = dest.clone();
                source = newsource.clone();
                continue;
            }
            // deos some path start at our destination?
            if let Some(newdestination) = sources.get(&dest) {
                // then we will replace our destination with the newdestination.
            
                sources.remove(&dest);
                sources[&source] = newdestination.clone();
                
                destinations[newdestination] = source.clone();
                dest = newdestination.clone();
                continue;
            }

            // the path seems to be remote from the ones we've seen before.
            // We add it to our list.

            sources[&source] = dest.clone();
            destinations[&dest] = source;
        }
    }

    if sources.len() != 1 {
        return Err(Error::UnclosedPath);
    }

    Ok(
        sources.into_iter().next().unwrap()
    )
}