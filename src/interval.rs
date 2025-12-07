use std::{fmt::Display, ops::Sub, ptr::eq, vec};

pub struct Interval<T> {
    from: T,
    to: T,
}

impl<T: std::str::FromStr> From<&str> for Interval<T>
where
    T::Err: std::fmt::Debug,
{
    fn from(s: &str) -> Self {
        let mut parts = s.split("-");
        let from = parts.next().unwrap().parse().unwrap();
        let to = parts.next().unwrap().parse().unwrap();
        Interval { from, to }
    }
}

impl<T: PartialOrd> Interval<T> {
    pub fn is_in_inclusive(&self, value: &T) -> bool {
        if self.from <= *value && *value <= self.to {
            return true;
        }

        false
    }
}

impl<T: Display> Display for Interval<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}, {}]", self.from, self.to)?;
        Ok(())
    }
}

impl<T: Clone> Clone for Interval<T> {
    fn clone(&self) -> Self {
        Interval {
            from: self.from.clone(),
            to: self.to.clone(),
        }
    }
}

impl<T> Interval<T>
where
    T: PartialOrd + Clone,
{
    pub fn join(&self, other: &Self) -> Vec<Interval<T>> {
        let lower = if self.from < other.from { self } else { other };
        let high = if self.to > other.to { self } else { other };

        if eq(lower, high) {
            return vec![Interval {
                from: lower.from.clone(),
                to: high.to.clone(),
            }];
        }

        if lower.to >= high.from {
            return vec![Interval {
                from: lower.from.clone(),
                to: high.to.clone(),
            }];
        }

        return vec![lower.clone(), high.clone()];
    }
}

impl<T> Interval<T>
where
    T: Sub<Output = T> + Clone,
{
    pub fn diff(&self) -> T {
        return self.to.clone() - self.from.clone();
    }
}
