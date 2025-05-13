// Copyright 2025 Atakku <https://atakku.dev>
//
// This project is dual licensed under MIT and Apache.

#[macro_export]
macro_rules! schema {
  ($(
    $(#[$($table_tt:tt)*])*
    enum $table:ident {$(
      $(#[$($col_tt:tt)*])* $col:ident
    ),*}
  )*) => {
    $(
      #[derive(sea_query::Iden)]
      pub enum $table {
        Table, $($col),*
      }

      impl $table {
        #[inline]
        pub fn col(self) -> (Self, Self) {
          (Self::Table, self)
        }

        #[inline]
        pub fn expr(self) -> sea_query::SimpleExpr {
          sea_query::Expr::col(self.col()).into()
        }
      }
    )*

    pub async fn init_tables() -> $crate::R {
      $(
        sqlx::query_with(&sea_query::Table::create()
        .if_not_exists()
        .table($table::Table)
      $(.$($table_tt)*)*
      $(.col(sea_query::ColumnDef::new($table::$col)$(.$($col_tt)*)*)
      )*
      .build(sea_query::PostgresQueryBuilder), sqlx::postgres::PgArguments::default())
      .execute($crate::get_pg().await).await?;
      )*
      Ok(())
    }
  };
}
