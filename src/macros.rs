// Copyright 2025 Atakku <https://atakku.dev>
//
// This project is dual licensed under MIT and Apache.

#[macro_export]
macro_rules! get_env {
  ($env:literal) => {
    std::env::var($env).expect(concat!("Environment variable '", $env, "' is not present").into())
  };
  ($env:literal, $default:literal) => {
    std::env::var($env).unwrap_or($default.into())
  };
}

#[macro_export]
macro_rules! once_cell {
  ($vis:vis async |$name:ident: $ty:ty | $block:block) => {
    $crate::paste::paste! {
      static [<$name:upper>]: tokio::sync::OnceCell<$ty> = tokio::sync::OnceCell::const_new();

      #[allow(unused)]
      $vis async fn [<get_$name>]() -> &'static $ty {
        [<$name:upper>].get_or_init(|| async $block).await
      }
    }
  };
  (|$vis:vis $name:ident: $ty:ty | $block:block) => {
    $crate::paste::paste! {
      static [<$name:upper>]: std::sync::OnceLock<$ty> = std::sync::OnceLock::new();

      #[allow(unused)]
      $vis fn [<get_$name>]() -> &'static $ty {
        [<$name:upper>].get_or_init(|| $block)
      }
    }
  };
  ($vis:vis $name:ident: $ty:ty) => {
    $crate::paste::paste! {
      static [<$name:upper>]: std::sync::OnceLock<$ty> = std::sync::OnceLock::new();

      #[allow(unused)]
      fn [<init_$name>]($name: $ty) {
        let _ = [<$name:upper>].set($name);
      }

      #[allow(unused)]
      $vis fn [<get_$name>]() -> $crate::Res<&'static $ty> {
        Ok([<$name:upper>].get().ok_or(concat!("Called 'get_", stringify!($name), "' before 'init_", stringify!($name), "'"))?)
      }
    }
  };
}

#[macro_export]
macro_rules! use_mod {
  ($($(#[$export:ident])? mod $ident:ident$({$($tt:tt)*})?$(;)?)*) => {$(
    $crate::use_mod!(@internal $(#[$export])? mod $ident$({$($tt)*})?);
  )*};
  (@internal #[no_use] mod $ident:ident {$($tt:tt)*}) => {pub mod $ident {$crate::use_mod!($($tt)*);}};
  (@internal mod $ident:ident {$($tt:tt)*}) => {mod $ident {$crate::use_mod!($($tt)*);} #[allow(unused)] pub use $ident::*;};
  (@internal #[no_use] mod $ident:ident) => {pub mod $ident;};
  (@internal mod $ident:ident) => {mod $ident; #[allow(unused)] pub use $ident::*;};
}

#[macro_export]
macro_rules! into {
  ($($expr:expr),*) => {
    [$(($expr).into()),*]
  };
}

#[macro_export]
macro_rules! if_return {
  ($expr:expr) => {
    $crate::if_return!($expr, {});
  };
  ($expr:expr, $return:expr) => {
    if $expr {
      return $return;
    }
  };
}
