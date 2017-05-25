use app::both::NamedFields;

#[derive(FromForm)]
pub struct LimitOffsetParam {
  pub limit: Option<u32>,
  pub offset: Option<u32>,
}

impl NamedFields for LimitOffsetParam {
  const FIELDS: &'static [&'static str] = &["limit", "offset"];
}
