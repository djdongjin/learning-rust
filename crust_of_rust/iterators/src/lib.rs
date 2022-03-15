// So that we can call flatten as a method instead of a function.
// The struct implementing the trait should be a two-level iterator.
// We limit the inner iterator (Self::Item) to be `IntoIterator` instead of
// "Iterator", so that the inner iterators can be some collections such as
// `Vec` and we will convert it into `Iterator` in our implementation.
pub trait IteratorExt: Iterator + Sized {
    fn our_flatten(self) -> Flatten<Self>
    where
        Self::Item: IntoIterator;
}

// Implement `IteratorExt` for all `Iterator` types.
impl<T> IteratorExt for T
where
    T: Iterator,
{
    fn our_flatten(self) -> Flatten<Self>
    where
        Self::Item: IntoIterator,
    {
        flatten(self)
    }
}

// By specifying `I` as `IntoIterator` instead of `Iterator`, the caller doesn't
// need to convert a connection into `Iterator` before calling.
// E.g., we can call `flatten(vec![vec![1], vec![3]])` instead of
// `flatten(vec![vec![1].into_iter(), vec![3].into_iter()].into_iter())`.
pub fn flatten<I>(iter: I) -> Flatten<I::IntoIter>
where
    I: IntoIterator,
    I::Item: IntoIterator,
{
    Flatten::new(iter.into_iter())
}

// Flatten only stores an outer iterator, and each of its element is also
// an iterator (inner iterator).
pub struct Flatten<O>
where
    O: Iterator,
    O::Item: IntoIterator,
{
    outer: O,
    front_iter: Option<<O::Item as IntoIterator>::IntoIter>,
    back_iter: Option<<O::Item as IntoIterator>::IntoIter>,
}

impl<O> Flatten<O>
where
    O: Iterator,
    O::Item: IntoIterator,
{
    pub fn new(iter: O) -> Self {
        Flatten {
            outer: iter,
            front_iter: None,
            back_iter: None,
        }
    }
}

impl<O> Iterator for Flatten<O>
where
    O: Iterator,
    O::Item: IntoIterator,
{
    type Item = <O::Item as IntoIterator>::Item; // `Item` should match `Item` in the inner iterator.
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            // `front_iter` is not None.
            if let Some(front_iter) = &mut self.front_iter {
                // `front_iter` has value in its `next` call.
                if let Some(i) = front_iter.next() {
                    return Some(i);
                }
                // `front_iter` has no value.
                self.front_iter = None;
            }

            // `front_iter` is None, we need to move `outer` if it's not None, or return value from
            // `back_iter`.
            if let Some(next_inner) = self.outer.next() {
                self.front_iter = Some(next_inner.into_iter()) // `O::Item` is `IntoIterator`.
            } else {
                return self.back_iter.as_mut()?.next();
            }
        }
    }
}

impl<O> DoubleEndedIterator for Flatten<O>
where
    O: DoubleEndedIterator,
    O::Item: IntoIterator,
    <O::Item as IntoIterator>::IntoIter: DoubleEndedIterator,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        loop {
            // `back_iter` is not None.
            if let Some(back_iter) = &mut self.back_iter {
                // `back_iter` has value in its `next` call.
                if let Some(i) = back_iter.next_back() {
                    return Some(i);
                }
                // `back_iter` has no value.
                self.back_iter = None;
            }

            // `back_iter` is None, we need to move `outer` if it's not None, or return value from
            // `front_iter`.
            if let Some(next_inner) = self.outer.next_back() {
                self.back_iter = Some(next_inner.into_iter()) // `O::Item` is `IntoIterator`.
            } else {
                return self.front_iter.as_mut()?.next();
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        assert_eq!(flatten(std::iter::empty::<Vec<()>>()).count(), 0);
    }

    #[test]
    fn empty_wide() {
        assert_eq!(flatten(vec![Vec::<()>::new(), vec![], vec![]]).count(), 0);
    }

    #[test]
    fn one() {
        assert_eq!(flatten(std::iter::once(vec!["a"])).count(), 1);
    }

    #[test]
    fn two() {
        assert_eq!(flatten(std::iter::once(vec!["a", "b"])).count(), 2);
    }

    #[test]
    fn two_wide() {
        assert_eq!(flatten(vec![vec!["a"], vec!["b"]]).count(), 2);
    }

    #[test]
    fn reverse() {
        assert_eq!(
            flatten(std::iter::once(vec!["a", "b"]))
                .rev()
                .collect::<Vec<_>>(),
            vec!["b", "a"]
        );
    }

    #[test]
    fn reverse_wide() {
        assert_eq!(
            flatten(vec![vec!["a"], vec!["b"]])
                .rev()
                .collect::<Vec<_>>(),
            vec!["b", "a"]
        );
    }

    #[test]
    fn both_ends() {
        let mut iter = flatten(vec![vec!["a1", "a2", "a3"], vec!["b1", "b2", "b3"]]);
        assert_eq!(iter.next(), Some("a1"));
        assert_eq!(iter.next_back(), Some("b3"));
        assert_eq!(iter.next(), Some("a2"));
        assert_eq!(iter.next_back(), Some("b2"));
        assert_eq!(iter.next(), Some("a3"));
        assert_eq!(iter.next_back(), Some("b1"));
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next_back(), None);
    }

    #[test]
    fn inf() {
        let mut iter = flatten((0..).map(|i| 0..i));
        // 0 => 0..0 => empty
        // 1 => 0..1 => [0]
        // 2 => 0..2 => [0, 1]
        assert_eq!(iter.next(), Some(0));
        assert_eq!(iter.next(), Some(0));
        assert_eq!(iter.next(), Some(1));
    }

    #[test]
    fn deep() {
        assert_eq!(flatten(flatten(vec![vec![vec![0, 1]]])).count(), 2);
    }

    #[test]
    fn ext() {
        assert_eq!(vec![vec![0, 1]].into_iter().our_flatten().count(), 2);
    }
}
