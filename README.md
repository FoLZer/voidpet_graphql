This crate provides graphql schema and convinient rust structs for creating graphql queries to http://api.voidpet.com/ api.

## How To Install:
This crate is not meant to be published on crates.io and only accessible through github.
To Install add this dependency to your Cargo.toml:
```toml
[dependencies]
voidpet_graphql = { git = "https://github.com/FoLZer/voidpet_graphql" }
```

## How To use:
Most of documentation and examples can be found at https://github.com/graphql-rust/graphql-client, this crate allows not to mess with graphql_client structs.

Simple Example:
```rust
use std::error::Error;
use reqwest;
use voidpet_graphql::graphql::{Me2, me2};
use graphql_client::{GraphQLQuery, Response};

async fn perform_me2_query(variables: me2::Variables) -> Result<(), Box<dyn Error>> {

    // this is the important line
    let request_body = Me2::build_query(variables);

    // change ACCESS_COOKIES to your cookies when accessing api.voidpet.com
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(reqwest::header::COOKIE, ACCESS_COOKIES);
    // change USER_AGENT to your user agent when accessing api.voidpet.com
    headers.insert(reqwest::header::USER_AGENT, USER_AGENT);
    let client = reqwest::Client::builder().default_headers(headers).build().unwrap();
    
    let mut res = client.post("https://api.voidpet.com/graphql").json(&request_body).send().await?;
    let response_body: Response<me2::ResponseData> = res.json().await?;
    let response_data = response_body.data?.me2;
    println!("{}", response_data.user.id);
    Ok(())
}
```

## Version
Made for Voidpet version 0.22.0
