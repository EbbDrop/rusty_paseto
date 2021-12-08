use super::PasetoClaim;
use serde::ser::SerializeMap;

#[derive(Clone)]
pub struct TokenIdentifierClaim<'a>((&'a str, &'a str));
impl<'a> PasetoClaim for TokenIdentifierClaim<'a> {
  fn get_key(&self) -> &str {
    self.0 .0
  }
}

impl<'a> Default for TokenIdentifierClaim<'a> {
  fn default() -> Self {
    Self(("jti", ""))
  }
}

//created using the From trait
impl<'a> From<&'a str> for TokenIdentifierClaim<'a> {
  fn from(s: &'a str) -> Self {
    Self(("jti", s))
  }
}

//want to receive a reference as a tuple
impl<'a> AsRef<(&'a str, &'a str)> for TokenIdentifierClaim<'a> {
  fn as_ref(&self) -> &(&'a str, &'a str) {
    &self.0
  }
}

impl<'a> serde::Serialize for TokenIdentifierClaim<'a> {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: serde::Serializer,
  {
    let mut map = serializer.serialize_map(Some(2))?;
    map.serialize_key(&self.0 .0)?;
    map.serialize_value(&self.0 .1)?;
    //map.serialize_entry(self.0 .0, self.0 .1)?;
    map.end()
  }
}