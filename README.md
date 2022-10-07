# Flightroute Microservice

microservice to summarize flight routes.

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
