use super::*;
use serde::*;
use validator::{Validate, ValidationError, ValidationErrors};

pub type Probability = f32;

//////////////////////////////////////////////////////////////////////////
/// WeightedList

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum WeightedList<T: fmt::Debug + Clone> {
    Single(T),
    List(Vec<Weighted<T>>),
}
impl<T: fmt::Debug + Clone> Default for WeightedList<T> {
    fn default() -> Self {
        Self::List(Vec::new())
    }
}
impl<T: fmt::Debug + Clone> WeightedList<T> {
    pub fn len(&self) -> usize {
        match self {
            WeightedList::Single(_) => 1,
            WeightedList::List(vec) => vec.len(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn try_for_each<E, F: FnMut(&T) -> Result<(), E>>(&self, mut f: F) -> Result<(), E> {
        match self {
            WeightedList::Single(v) => f(v),
            WeightedList::List(vec) => vec
                .iter()
                .map(|v| match v {
                    Weighted::Weighted { item, weight: _ } => item,
                    Weighted::Unweighted(item) => item,
                })
                .try_for_each(f),
        }
    }

    pub fn map<F, S>(&self, mut map: F) -> WeightedList<S>
    where
        F: FnMut(&T) -> S,
        S: fmt::Debug + Clone,
    {
        match self {
            WeightedList::Single(v) => WeightedList::Single(map(v)),
            WeightedList::List(vec) => {
                let mut out = Vec::<Weighted<S>>::with_capacity(vec.len());
                for v in vec {
                    out.push(match v {
                        Weighted::Weighted { item, weight } => Weighted::Weighted {
                            item: map(item),
                            weight: *weight,
                        },
                        Weighted::Unweighted(item) => Weighted::Unweighted(map(item)),
                    });
                }
                WeightedList::List(out)
            }
        }
    }

    pub fn filter<F>(&self, mut filter: F) -> Option<WeightedList<T>>
    where
        F: FnMut(&T) -> bool,
    {
        match self {
            WeightedList::Single(v) => {
                if filter(v) {
                    Some(self.clone())
                } else {
                    None
                }
            }
            WeightedList::List(vec) => {
                let mut out = Vec::<Weighted<T>>::with_capacity(vec.len());
                for v in vec {
                    if filter(v.item()) {
                        out.push(v.clone());
                    }
                }
                if out.is_empty() {
                    None
                } else {
                    Some(WeightedList::List(out))
                }
            }
        }
    }

    pub fn try_filter<F, E>(&self, mut filter: F) -> Result<Option<WeightedList<T>>, E>
    where
        F: FnMut(&T) -> Result<bool, E>,
    {
        match self {
            WeightedList::Single(v) => {
                if filter(v)? {
                    Ok(Some(self.clone()))
                } else {
                    Ok(None)
                }
            }
            WeightedList::List(vec) => {
                let mut out = Vec::<Weighted<T>>::with_capacity(vec.len());
                for v in vec {
                    if filter(v.item())? {
                        out.push(v.clone());
                    }
                }
                if out.is_empty() {
                    Ok(None)
                } else {
                    Ok(Some(WeightedList::List(out)))
                }
            }
        }
    }
    pub fn try_filter_map<F, S, E>(&self, mut filter: F) -> Result<Option<WeightedList<S>>, E>
    where
        F: FnMut(&T) -> Result<Option<S>, E>,
        S: fmt::Debug + Clone,
    {
        match self {
            WeightedList::Single(v) => {
                if let Some(item) = filter(v)? {
                    Ok(Some(WeightedList::Single(item)))
                } else {
                    Ok(None)
                }
            }
            WeightedList::List(vec) => {
                let mut out = Vec::<Weighted<S>>::with_capacity(vec.len());
                for v in vec {
                    if let Some(item) = filter(v.item())? {
                        out.push(match v {
                            Weighted::Weighted { item: _, weight } => Weighted::Weighted {
                                item,
                                weight: *weight,
                            },
                            Weighted::Unweighted(_) => Weighted::Unweighted(item),
                        });
                    }
                }
                if out.is_empty() {
                    Ok(None)
                } else {
                    Ok(Some(WeightedList::List(out)))
                }
            }
        }
    }

    pub fn try_map<F, S, E>(&self, mut filter: F) -> Result<WeightedList<S>, E>
    where
        F: FnMut(&T) -> Result<S, E>,
        S: fmt::Debug + Clone,
    {
        match self {
            WeightedList::Single(v) => {
                let item = filter(v)?;
                Ok(WeightedList::Single(item))
            }
            WeightedList::List(vec) => {
                let mut out = Vec::<Weighted<S>>::with_capacity(vec.len());
                for v in vec {
                    let item = filter(v.item())?;

                    out.push(match v {
                        Weighted::Weighted { item: _, weight } => Weighted::Weighted {
                            item,
                            weight: *weight,
                        },
                        Weighted::Unweighted(_) => Weighted::Unweighted(item),
                    });
                }
                Ok(WeightedList::List(out))
            }
        }
    }

    pub fn iter(&self) -> WeightedListIter<'_, T> {
        WeightedListIter {
            values: self,
            index: 0,
        }
    }
}

//////////////////////////////////////////////////////////////////////////
/// Index

impl<T: fmt::Debug + Clone> core::ops::Index<usize> for WeightedList<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        match self {
            WeightedList::Single(s) => s,
            WeightedList::List(vec) => vec[index].item(),
        }
    }
}

