use app::both;

#[derive(FromForm)]
pub struct LimitOffsetParam {
  pub limit: Option<u32>,
  pub offset: Option<u32>,
}

impl both::NamedFields for LimitOffsetParam {
  const FIELDS: &'static [&'static str] = &["limit", "offset"];
}
