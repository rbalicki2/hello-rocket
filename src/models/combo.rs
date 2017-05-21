use app::field_names::HasFieldNames;
use rocket::request::{FromForm, FormItems};
use std::marker::PhantomData;

pub struct Combo<'f, A, B>
  where A: FromForm<'f>, B: FromForm<'f> + HasFieldNames {
  pub first: A,
  pub second: B,
  pub phantom: PhantomData<&'f ()>,
}

impl<'f, A, B> FromForm<'f> for Combo<'f, A, B>
  where A: FromForm<'f>, B: FromForm<'f> + HasFieldNames {
  type Error = ();

  fn from_form_items(form_items: &mut FormItems<'f>) -> Result<Self, Self::Error> {
    let b_names: &'static [&'static str] = B::field_names();

    let b_query_string: String = form_items
      .filter(|p| b_names.contains(&p.0))
      .map(|tuple| format!("{}={}", tuple.0, tuple.1))
      .fold("".to_string(), |acc, x| format!("{}{}", acc, x));

    let b_item: &mut FormItems = &mut FormItems::from(&b_query_string[..]);
    let results_b = B::from_form_items(b_item);

    return Err(());
//
//    match results_a {
//      Ok(a_inner) => {
//        match results_b {
//          Ok(b_inner) => {
//            Ok(Self {
//              first: a_inner,
//              second: b_inner,
//              phantom: PhantomData,
//            })
//          },
//          Err(_) => Err(())
//        }
//      },
//      Err(_) => Err(())
//    }
  }
}

