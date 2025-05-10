use crate::coalesce::Coalesce;

pub trait History<S = Self> {
    fn prior(self, other: S) -> Self;
    fn posterior(self, other: S) -> Self;
}
pub trait IntoHistory: Sized {
    type History; // TODO remove this type parameter ?
    fn into_history(self) -> Self::History;
    fn history_prior<S>(self, other: S) -> Self::History
    where
        Self::History: History,
        S: IntoHistory<History = Self::History>,
    {
        self.into_history().prior(other.into_history())
    }
    fn history_posterior<S>(self, other: S) -> Self::History
    where
        Self::History: History,
        S: IntoHistory<History = Self::History>,
    {
        self.into_history().posterior(other.into_history())
    }
}
impl<T: Coalesce> IntoHistory for T {
    type History = Coalesced<T>;
    default fn into_history(self) -> Self::History {
        Coalesced::new(self)
    }
}
impl<T> IntoHistory for Coalesced<T> {
    type History = Self;
    fn into_history(self) -> Self::History {
        self
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
pub enum Priority {
    #[default]
    Prior,
    Posterior,
}
/// Some values for lazy evaluation
///
/// # Examples
/// An instance can be created with [`IntoHistory::into_history`].
/// ```
/// use coalesced_base::{coalesce::Coalesce, coalesced::{History, IntoHistory}};
/// let history = Some(100).into_history();
/// assert_eq!(history.base(), &Some(100));
///
/// let none = None.into_history();
/// let history = history.prior(none);
/// assert_eq!(history.base(), &Some(100));
/// assert_eq!(history.into(), Some(100));
/// ```
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
pub struct Coalesced<T> {
    base: T,
    history: Vec<(Priority, T)>,
}
impl<S, T> History<S> for Coalesced<T>
where
    S: IntoHistory<History = Coalesced<T>>,
{
    fn prior(mut self, other: S) -> Self {
        let history = other.into_history();
        self.history.push((Priority::Prior, history.base));
        self.history.extend(history.history);
        self
    }
    fn posterior(mut self, other: S) -> Self {
        let history = other.into_history();
        self.history.push((Priority::Posterior, history.base));
        self.history.extend(history.history);
        self
    }
}
impl<T: Coalesce> Coalesced<T> {
    pub fn new(value: T) -> Self {
        Self {
            base: value,
            history: vec![],
        }
    }
    pub fn base(&self) -> &T {
        &self.base
    }
    pub fn history(&self) -> &Vec<(Priority, T)> {
        &self.history
    }

    pub fn into(self) -> T {
        self.history
            .into_iter()
            .fold(self.base, |c, (p, x)| match p {
                Priority::Prior => c.prior(x),
                Priority::Posterior => c.posterior(x),
            })
    }
}
impl<T: Coalesce + Clone> Coalesced<T> {
    pub fn into_cloned(&self) -> T {
        self.history
            .iter()
            .cloned()
            .fold(self.base.clone(), |c, (p, x)| match p {
                Priority::Prior => c.prior(x),
                Priority::Posterior => c.posterior(x),
            })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_history_coalesced() {
        let v1 = Some(1);
        let v2 = None;

        let coalesced = v1.history_prior(v2);
        assert_eq!(coalesced.base(), &Some(1));
        assert_eq!(coalesced.history(), &vec![(Priority::Prior, None)]);
        assert_eq!(coalesced.into(), Some(1));
    }

    #[test]
    fn test_history_coalesced_prior() {
        let v1 = 1;
        let v2 = 2;
        let v3 = 3;

        let coalesced = v1.history_prior(v2).prior(v3);
        assert_eq!(coalesced.into(), 3);
    }

    #[test]
    fn test_history_coalesced_posterior() {
        let v1 = 1;
        let v2 = 2;
        let v3 = 3;

        let coalesced = v1.history_posterior(v2).posterior(v3);
        assert_eq!(coalesced.into(), 1);
    }
}