//////////////////////////////////////////////////////////////////////////
/// Iterator

pub struct WeightedListIter<'a, T: fmt::Debug + Clone> {
    values: &'a WeightedList<T>,
    index: usize,
}

impl<'a, T: fmt::Debug + Clone> Iterator for WeightedListIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.values.len() {
            return None;
        }

        self.index += 1;
        Some(&self.values[self.index - 1])
    }
}

//////////////////////////////////////////////////////////////////////////
/// Validate

impl<T: core::hash::Hash + Eq + fmt::Debug + Clone> Validate for WeightedList<T> {
    fn validate(&self) -> Result<(), ValidationErrors> {
        let mut errors = ValidationErrors::new();

        // Ensure weighted list does not have duplicates
        let items = self.iter().collect::<HashSet<_>>();
        if items.len() != self.len() {
            errors.add(
                "List",
                ValidationError::new("weightdup")
                    .with_message("weighted list must not have duplicate items".into()),
            );
        }

        // Make sure list is not empty
        match self {
            Self::List(v) => {
                if v.is_empty() {
                    errors.add(
                        "List",
                        ValidationError::new("len")
                            .with_message("weighted list must not be empty".into()),
                    )
                }
                errors.merge_self("List", v.validate());
            }
            Self::Single(_addr) => {}
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}

// impl<T: core::hash::Hash + Eq + fmt::Debug + Clone> WeightedList<T> {
//     pub fn validate_once(&self) -> Result<(), ValidationError> {
//         self.validate().map_err(|errs| {
//             ValidationError::new("multiple")
//                 .with_message(format!("multiple validation errors: {}", errs).into())
//         })
//     }
// }

//////////////////////////////////////////////////////////////////////////
/// Weighted

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Weighted<T: fmt::Debug + Clone> {
    Weighted { item: T, weight: f32 },
    Unweighted(T),
}

impl<T: fmt::Debug + Clone> Validate for Weighted<T> {
    fn validate(&self) -> Result<(), ValidationErrors> {
        let mut errors = ValidationErrors::new();
        if let Self::Weighted { item: _, weight } = self {
            if *weight <= 0.0 {
                errors.add(
                    "Weighted",
                    ValidationError::new("len")
                        .with_message("weight must be a positive value".into()),
                )
            }
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}

impl<T: fmt::Debug + Clone> Weighted<T> {
    pub fn item(&self) -> &T {
        match self {
            Weighted::Weighted { item, weight: _ } => item,
            Weighted::Unweighted(item) => item,
        }
    }
    pub fn into_item(self) -> T {
        match self {
            Weighted::Weighted { item, weight: _ } => item,
            Weighted::Unweighted(item) => item,
        }
    }
    pub fn weight(&self) -> f32 {
        match self {
            Weighted::Weighted { item: _, weight } => *weight,
            Weighted::Unweighted(_) => 1.0f32,
        }
    }
}
