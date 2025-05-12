// Copyright 2025 Atakku <https://atakku.dev>
//
// This project is dual licensed under MIT and Apache.

macro_rules! gen_impl {
  ($ident:ident<$($g:ident),*>) => {
    paste::paste! {
      #[extend::ext(name = [<$ident Ext>])]
      #[allow(unused_parens)]
      pub impl<$($g),*> $ident<$($g),*> {
        fn collect<R: FromIterator<($($g),*)>>(self) -> R {
          self.into_iter().collect()
        }
      
        fn map<R: FromIterator<RT>, RT>(self, f: impl FnMut(($($g),*)) -> RT) -> R {
          self.into_iter().map(f).collect()
        }
      
        fn filter<R: FromIterator<($($g),*)>>(self, f: impl FnMut(&($($g),*)) -> bool) -> R {
          self.into_iter().filter(f).collect()
        }
      
        fn filter_map<R: FromIterator<RT>, RT>(self, f: impl FnMut(($($g),*)) -> Option<RT>) -> R {
          self.into_iter().filter_map(f).collect()
        }
      }
    }
  };
}

use std::collections::{HashMap, HashSet};

gen_impl!(HashMap<K, V>);
gen_impl!(HashSet<K>);
gen_impl!(Vec<V>);