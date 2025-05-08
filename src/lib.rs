// Copyright 2025 Atakku <https://atakku.dev>
//
// This project is dual licensed under MIT and Apache.

mod macros;

pub type Err = Box<dyn std::error::Error + Send + Sync + 'static>;
pub type Res<T> = Result<T, Err>;
pub type R = Res<()>;

pub use paste;
