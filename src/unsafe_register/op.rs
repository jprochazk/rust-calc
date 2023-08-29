macro_rules! newtype_enum {
  ($vis:vis enum $name:ident {
    $($variant:ident {
      $($field:ident : $ty:ty),*
      $(,)?
    }),*
    $(,)?
  }) => {
    #[repr(u8)]
    $vis enum $name {
      $($variant($variant)),*
    }

    $(
      #[repr(packed)]
      $vis struct $variant {
        $(pub $field : $ty),*
      }
    )*

    $(
      #[allow(non_snake_case)]
      $vis fn $variant($($field : $ty),*) -> $name {
        $name::$variant($variant { $($field),* })
      }
    )*
  }
}

newtype_enum! {
  pub enum Op {
    LInt { dst: u8, val: i16 },
    LConst { dst: u8, idx: u16 },
    BAdd { dst: u8, lhs: u8, rhs: u8 },
    BSub { dst: u8, lhs: u8, rhs: u8 },
    BMul { dst: u8, lhs: u8, rhs: u8 },
    BDiv { dst: u8, lhs: u8, rhs: u8 },
    UMinus { dst: u8, rhs: u8 },
  }
}

const _: () = {
    let _ = std::mem::transmute::<Op, u32>;
};
