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
    async_graphql::{EmptySubscription, http::GraphiQLSource, Object, Schema},
    async_graphql_axum::{GraphQLRequest, GraphQLResponse},
    axum::{Extension, response::{Html, IntoResponse}},
    self::dashboard::MutDashboard,
};

mod dashboard;

pub type ConchSchema = Schema<QueryRoot, MutationRoot, EmptySubscription>;

pub struct QueryRoot;

pub struct MutationRoot;

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
    async fn greeting(&self) -> &str { "Hello welcome from Conch QueryRoot!" }
}

#[Object]
impl MutationRoot {
    async fn greeting(&self) -> &str { "Hello welcome from Conch MutationRoot!" }

    async fn dashboard(&self) -> MutDashboard { MutDashboard }
}
