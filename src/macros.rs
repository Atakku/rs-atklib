// Copyright 2025 Atakku <https://atakku.dev>
//
// This project is dual licensed under MIT and Apache.

/// Most commonly used in in SQLx [`sea_query::query::insert::InsertStatement::values`]
#[macro_export]
macro_rules! into {
  ($($expr:expr),*) => {
    [$(($expr).into()),*]
  };
}

#[macro_export]
macro_rules! mods {
  ($vis:vis use mod $ident:ident; $($pvis:vis $($pident:ident)+;)*) => {
    $crate::mods!($vis mod $ident; $($pvis $($pident)+;)*);
    #[allow(unused_imports)]
    pub use $ident::*;
  };
  ($vis:vis mod $ident:ident; $($pvis:vis $($pident:ident)+;)*) => {
    $vis mod $ident {$($crate::mods!(@sub $pvis $($pident)+);)*}
  };
  (@sub $vis:vis use $name:ident) => {
    $crate::mods!(@sub $vis $name);
    #[allow(unused_imports)]
    pub use self::$name::*;
  };
  (@sub $vis:vis $name:ident) => {
    $vis mod $name;
  };
}
