// Copyright 2025 Atakku <https://atakku.dev>
//
// This project is dual licensed under MIT and Apache.


mod macros;

pub type Err = Box<dyn std::error::Error + Send + Sync + 'static>;
pub type Res<T> = Result<T, Err>;
pub type R = Res<()>;

pub use paste;

use_mod!(
  mod extensions {
    mod iteratable;
    mod system_time;
  }
);

// TODO: Allow use_mod macro to parse attributes
mod features {
  #[cfg(feature = "postgres")]
  crate::use_mod!(
    mod postgres {
      mod pool;
      mod query;
      mod schema;
    }
  );
}
#[allow(unused)]
pub use features::*;
