# Flightroute Microservice

microservice to summarize flight routes.
Heavily optimized for long routes.
Computes in O(n).

## Starting

Starting the (debug-build of the) microservice can be done using `cargo run`.


## Building

can be done using

```
cargo build --release
```

The resulting binary can be found in ./target/release


## Endpoints

The service listens on port `:8080`

### Calculate

Using a `get` request on `:8080/calculate`, where the body is a Vec<(String, String)>.

#### Example

```bash
curl -X GET localhost:8080/calculate \
    -H "Content-Type: application/json" \
    -d '[["IND", "EWR"], ["SFO", "ATL"], ["GSO", "IND"], ["ATL", "GSO"]]'
```

#### Details

The algorithm has a runtime of $ O(n) $.
This behaviour is archieved by utilizing 2 hashmaps, which always store the "next expected step" in the algorithm.

A much more obvious solution would be to
- go through each list element
- select the next fitting flight route
- remove it from the list
- repeate on the now smaller list, until only one element is left

Both a guaranteed to complete (the second one is just iterating through a list, the second one works on ever smaller lists and you can't take away from positive integers indefinitly).
The trivial solution would terminate in $ O(n²) $ time
