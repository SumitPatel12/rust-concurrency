// Fair warning we're gonig full comment mode for these things. IT IS NOT AI GENERATED. STOP THE SLANDER!!
// I had to write that goofy ass comment. :shrug_emote:
// I'm gonna have this on all files :toll_face_emote:

// Iterator is a trait first of all. The most important aspect of it:
//   1. It has an Item type, which drives what the iterator will yield.
//   2. It has a next function which is called by the piece of code running the iterator. It returns either a Some(Item) or None when the iterator has been
//      exhausted
// There are ofcourse more functions in there but these two are the most common and important ones to know.

pub fn flatten<I>(iterator: I) -> Flatten<I::IntoIter>
where
    I: IntoIterator,
    I::Item: IntoIterator,
{
    Flatten::new(iterator.into_iter())
}

pub struct Flatten<O>
where
    O: Iterator,
    O::Item: IntoIterator,
{
    outer: O,
    // This is because we need to track which inner is being yielded from in the iterator for the flatten method.
    // The inner is question is going to be converted to its iterable form for convenience reasons.
    inner: Option<<O::Item as IntoIterator>::IntoIter>,
}

impl<O> Flatten<O>
where
    O: Iterator,
    O::Item: IntoIterator,
{
    fn new(iterator: O) -> Self {
        Flatten {
            outer: iterator,
            inner: None,
        }
    }
}

impl<O> Iterator for Flatten<O>
where
    // Outer is iteratble and the iner types or the constituent types of outer are required to implement the IntoIterator.
    O: Iterator,
    O::Item: IntoIterator,
{
    type Item = <O::Item as IntoIterator>::Item;

    fn next(&mut self) -> Option<Self::Item> {
        // We keep the state of the inner elements in the struct itself. This is because we need to yield the next element from the inner iterator.
        // So, we simply loop over the following code:
        //  1. If the inner iterator has a value return it.
        //  2. Otherwise set the inner iterator to the next outer element.
        //  3. If it is empty it'd return back none.
        //  4. Otherwise set the inner iterator to the element of value yielded by the outer one and go to step 1.
        loop {
            if let Some(ref mut inner_iterator) = self.inner {
                if let Some(item) = inner_iterator.next() {
                    return Some(item);
                }

                self.inner = None;
            }

            let next_inner_iterator = self.outer.next()?.into_iter();
            self.inner = Some(next_inner_iterator);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        assert_eq!(flatten(std::iter::empty::<Vec<()>>()).count(), 0)
    }

    #[test]
    fn empty_nested() {
        assert_eq!(flatten(vec![Vec::<()>::new(), vec![], vec![]]).count(), 0)
    }

    #[test]
    fn one() {
        assert_eq!(flatten(std::iter::once(vec![0])).count(), 1)
    }

    #[test]
    fn two_in_same_vec() {
        assert_eq!(flatten(std::iter::once(vec![0, 1])).count(), 2)
    }

    #[test]
    fn two_different_vecs() {
        assert_eq!(flatten(vec![vec![0], vec![1]]).count(), 2)
    }

    #[test]
    fn four_in_two_vecs() {
        assert_eq!(flatten(vec![vec![0, 0], vec![1, 1]]).count(), 4)
    }
}
