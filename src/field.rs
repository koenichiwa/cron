use crate::specifier::RootSpecifier;
use crate::error::Error;

#[derive(Debug, PartialEq)]
pub struct Field {
    pub specifiers: Vec<RootSpecifier>, // TODO: expose iterator?
}

pub trait FromField
where
    Self: Sized,
{
    //TODO: Replace with std::convert::TryFrom when stable
    fn from_field(field: Field) -> Result<Self, Error>;
}