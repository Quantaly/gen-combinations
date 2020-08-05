//! A general combination generator that iterates over all possible combinations of a slice of items.
//!
//! Note that combinations are different than permutations in that this crate will not generate all possible orderings
//! of those items.
//!
//! This crate does not check for uniqueness among the items; if this is desired, it is left up to the user to ensure that
//! the items are unique before passing them to [`generate`].
//! 
//! [`generate`]: fn.generate.html

/// Iterates over all possible combinations of `n` `items`.
/// 
/// The combinations are of immutable references to the items.
///
/// If n is larger than the length of items, or if n is 0, the iterator will produce no values.
/// 
/// # Examples
/// 
/// ```
/// use gen_combinations::generate;
/// 
/// let items = [1, 2, 3];
/// for combo in generate(&items, 2) {
///     println!("{:?}", combo);
///     // [1, 2]
///     // [1, 3]
///     // [2, 3]
/// }
/// ```
pub fn generate<T>(items: &[T], n: usize) -> CombinationIterator<T> {
    let indices = (0..n).collect();
    CombinationIterator { items, indices }
}


/// Returned by [`generate`]. Iterates over all possible combinations of items.
/// 
/// [`generate`]: fn.generate.html
pub struct CombinationIterator<'a, T> {
    items: &'a [T],
    indices: Vec<usize>,
}

impl<'a, T> Iterator for CombinationIterator<'a, T> {
    type Item = Vec<&'a T>;

    fn next(&mut self) -> Option<Vec<&'a T>> {
        if self.indices.is_empty() || self.indices.len() > self.items.len() {
            None
        } else {
            let ret = self.indices.iter().map(|i| &(self.items[*i])).collect();
            for i in (0..self.indices.len()).rev() {
                if self.indices[i] < self.items.len() - (self.indices.len() - i) {
                    self.indices[i] += 1;
                    for j in i..self.indices.len() {
                        self.indices[j] = self.indices[i] + (j - i);
                    }
                    return Some(ret);
                }
            }
            self.indices.clear(); // next iteration will see that self.indices is empty and stop
            Some(ret)
        }
    }
}

#[test]
fn generate_combinations() {
    let items = [1, 2, 3];
    let mut c = generate(&items, 2);
    assert_eq!(c.next(), Some(vec![&1, &2]));
    assert_eq!(c.next(), Some(vec![&1, &3]));
    assert_eq!(c.next(), Some(vec![&2, &3]));
    assert_eq!(c.next(), None);
}

#[test]
fn generate_more_combinations() {
    let items = [1, 2, 3, 4, 5];
    let mut c = generate(&items, 3);
    assert_eq!(c.next(), Some(vec![&1, &2, &3]));
    assert_eq!(c.next(), Some(vec![&1, &2, &4]));
    assert_eq!(c.next(), Some(vec![&1, &2, &5]));
    assert_eq!(c.next(), Some(vec![&1, &3, &4]));
    assert_eq!(c.next(), Some(vec![&1, &3, &5]));
    assert_eq!(c.next(), Some(vec![&1, &4, &5]));
    assert_eq!(c.next(), Some(vec![&2, &3, &4]));
    assert_eq!(c.next(), Some(vec![&2, &3, &5]));
    assert_eq!(c.next(), Some(vec![&2, &4, &5]));
    assert_eq!(c.next(), Some(vec![&3, &4, &5]));
    assert_eq!(c.next(), None);
}

#[test]
fn generate_combinations_of_things_that_arent_copy_just_to_be_sure() {
    let items = [String::from("one"), String::from("two"), String::from("yeet")];
    let mut c = generate(&items, 2);
    assert_eq!(c.next(), Some(vec![&String::from("one"), &String::from("two")]));
    assert_eq!(c.next(), Some(vec![&String::from("one"), &String::from("yeet")]));
    assert_eq!(c.next(), Some(vec![&String::from("two"), &String::from("yeet")]));
    assert_eq!(c.next(), None);
}

#[test]
fn misuse_arguments() {
    let items = [1, 2, 3];
    let mut c = generate(&items, 500);
    assert_eq!(c.next(), None);

    let mut c = generate(&items, 0);
    assert_eq!(c.next(), None);
}
