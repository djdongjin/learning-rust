// #![warn(missing_debug_implementations, rust_2018_idioms, missing_docs)]

/*!
 * Two things not added:
 * 1. change 'b to a generic type that impl a Delimiter trait.
 * 2. remove `empty_tail` and change `remainder` to an `Option<&'a str>` to
 *      indicate if the remainder is Empty or None.
 */
#[derive(Debug)]
pub struct StrSplit<'a, 'b> {
    remainder: &'a str,
    delimiter: &'b str,
    empty_tail: bool,
}

impl<'a, 'b> StrSplit<'a, 'b> {
    pub fn new(haystack: &'a str, delimiter: &'b str) -> Self {
        Self {
            remainder: haystack,
            delimiter,
            empty_tail: false,
        }
    }
}

impl<'a, 'b> Iterator for StrSplit<'a, 'b> {
    type Item = &'a str;
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(next_delim) = self.remainder.find(self.delimiter) {
            // find a delimiter, return first part, change remainder to the rest part.
            let until_delimiter = &self.remainder[..next_delim];
            self.remainder = &self.remainder[(next_delim + self.delimiter.len())..];
            if self.remainder.is_empty() {
                self.empty_tail = true;
            }
            Some(until_delimiter)
        } else if self.remainder.is_empty() {
            match self.empty_tail {
                true => {
                    self.empty_tail = false;
                    Some("")
                }
                false => None,
            }
        } else {
            // no more delimiter, return the last part.
            let rest = self.remainder;
            // below line is ok because:
            // self.remainder: 'a, "": 'static -> 'static > 'a
            self.remainder = "";
            Some(rest)
        }
    }
}

pub fn until_char<'s>(s: &'s str, c: char) -> &'s str {
    StrSplit::new(s, &format!("{}", c))
        .next()
        .expect("StrSplit always return at least one segment.")
}

#[test]
fn test_until_char() {
    assert_eq!(until_char("hello world", 'o'), "hell");
}

#[test]
fn it_works() {
    let haystack = "a b c d e";
    let letters = StrSplit::new(haystack, " ");
    // assert_eq!(letters, vec!["a", "b", "c", "d", "e"].into_iter());
    assert!(letters.eq(vec!["a", "b", "c", "d", "e"].into_iter()));
}

#[test]
fn it_works_tail() {
    let haystack = "a b c d ";
    let letters = StrSplit::new(haystack, " ");
    // assert_eq!(letters, vec!["a", "b", "c", "d", "e"].into_iter());
    assert!(letters.eq(vec!["a", "b", "c", "d", ""].into_iter()));
}

#[test]
fn it_works_head() {
    let haystack = " b c d e";
    let letters = StrSplit::new(haystack, " ");
    // assert_eq!(letters, vec!["a", "b", "c", "d", "e"].into_iter());
    assert!(letters.eq(vec!["", "b", "c", "d", "e"].into_iter()));
}
