#[derive(FromForm)]
pub struct LimitOffsetParam {
  pub limit: Option<u32>,
  pub offset: Option<u32>,
}
