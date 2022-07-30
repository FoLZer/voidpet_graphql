This crate provides graphql schema and convinient rust structs for creating graphql queries to http://api.voidpet.com/ api.

## How To use:
Most of documentation and examples can be found at https://github.com/graphql-rust/graphql-client, this crate allows not to mess with graphql_client structs.

Simple Example:
```rust
use graphql_client::{GraphQLQuery, Response};
use std::error::Error;
use reqwest;
use voidpet_graphql::graphql::Me2;
use voidpet_graphql::graphql::me2;

async fn perform_me2_query(variables: me2::Variables) -> Result<(), Box<dyn Error>> {

    // this is the important line
    let request_body = Me2::build_query(variables);

    // change ACCESS_COOKIES to your cookies when accessing api.voidpet.com
    let client = reqwest::Client::builder().default_headers(ACCESS_COOKIES).build().unwrap();
    let mut res = client.post("http://api.voidpet.com/graphql").json(&request_body).send().await?;
    let response_body: Response<me2::ResponseData> = res.json().await?;
    println!("{:#?}", response_body);
    Ok(())
}
```
