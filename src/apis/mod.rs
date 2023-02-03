////////////////////////////////////////////////////////////////////////////////
// Copyright (c) 2022-2023 qianjunakasumi <i@qianjunakasumi.ren>               /
//                     qianjunakasumi <qianjunakasumi@outlook.com>             /
//                     https://github.com/qianjunakasumi                       /
//                                                                             /
//     This Source Code Form is subject to the terms of the Mozilla Public     /
//     License, v. 2.0. If a copy of the MPL was not distributed with this     /
//     file, You can obtain one at http://mozilla.org/MPL/2.0/.                /
////////////////////////////////////////////////////////////////////////////////

use {
    async_graphql::{EmptyMutation, EmptySubscription, http::GraphiQLSource, Object, Schema},
    async_graphql_axum::{GraphQLRequest, GraphQLResponse},
    axum::{Extension, response::{Html, IntoResponse}},
};

pub type ConchSchema = Schema<QueryRoot, EmptyMutation, EmptySubscription>;

pub struct QueryRoot;

pub async fn graphql_handler(schema: Extension<ConchSchema>, req: GraphQLRequest)
                             -> GraphQLResponse
{
    schema.execute(req.into_inner()).await.into()
}

pub async fn graphiql() -> impl IntoResponse {
    Html(GraphiQLSource::build().endpoint("/apis").finish())
}


#[Object]
impl QueryRoot {
    async fn hello(&self) -> &str {
        "Hello World!"
    }
}
