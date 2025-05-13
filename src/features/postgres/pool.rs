// Copyright 2025 Atakku <https://atakku.dev>
//
// This project is dual licensed under MIT and Apache.

use crate::{get_env, once_cell};
use sqlx::{PgPool, postgres::PgPoolOptions};

once_cell!(async |pub pg: PgPool| {
  PgPoolOptions::default().connect(&get_env!("DATABASE_URL")).await.unwrap()
});

