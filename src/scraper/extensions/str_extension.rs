pub struct StrExtension<'a>(pub &'a str);

impl<'a> TryInto<u32> for StrExtension<'a> {
  type Error = ();

  fn try_into(self) -> Result<u32, ()> {
    self
      .0
      .parse::<u32>()
      .or(Err(()))
  }
}