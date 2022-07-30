use graphql_client::*;

#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "src/queries/schema.graphql",
	query_path = "src/queries/queries.graphql"
)]
pub struct AdminGiveItems;