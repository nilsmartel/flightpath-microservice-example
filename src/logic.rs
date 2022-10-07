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
    let mut sources: Map<String, String> = Map::new();

    // keys that can be reached via their value
    let mut destinations: Map<String, String> = Map::new();

    for (mut source, mut dest) in route {
        loop {
            // does some path lead to source?
            if let Some(newsource) = destinations.get(&source.clone()).cloned() {
                // then this path also leads to our destination!

                destinations.remove(&source);
                destinations.insert(dest.clone(), newsource.clone());

                sources.insert(newsource.clone(), dest.clone());
                source = newsource.clone();
                continue;
            }

            // does some path start at our destination?
            if let Some(newdestination) = sources.get(&dest.clone()).cloned() {
                // then we will replace our destination with the newdestination.

                sources.remove(&dest);
                sources.insert(source.clone(), newdestination.clone());

                destinations.insert(newdestination.clone(), source.clone());
                dest = newdestination.clone();
                continue;
            }

            // the path seems to be remote from the ones we've seen before.
            // We add it to our list.

            sources.insert(source.clone(), dest.clone());
            destinations.insert(dest.clone(), source.clone());
            break;
        }
    }

    if sources.len() != 1 {
        return Err(Error::UnclosedPath);
    }

    Ok(sources.into_iter().next().unwrap())
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
