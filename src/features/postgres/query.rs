// Copyright 2025 Atakku <https://atakku.dev>
//
// This project is dual licensed under MIT and Apache.

use crate::Res;
use sea_query::{
  DeleteStatement, InsertStatement, PostgresQueryBuilder, SelectStatement, UpdateStatement,
};
use sea_query_binder::SqlxValues;
use sqlx::{
  FromRow,
  postgres::{PgQueryResult, PgRow},
};

macro_rules! impl_ext {
  ($ident:ident) => {
    paste::paste! {
      #[allow(async_fn_in_trait)]
      #[extend::ext(name = [<$ident Query>])]
      pub impl $ident {
        fn build_sqlx(&self) -> (String, SqlxValues) {
          let (q, v) = self.build(PostgresQueryBuilder);
          (q, SqlxValues(v))
        }

        async fn execute(&self) -> Res<PgQueryResult> {
          let (q, v) = self.build_sqlx();
          Ok(sqlx::query_with(&q, v).execute(crate::get_pg().await).await?)
        }

        async fn fetch_one<T: Send + Unpin + for<'r> FromRow<'r, PgRow>>(&self) -> Res<T> {
          let (q, v) = self.build_sqlx();
          Ok(sqlx::query_as_with(&q, v).fetch_one(crate::get_pg().await).await?)
        }

        async fn fetch_opt<T: Send + Unpin + for<'r> FromRow<'r, PgRow>>(&self) -> Res<Option<T>> {
          let (q, v) = self.build_sqlx();
          Ok(sqlx::query_as_with(&q, v).fetch_optional(crate::get_pg().await).await?)
        }

        async fn fetch_all<T: Send + Unpin + for<'r> FromRow<'r, PgRow>>(&self) -> Res<Vec<T>> {
          let (q, v) = self.build_sqlx();
          Ok(sqlx::query_as_with(&q, v).fetch_all(crate::get_pg().await).await?)
        }
      }
    }
  };
}

impl_ext!(SelectStatement);
impl_ext!(InsertStatement);
impl_ext!(UpdateStatement);
impl_ext!(DeleteStatement);
