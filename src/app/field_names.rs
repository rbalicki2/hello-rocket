pub trait HasFieldNames {
  fn field_names() -> &'static [&'static str];
}

#[macro_export]
macro_rules! add_field_names {
  (#[derive(FromForm)] pub struct $name:ident { $(pub $fname:ident : $ftype:ty),*, }) => {

    #[derive(FromForm)]
    pub struct $name {
      $(pub $fname : $ftype),*
    }

    impl HasFieldNames for $name {
      fn field_names() -> &'static [&'static str] {
        static NAMES: &'static [&'static str] = &[$(stringify!($fname)),*];
        NAMES
      }
    }
  };
}

