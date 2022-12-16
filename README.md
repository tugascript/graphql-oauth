# GraphQL Local OAuth

<small>
<b>NOTE</b>: This is a work in progress and is not yet ready for production use, use at your own risk.
</small>

## Description

This is a simple implementation of local [OAuth2.0](https://oauth.net/2/) authentication with [GraphQL](https://graphql.org/) and [GraphQL Federation](https://www.apollographql.com/docs/federation/).

## Technologies

This project is purely written with safe [rust](https://www.rust-lang.org/) and uses the following frameworks and crates:

- [actix-web](https://actix.rs/) for the web server;
- [async-graphql](https://async-graphql.github.io/async-graphql/en/index.html) for the GraphQL adapted;
- [sea-orm](https://www.sea-ql.org/SeaORM/) for the database ORM.
