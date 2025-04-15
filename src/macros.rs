// Copyright 2025 Atakku <https://atakku.dev>
//
// This project is dual licensed under MIT and Apache.

/// A helper macro, most commonly used in in SQLx [`sea_query::query::insert::InsertStatement::values`]
#[macro_export]
macro_rules! into {
  ($($expr:expr),*) => {
    [$(($expr).into()),*]
  };
}