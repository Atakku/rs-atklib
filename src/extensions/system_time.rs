// Copyright 2025 Atakku <https://atakku.dev>
//
// This project is dual licensed under MIT and Apache.

use crate::Res;
use std::time::{SystemTime, UNIX_EPOCH};

#[extend::ext]
pub impl SystemTime {
  fn unix_secs() -> Res<u64> {
    Ok(Self::now().duration_since(UNIX_EPOCH).map(|t| t.as_secs())?)
  }
}
