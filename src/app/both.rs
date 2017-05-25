use rocket::request::{FromForm, FormItems};

/// This trait is used to get the names of the fields of the structures.
pub trait NamedFields {
  const FIELDS: &'static [&'static str];
}

pub struct QueryParamGroup<T>(pub T);

impl<T> QueryParamGroup<T> {
  pub fn get(&self) -> &T {
    &self.0
  }

  pub fn get_owned(self) -> T {
    self.0
  }
}

macro_rules! combined_params_2 {
  () => {};
  ($($name:ident,)+) => {
    impl<'f, $($name),*> FromForm<'f> for QueryParamGroup<($($name,)*)>
      where $($name: for<'x> FromForm<'x> + NamedFields),*
    {
      type Error = ();
      fn from_form_items(form_items: &mut FormItems<'f>) -> Result<Self, ()> {
        $(
          let query_string_items: Vec<String> = FormItems::from(form_items.inner_str())
            .filter(|&(ref k, _)| $name::FIELDS.contains(k))
            .map(|(k,v)| format!("{}={}", k, v))
            .collect();

          let query_string = query_string_items.join("&");
          let mut items: FormItems = FormItems::from(query_string.as_str());

          #[allow(non_snake_case)]
          let $name = $name::from_form_items(&mut items).map_err(|_| ())?;
        )*
        Ok(QueryParamGroup(($($name,)*)))
      }
    }
  }
}

combined_params_2! { T0, T1, }
