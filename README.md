# NASA CLI Tool

This is a tool for interacting with the different NASA APIs available. I am using this api to learn more about putting together am app using the rust programming language. Documentation for the API can be found here: [https://api.nasa.gov/](https://api.nasa.gov/)

## How to use the CLI Tool

### NASA Image of the day

Run the following at the root level of the project:

-- date = YYYY-MM-DD

```
cargo run apod --date=2023-07-20
```

### NASA Asteroid - NeoWs

Run the following at the root level of the project:

-- start_date = YYYY-MM-DD
-- end_date = YYYY-MM-DD

```
cargo run asteroids --start-date 2023-08-20 2023-08-21

```
