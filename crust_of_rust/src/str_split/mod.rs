// Lifetime is specifying that the values that the struct hold live only as long as the struct does.
pub struct StrSplit<'a> {
    // The remaining part of the string that we have yet to go over.
    remainder: &'a str,

    // The value we are splitting by.
    delimiter: &'a str,
}

impl<'a> StrSplit<'a> {
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
    pub fn new(haystack: &'a str, delimiter: &'a str) -> Self {
        Self {
            remainder: haystack,
            delimiter,
        }
    }
}

impl<'a> Iterator for StrSplit<'a> {
    // It is important to specify the lifetime here since we are using an iterator the reference the iterator gives out may likely outlive the remainder or the StrSplit.
    // It doesn't know when calling the iterator next method how long to keep the reference/pointer for. That's why it asks us to explicitly give it the lifetime specifier for the Item.
    // Essentially here there is a lifetime that is longer than the one given to StrSplit, i.e. what the iterator returns. It can outlive the StrSplit and the strings inside it
    // leading to use after drop which is problematic.
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(next_delimiter) = self.remainder.find(self.delimiter) {
            let until_delimiter = &self.remainder[..next_delimiter];
            self.remainder = &self.remainder[(next_delimiter + self.delimiter.len())..];
            Some(until_delimiter)
        } else if !self.remainder.is_empty() {
            let remaining_string = self.remainder;
            // self.remainder has 'a lifetime and "" has 'static lifetime.
            // Know that 'static lifetime lives for the entire lifetime of the program.
            // The compiler doesn't complaint about this because anything that has the same type but a longer lifetime can be assigned to one with a shorter lifetime.
            // Because it is guaranteed that it will be living for as long the shorter one does, the compiler is fine with it.
            self.remainder = "";
            Some(remaining_string)
        } else {
            None
        }
    }
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
