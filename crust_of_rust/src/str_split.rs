// Lifetime is specifying that the values that the struct hold live only as long as the struct does.
// We need two different lifetimes because the returned value is tied only with the remainder and not the delimiter, so they can have different lifetimes.
// For more clarification refert to the function until_first_occurance_of_char in this file.
#[derive(Debug)]
pub struct StrSplit<'haystack, D> {
    // The remaining part of the string that we have yet to go over.
    remainder: Option<&'haystack str>,

    // The value we are splitting by.
    delimiter: D,
}

impl<'haystack, D> StrSplit<'haystack, D> {
    // We've to specify a lifetime here is because, without them haystack and delimiter that are passed to the function as an argument would have
    // a lifetime unknown to the compiler and we've defined on StrSplit struct that the lifetime of remainder, delimiter and StrSplit are the
    // same, this can be violated if we do not specify a lifetime. Essentially without specifying a lifetime StrSplit has some arbitrary lifetime
    // which is not directly associated with the inputs of the new functon, leading to anonymitiy for which the compiler complains.
    //
    // Think of it like this, its clear that '_ and 'some_other_lifetime have noting that ties them together.
    // impl StrSplit<'_> {
    //     pub fn new(haystack: &'some_other_lifetime str, delimiter: &'some_other_lifetime str) -> Self<'_> {
    //         Self {
    //             remainder: haystack,
    //             delimiter,
    //         }
    //     }
    // }
    //
    // Kind of hard to understand I'm not gonna lie.
    //
    // TL;DR; We specify that the StrSplit is valid as long as the strings we've provided it as input to the new function are valid.
    // Without explicitly specifying that it's ambiguous whether or not the StrSplit will live longer than, shorter than or just the same time as the input strings.
    pub fn new(haystack: &'haystack str, delimiter: D) -> Self {
        Self {
            remainder: Some(haystack),
            delimiter,
        }
    }
}

pub trait Delimiter {
    fn find_next(&self, s: &str) -> Option<(usize, usize)>;
}

impl<'haystack, D> Iterator for StrSplit<'haystack, D>
where
    D: Delimiter,
{
    // It is important to specify the lifetime here since we are using an iterator the reference the iterator gives out may likely outlive the remainder or the StrSplit.
    // It doesn't know when calling the iterator next method how long to keep the reference/pointer for. That's why it asks us to explicitly give it the lifetime specifier for the Item.
    // Essentially here there is a lifetime that is longer than the one given to StrSplit, i.e. what the iterator returns. It can outlive the StrSplit and the strings inside it
    // leading to use after drop which is problematic.
    type Item = &'haystack str;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(ref mut remainder) = self.remainder {
            if let Some((delim_start, delim_end)) = self.delimiter.find_next(&*remainder) {
                let string_until_delimiter = &remainder[..delim_start];
                // Deref cause ofcourse the types on both sides of the equals should match.
                *remainder = &remainder[delim_end..];
                Some(string_until_delimiter)
            } else {
                self.remainder.take()
            }
        } else {
            None
        }
    }
}

// You can also implement Delimiter for char, he does it in the video I don't do it here.
impl Delimiter for &str {
    fn find_next(&self, s: &str) -> Option<(usize, usize)> {
        // self cause of course we're finding the delimiter in the given string. Delimiter here is self.
        s.find(self).map(|start| (start, start + self.len()))
    }
}

// Currently here for explanation purposes only.
// Shows why we need more than one lifetime annotations on the StrSplit function.
#[allow(unused)]
fn until_first_occurance_of_char(s: &str, c: char) -> &str {
    // Do know that implementing Delimiter for char would be a better idea since it avoids the extra allocation that we do here.
    let delim: &str = &format!("{}", c);

    // If you have just one lifetime on StrSplit then this would be wrong, cause the compiler says that the returned str's lifetime is associated with
    // the lifetime of the delimiter: the &format!("{}", c) which is scopted to the function, but the return value outlives it thus we have a problem.
    // s has a lifetime of greater than the function in question so it gets downgraded to the lifetime of the function.
    StrSplit::new(s, delim)
        .next()
        .expect("StrSplit is guaranteed to return at least one result.")
}

#[test]
fn initial_working_test() {
    let haystack = "s o m e t h i n g t o r e f l e c t u p o n";
    let letters: Vec<_> = StrSplit::new(haystack, " ").collect();

    assert_eq!(
        letters,
        vec![
            "s", "o", "m", "e", "t", "h", "i", "n", "g", "t", "o", "r", "e", "f", "l", "e", "c",
            "t", "u", "p", "o", "n"
        ]
    );

    // Or you could do
    // let letters = StrSplit::new(haystack, " ");

    // assert_eq!(
    //     letters,
    //     vec![
    //         "s", "o", "m", "e", "t", "h", "i", "n", "g", "t", "o", "r", "e", "f", "l", "e", "c",
    //         "t", "u", "p", "o", "n"
    //     ]
    //     .into_iter()
    // );
}

#[test]
fn ends_in_delimiter_test() {
    let haystack = "a b c d ";
    let letters: Vec<_> = StrSplit::new(haystack, " ").collect();

    // Since it ends in a delimiter we want the function to generate a default/empty state representing that.
    // Otherwise it feels like we are discarding or loosing data.
    assert_eq!(letters, vec!["a", "b", "c", "d", ""]);
}

#[test]
fn multiple_lifetime_function_test() {
    let haystack = "a b c d ";
    let letters: &str = until_first_occurance_of_char(haystack, ' ');

    // Since it ends in a delimiter we want the function to generate a default/empty state representing that.
    // Otherwise it feels like we are discarding or loosing data.
    assert_eq!(letters, &"a".to_string());
}
