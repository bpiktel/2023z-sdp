use derive_more::{Deref, DerefMut};
use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

/// Wrapper to store database identifiers.
/// By default the identifiers use the [`Thing`] type, which store arbitrary IDs,
/// but it can be downcasted to a string type through [`TryIntoStringId::try_into_string_id`].
#[derive(Debug, Serialize, Deserialize, PartialEq, Deref, DerefMut, Clone)]
pub struct Identified<T = (), Id = Thing> {
    pub id: Id,
    #[serde(flatten)]
    #[deref]
    #[deref_mut]
    pub data: T,
}

impl<T, Id> Identified<T, Id> {
    pub fn new(id: Id, data: T) -> Self {
        Self { id, data }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum IdConversionError {
    #[error("Id is not string")]
    IdIsNotString,
}

pub trait TryIntoStringId {
    type Output;

    fn try_into_string_id(self) -> Result<Self::Output, IdConversionError>;
}

impl TryIntoStringId for Thing {
    type Output = String;

    fn try_into_string_id(self) -> Result<Self::Output, IdConversionError> {
        match self.id {
            surrealdb::sql::Id::String(id) => Ok(id),
            _ => Err(IdConversionError::IdIsNotString),
        }
    }
}

impl<T> TryIntoStringId for Identified<T, Thing> {
    type Output = Identified<T, String>;

    fn try_into_string_id(self) -> Result<Self::Output, IdConversionError> {
        let id = self.id.try_into_string_id()?;
        Ok(Identified::new(id, self.data))
    }
}

impl<T> TryIntoStringId for Vec<Identified<T, Thing>> {
    type Output = Vec<Identified<T, String>>;

    fn try_into_string_id(self) -> Result<Self::Output, IdConversionError> {
        self.into_iter().map(|x| x.try_into_string_id()).collect()
    }
}

pub type StringIdentified<T = ()> = Identified<T, String>;
