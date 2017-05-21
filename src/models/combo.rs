use app::field_names::HasFieldNames;
use rocket::request::{FromForm, FormItems};
use std::marker::PhantomData;

pub struct Combo<'f, A: 'f, B: 'f>
  where A: FromForm<'f>, B: FromForm<'f> + HasFieldNames {
  pub first: A,
  pub second: B,
  phantom: PhantomData<&'f A>,
}

impl<'f, A, B> FromForm<'f> for Combo<'f, A, B>
  where A: FromForm<'f>, B: FromForm<'f> + HasFieldNames {
  type Error = ();

  fn from_form_items(form_items: &mut FormItems<'f>) -> Result<Self, Self::Error> {
//    for i in form_items {
//      println!("{:?}", i);
//    }
//
//    return Err(());

//    println!("{:?}", B::field_names());
    let b_names: &'static [&'static str] = B::field_names();

    let b_string: String = form_items.filter(
      |p| b_names.contains(&p.0)
    )
      .map(|tuple| {
        let mut str = tuple.0.to_owned();
        str.push_str("=");
        str.push_str(tuple.1);
        str
      })
      .fold("".to_string(), |acc, x| {
        let mut str: String = acc.to_owned();
        str.push_str(&x);
        str
      });


//    return Err(());

//    let results_a = A::from_form_items(form_items);
    let b_item: FormItems = FormItems::from(&b_string);
//    let results_b = B::from_form_items(&mut FormItems::from(&b_string));

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

