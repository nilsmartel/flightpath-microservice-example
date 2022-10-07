// we alias the HashMap as Map, because there might value in using a different Map implementation.
use std::collections::HashMap as Map;

use crate::model::{Flight, Route};

#[derive(Debug)]
pub enum Error {
    UnclosedPath,
}

/// Compiles all flights into one source and one destination.
/// runs in O(1)
pub fn calculate(route: Route) -> Result<Flight, Error> {
    let mut sources: Map<&String, &String> = Map::new();

    // keys that can be reached via their value
    let mut destinations: Map<&String, &String> = Map::new();

    for flight in route.iter() {
        let mut source = &flight.0;
        let mut dest = &flight.1;

        loop {
            // does some path lead to source?
            if let Some(newsource) = destinations.get(&source.clone()).cloned() {
                // then this path also leads to our destination!

                destinations.remove(&source);
                destinations.insert(dest, newsource);

                sources.insert(newsource, dest);
                source = newsource;
                continue;
            }

            // does some path start at our destination?
            if let Some(newdestination) = sources.get(&dest.clone()).cloned() {
                // then we will replace our destination with the newdestination.

                sources.remove(&dest);
                sources.insert(source, newdestination);

                destinations.insert(newdestination, source);
                dest = newdestination;
                continue;
            }

            // the path seems to be remote from the ones we've seen before.
            // We add it to our list.

            sources.insert(source, dest);
            destinations.insert(dest, source);
            break;
        }
    }

    if sources.len() != 1 {
        return Err(Error::UnclosedPath);
    }

    Ok(sources.into_iter().next().map(|(a, b)| (a.clone(), b.clone())).unwrap())
}

#[cfg(test)]
mod route_tests {
    use super::*;

    #[test]
    fn correctness_tests() {
        let route = "df ay ec ye cd"
            .split(' ')
            .map(|s| {
                let mut s = s.chars();
                let start = String::from(s.next().unwrap());
                let end = String::from(s.next().unwrap());
                (start, end)
            })
            .collect::<Route>();

        assert_eq!(
            calculate(route).unwrap(),
            ("a".to_string(), "f".to_string())
        );
    }
}
