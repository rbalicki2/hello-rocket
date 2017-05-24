use rocket::request::{FromForm, FormItems};

/// This trait is used to get the names of the fields of the structures.
pub trait NamedFields {
  const FIELDS: &'static [&'static str];
}

pub struct Both<A, B>(pub A, pub B);

impl<'f, A, B> FromForm<'f> for Both<A, B>
  where A: for<'x> FromForm<'x> + NamedFields,
        B: for<'x> FromForm<'x> + NamedFields,
{
  type Error = ();

  fn from_form_items(form_items: &mut FormItems<'f>) -> Result<Both<A, B>, ()> {
    let first_query_items: Vec<_> = form_items
      .filter(|&(ref k, _)| A::FIELDS.contains(k))
      .map(|(k, v)| format!("{}={}", k, v))
      .collect();

    let second_query_items: Vec<_> = FormItems::from(form_items.inner_str())
      .filter(|&(ref k, _)| B::FIELDS.contains(k))
      .map(|(k, v)| format!("{}={}", k, v))
      .collect();

    let first_query_string = first_query_items.join("&");
    let second_query_string = second_query_items.join("&");
    let mut first_items = FormItems::from(first_query_string.as_str());
    let mut second_items = FormItems::from(second_query_string.as_str());

    let a = A::from_form_items(&mut first_items).map_err(|_| ())?;
    let b = B::from_form_items(&mut second_items).map_err(|_| ())?;
    Ok(Both(a, b))
  }
}

macro_rules! combined_params {
  () => ();
  ($struct_name:ident; $($name:ident,)+) => {
    pub struct $struct_name<$($name,)*> ($(pub $name,)*);
    impl<'f, $($name),*> FromForm<'f> for $struct_name<$($name,)*>
      where $($name: for<'x> FromForm<'x> + NamedFields),*
    {
      type Error = ();

      fn from_form_items(form_items: &mut FormItems<'f>) -> Result<Self, ()> {
        let a = $struct_name($({
          {
            let query_string_items: Vec<String> = FormItems::from(form_items.inner_str())
            .filter(|&(ref k, _)| $name::FIELDS.contains(k))
            .map(|(k, v)| format!("{}={}", k, v))
            .collect();

            let query_string = query_string_items.join("&");
            let mut items: FormItems = FormItems::from(query_string.as_str());

            $name::from_form_items(&mut items).map_err(|_| ())?
          }
        }),*);
        Ok(a)
      }
    }
  };
}

combined_params! { MyStruct1; T0, T1, }
//combined_params! { T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, }
