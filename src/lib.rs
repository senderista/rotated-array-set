//! An ordered set based on a 2-level rotated array.
//!
//! See <a href="https://github.com/senderista/rotated-array-set/blob/master/README.md">the repository README</a> for a detailed discussion of this collection's performance
//! benefits and drawbacks.

#![doc(html_root_url = "https://docs.rs/rotated-array-set/0.1.0/rotated_array_set/")]
#![doc(
    html_logo_url = "https://raw.githubusercontent.com/senderista/rotated-array-set/master/img/cells.png"
)]

use std::cmp::Ordering::{self, Equal, Greater, Less};
use std::cmp::{max, min};
use std::fmt::Debug;
use std::hash::{Hash, Hasher};
use std::iter::{DoubleEndedIterator, ExactSizeIterator, FromIterator, FusedIterator, Peekable};
use std::mem;
use std::ops::Bound::{Excluded, Included, Unbounded};
use std::ops::RangeBounds;
// remove when Iterator::is_sorted is stabilized
use is_sorted::IsSorted;

/// An ordered set based on a 2-level rotated array.
///
/// # Examples
///
/// ```
/// use rotated_array_set::RotatedArraySet;
///
/// // Type inference lets us omit an explicit type signature (which
/// // would be `RotatedArraySet<i32>` in this example).
/// let mut ints = RotatedArraySet::new();
///
/// // Add some integers.
/// ints.insert(-1);
/// ints.insert(6);
/// ints.insert(1729);
/// ints.insert(24);
///
/// // Check for a specific one.
/// if !ints.contains(&42) {
///     println!("We don't have the answer to Life, the Universe, and Everything :-(");
/// }
///
/// // Remove an integer.
/// ints.remove(&6);
///
/// // Iterate over everything.
/// for int in &ints {
///     println!("{}", int);
/// }
/// ```
#[derive(Debug, Clone)]
pub struct RotatedArraySet<T> {
    data: Vec<T>,
    min_indexes: Vec<usize>,
    min_data: Vec<T>,
}

// Internal encapsulation of container + bounds
#[derive(Debug, Copy, Clone)]
struct Range<'a, T: 'a> {
    container: &'a RotatedArraySet<T>,
    start_index_inclusive: usize,
    end_index_exclusive: usize,
}

impl<'a, T> Range<'a, T>
where
    T: Ord + Copy + Default + Debug,
{
    fn with_bounds(
        container: &'a RotatedArraySet<T>,
        start_index_inclusive: usize,
        end_index_exclusive: usize,
    ) -> Range<'a, T> {
        assert!(end_index_exclusive >= start_index_inclusive);
        assert!(end_index_exclusive <= container.len());
        Range {
            container,
            start_index_inclusive,
            end_index_exclusive,
        }
    }

    fn new(container: &'a RotatedArraySet<T>) -> Range<'a, T> {
        Range::with_bounds(container, 0, container.len())
    }

    fn at(&self, index: usize) -> Option<&'a T> {
        let container_idx = index + self.start_index_inclusive;
        self.container.select(container_idx)
    }

    fn len(&self) -> usize {
        self.end_index_exclusive - self.start_index_inclusive
    }
}

/// An iterator over the items of a `RotatedArraySet`.
///
/// This `struct` is created by the [`iter`] method on [`RotatedArraySet`][`RotatedArraySet`].
/// See its documentation for more.
///
/// [`RotatedArraySet`]: struct.RotatedArraySet.html
/// [`iter`]: struct.RotatedArraySet.html#method.iter
#[derive(Debug, Copy, Clone)]
pub struct Iter<'a, T: 'a> {
    range: Range<'a, T>,
    next_index: usize,
    next_rev_index: usize,
}

impl<'a, T> Iter<'a, T>
where
    T: Ord + Copy + Default + Debug,
{
    fn new(range: Range<'a, T>) -> Iter<'a, T> {
        let next_index = 0;
        let next_rev_index = if range.len() == 0 { 0 } else { range.len() - 1 };
        Iter {
            range,
            next_index,
            next_rev_index,
        }
    }

    #[inline(always)]
    fn assert_invariants(&self) -> bool {
        assert!(self.next_index <= self.range.len());
        assert!(self.next_rev_index <= self.range.len());
        if self.next_rev_index < self.next_index {
            assert!(self.next_index - self.next_rev_index == 1);
        }
        true
    }
}

/// An owning iterator over the items of a `RotatedArraySet`.
///
/// This `struct` is created by the [`into_iter`] method on [`RotatedArraySet`][`RotatedArraySet`]
/// (provided by the `IntoIterator` trait). See its documentation for more.
///
/// [`RotatedArraySet`]: struct.RotatedArraySet.html
/// [`into_iter`]: struct.RotatedArraySet.html#method.into_iter
#[derive(Debug, Clone)]
pub struct IntoIter<T> {
    vec: Vec<T>,
    next_index: usize,
}

/// A lazy iterator producing elements in the difference of `RotatedArraySet`s.
///
/// This `struct` is created by the [`difference`] method on [`RotatedArraySet`].
/// See its documentation for more.
///
/// [`RotatedArraySet`]: struct.RotatedArraySet.html
/// [`difference`]: struct.RotatedArraySet.html#method.difference
#[derive(Debug, Clone)]
pub struct Difference<'a, T: 'a> {
    self_iter: Iter<'a, T>,
    other_set: &'a RotatedArraySet<T>,
}

/// A lazy iterator producing elements in the symmetric difference of `RotatedArraySet`s.
///
/// This `struct` is created by the [`symmetric_difference`] method on
/// [`RotatedArraySet`]. See its documentation for more.
///
/// [`RotatedArraySet`]: struct.RotatedArraySet.html
/// [`symmetric_difference`]: struct.RotatedArraySet.html#method.symmetric_difference
#[derive(Debug, Clone)]
pub struct SymmetricDifference<'a, T: 'a>
where
    T: Ord + Copy + Default + Debug,
{
    a: Peekable<Iter<'a, T>>,
    b: Peekable<Iter<'a, T>>,
}

/// A lazy iterator producing elements in the intersection of `RotatedArraySet`s.
///
/// This `struct` is created by the [`intersection`] method on [`RotatedArraySet`].
/// See its documentation for more.
///
/// [`RotatedArraySet`]: struct.RotatedArraySet.html
/// [`intersection`]: struct.RotatedArraySet.html#method.intersection
#[derive(Debug, Clone)]
pub struct Intersection<'a, T: 'a> {
    small_iter: Iter<'a, T>,
    large_set: &'a RotatedArraySet<T>,
}

/// A lazy iterator producing elements in the union of `RotatedArraySet`s.
///
/// This `struct` is created by the [`union`] method on [`RotatedArraySet`].
/// See its documentation for more.
///
/// [`RotatedArraySet`]: struct.RotatedArraySet.html
/// [`union`]: struct.RotatedArraySet.html#method.union
#[derive(Debug, Clone)]
pub struct Union<'a, T: 'a>
where
    T: Ord + Copy + Default + Debug,
{
    a: Peekable<Iter<'a, T>>,
    b: Peekable<Iter<'a, T>>,
}

impl<T> RotatedArraySet<T>
where
    T: Ord + Copy + Default + Debug,
{
    /// Makes a new `RotatedArraySet` without any heap allocations.
    ///
    /// This is a constant-time operation.
    ///
    /// # Examples
    ///
    /// ```
    /// # #![allow(unused_mut)]
    /// use rotated_array_set::RotatedArraySet;
    ///
    /// let mut set: RotatedArraySet<i32> = RotatedArraySet::new();
    /// ```
    pub fn new() -> Self {
        RotatedArraySet {
            data: Vec::new(),
            min_indexes: Vec::new(),
            min_data: Vec::new(),
        }
    }

    /// Constructs a new, empty `RotatedArraySet<T>` with the specified capacity.
    ///
    /// The set will be able to hold exactly `capacity` elements without
    /// reallocating. If `capacity` is 0, the set will not allocate.
    ///
    /// It is important to note that although the returned set has the
    /// *capacity* specified, the set will have a zero *length*.
    ///
    /// # Examples
    ///
    /// ```
    /// use rotated_array_set::RotatedArraySet;
    ///
    /// let mut set = RotatedArraySet::with_capacity(10);
    ///
    /// // The set contains no items, even though it has capacity for more
    /// assert_eq!(set.len(), 0);
    ///
    /// // These are all done without reallocating...
    /// for i in 0..10 {
    ///     set.insert(i);
    /// }
    ///
    /// // ...but this may make the set reallocate
    /// set.insert(11);
    /// ```
    pub fn with_capacity(capacity: usize) -> RotatedArraySet<T> {
        let min_indexes_capacity = if capacity > 0 {
            Self::get_subarray_idx_from_array_idx(capacity - 1) + 1
        } else {
            0
        };
        RotatedArraySet {
            data: Vec::with_capacity(capacity),
            min_indexes: Vec::with_capacity(min_indexes_capacity),
            min_data: Vec::with_capacity(min_indexes_capacity),
        }
    }

    /// Clears the set, removing all values.
    ///
    /// This is a constant-time operation.
    ///
    /// # Examples
    ///
    /// ```
    /// use rotated_array_set::RotatedArraySet;
    ///
    /// let mut v = RotatedArraySet::new();
    /// v.insert(1);
    /// v.clear();
    /// assert!(v.is_empty());
    /// ```
    pub fn clear(&mut self) {
        self.data.clear();
        self.min_indexes.clear();
        self.min_data.clear();
    }

    /// Returns `true` if the set contains a value.
    ///
    /// This is an `O(lg n)` operation.
    ///
    /// # Examples
    ///
    /// ```
    /// use rotated_array_set::RotatedArraySet;
    ///
    /// let set: RotatedArraySet<_> = vec![1, 2, 3].into();
    /// assert_eq!(set.contains(&1), true);
    /// assert_eq!(set.contains(&4), false);
    /// ```
    pub fn contains(&self, value: &T) -> bool {
        self.get(value).is_some()
    }

    /// Returns `true` if `self` has no elements in common with `other`.
    /// This is equivalent to checking for an empty intersection.
    ///
    /// # Examples
    ///
    /// ```
    /// use rotated_array_set::RotatedArraySet;
    ///
    /// let a: RotatedArraySet<_> = vec![1, 2, 3].into();
    /// let mut b = RotatedArraySet::new();
    ///
    /// assert_eq!(a.is_disjoint(&b), true);
    /// b.insert(4);
    /// assert_eq!(a.is_disjoint(&b), true);
    /// b.insert(1);
    /// assert_eq!(a.is_disjoint(&b), false);
    /// ```
    pub fn is_disjoint(&self, other: &RotatedArraySet<T>) -> bool {
        self.intersection(other).next().is_none()
    }

    /// Returns `true` if the set is a subset of another,
    /// i.e., `other` contains at least all the values in `self`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rotated_array_set::RotatedArraySet;
    ///
    /// let sup: RotatedArraySet<_> = vec![1, 2, 3].into();
    /// let mut set = RotatedArraySet::new();
    ///
    /// assert_eq!(set.is_subset(&sup), true);
    /// set.insert(2);
    /// assert_eq!(set.is_subset(&sup), true);
    /// set.insert(4);
    /// assert_eq!(set.is_subset(&sup), false);
    /// ```
    pub fn is_subset(&self, other: &RotatedArraySet<T>) -> bool {
        // Same result as self.difference(other).next().is_none()
        // but much faster.
        if self.len() > other.len() {
            false
        } else {
            // Iterate `self`, searching for matches in `other`.
            for next in self {
                if !other.contains(next) {
                    return false;
                }
            }
            true
        }
    }

    /// Returns `true` if the set is a superset of another,
    /// i.e., `self` contains at least all the values in `other`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rotated_array_set::RotatedArraySet;
    ///
    /// let sub: RotatedArraySet<_> = vec![1, 2].into();
    /// let mut set = RotatedArraySet::new();
    ///
    /// assert_eq!(set.is_superset(&sub), false);
    ///
    /// set.insert(0);
    /// set.insert(1);
    /// assert_eq!(set.is_superset(&sub), false);
    ///
    /// set.insert(2);
    /// assert_eq!(set.is_superset(&sub), true);
    /// ```
    pub fn is_superset(&self, other: &RotatedArraySet<T>) -> bool {
        other.is_subset(self)
    }

    /// Returns a reference to the value in the set, if any, that is equal to the given value.
    ///
    /// This is an `O(lg n)` operation.
    ///
    /// # Examples
    ///
    /// ```
    /// use rotated_array_set::RotatedArraySet;
    ///
    /// let set: RotatedArraySet<_> = vec![1, 2, 3].into();
    /// assert_eq!(set.get(&2), Some(&2));
    /// assert_eq!(set.get(&4), None);
    /// ```
    pub fn get(&self, value: &T) -> Option<&T> {
        let raw_idx = self.find_raw_index(value).ok()?;
        Some(&self.data[raw_idx])
    }

    /// Returns the rank of the value in the set if it exists (as `Result::Ok`),
    /// or the rank of its largest predecessor plus one, if it does not exist (as `Result::Err`).
    /// This is a constant-time operation.
    ///
    /// # Examples
    ///
    /// ```
    /// use rotated_array_set::RotatedArraySet;
    ///
    /// let set: RotatedArraySet<_> = vec![1, 2, 3].into();
    /// assert_eq!(set.rank(&1), Ok(0));
    /// assert_eq!(set.rank(&4), Err(3));
    /// ```
    pub fn rank(&self, value: &T) -> Result<usize, usize> {
        let (raw_index, exists) = match self.find_raw_index(value) {
            Ok(index) => (index, true),
            Err(index) => (index, false),
        };
        if raw_index == self.data.len() {
            return Err(raw_index);
        }
        debug_assert!(raw_index < self.data.len());
        let subarray_idx = Self::get_subarray_idx_from_array_idx(raw_index);
        let subarray_start_idx = Self::get_array_idx_from_subarray_idx(subarray_idx);
        let subarray_len = if subarray_idx == self.min_indexes.len() - 1 {
            self.data.len() - subarray_start_idx
        } else {
            subarray_idx + 1
        };
        let pivot_idx = subarray_start_idx + self.min_indexes[subarray_idx];
        let logical_index = if raw_index >= pivot_idx {
            subarray_start_idx + raw_index - pivot_idx
        } else {
            subarray_start_idx + subarray_len - (pivot_idx - raw_index)
        };
        if exists {
            Ok(logical_index)
        } else {
            Err(logical_index)
        }
    }

    /// Returns a reference to the value in the set, if any, with the given rank.
    ///
    /// This is a constant-time operation.
    ///
    /// # Examples
    ///
    /// ```
    /// use rotated_array_set::RotatedArraySet;
    ///
    /// let set: RotatedArraySet<_> = vec![1, 2, 3].into();
    /// assert_eq!(set.select(0), Some(&1));
    /// assert_eq!(set.select(3), None);
    /// ```
    pub fn select(&self, rank: usize) -> Option<&T> {
        if rank >= self.data.len() {
            return None;
        }
        let subarray_idx = Self::get_subarray_idx_from_array_idx(rank);
        let subarray_start_idx = Self::get_array_idx_from_subarray_idx(subarray_idx);
        let subarray_len = if subarray_idx == self.min_indexes.len() - 1 {
            self.data.len() - subarray_start_idx
        } else {
            subarray_idx + 1
        };
        debug_assert!(rank >= subarray_start_idx);
        let idx_offset = rank - subarray_start_idx;
        let pivot_offset = self.min_indexes[subarray_idx];
        let rotated_offset = (pivot_offset + idx_offset) % subarray_len;
        debug_assert!(rotated_offset < subarray_len);
        let raw_idx = subarray_start_idx + rotated_offset;
        Some(&self.data[raw_idx])
    }

    /// Adds a value to the set.
    ///
    /// This is an `O(√n)` operation.
    ///
    /// If the set did not have this value present, `true` is returned.
    ///
    /// If the set did have this value present, `false` is returned, and the
    /// entry is not updated.
    ///
    /// # Examples
    ///
    /// ```
    /// use rotated_array_set::RotatedArraySet;
    ///
    /// let mut set = RotatedArraySet::new();
    ///
    /// assert_eq!(set.insert(2), true);
    /// assert_eq!(set.insert(2), false);
    /// assert_eq!(set.len(), 1);
    /// ```
    pub fn insert(&mut self, value: T) -> bool {
        let insert_idx = match self.find_raw_index(&value).err() {
            None => return false,
            Some(idx) => idx,
        };
        // find subarray containing this insertion point
        let subarray_idx = Self::get_subarray_idx_from_array_idx(insert_idx);
        // inserted element could be in a new subarray
        debug_assert!(subarray_idx <= self.min_indexes.len());
        // create a new subarray if necessary
        if subarray_idx == self.min_indexes.len() {
            self.min_indexes.push(0);
            self.min_data.push(T::default());
        }
        debug_assert_eq!(self.min_indexes.len(), self.min_data.len());
        let subarray_offset = Self::get_array_idx_from_subarray_idx(subarray_idx);
        // if insertion point is in last subarray and last subarray isn't full, just insert the new element
        if subarray_idx == self.min_indexes.len() - 1 && !self.is_last_subarray_full() {
            // Since we always insert into a partially full subarray in sorted order,
            // there is no need to update the pivot location, but we do have to update
            // the pivot value.
            debug_assert!(self.min_indexes[subarray_idx] == 0);
            self.data.insert(insert_idx, value);
            // These writes are redundant unless the minimum has changed, but avoiding a branch may be worth it,
            // given that the end of the data arrays should be in cache.
            self.min_data[subarray_idx] = self.data[subarray_offset];
            debug_assert!(self.assert_invariants());
            return true;
        }
        // From now on, we can assume that the subarray we're inserting into is always full.
        let next_subarray_offset = Self::get_array_idx_from_subarray_idx(subarray_idx + 1);
        let subarray = &mut self.data[subarray_offset..next_subarray_offset];
        let pivot_offset = self.min_indexes[subarray_idx];
        let insert_offset = insert_idx - subarray_offset;
        let max_offset = if pivot_offset == 0 {
            subarray.len() - 1
        } else {
            pivot_offset - 1
        };
        let mut prev_max = subarray[max_offset];
        // this logic is best understood with a diagram of a rotated array, e.g.:
        //
        // ------------------------------------------------------------------------
        // | 12 | 13 | 14 | 15 | 16 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 |
        // ------------------------------------------------------------------------
        //
        if max_offset < pivot_offset && insert_offset >= pivot_offset {
            subarray.copy_within(pivot_offset..insert_offset, max_offset);
            subarray[insert_offset - 1] = value;
            self.min_indexes[subarray_idx] = max_offset;
            self.min_data[subarray_idx] = subarray[max_offset];
        } else {
            subarray.copy_within(insert_offset..max_offset, insert_offset + 1);
            subarray[insert_offset] = value;
            if insert_offset == pivot_offset {
                // inserted value is new minimum for subarray
                self.min_data[subarray_idx] = value;
            }
        }
        debug_assert!(self.assert_invariants());
        let max_subarray_idx = self.min_indexes.len() - 1;
        let next_subarray_idx = subarray_idx + 1;
        let last_subarray_full = self.is_last_subarray_full();
        // now loop over all remaining subarrays, setting the min (pivot) of each to the max of its predecessor
        for (i, pivot_offset_ref) in self.min_indexes[next_subarray_idx..].iter_mut().enumerate() {
            let cur_subarray_idx = next_subarray_idx + i;
            // if the last subarray isn't full, skip it
            if cur_subarray_idx == max_subarray_idx && !last_subarray_full {
                break;
            }
            let max_offset = if *pivot_offset_ref == 0 {
                cur_subarray_idx
            } else {
                *pivot_offset_ref - 1
            };
            let max_idx = max_offset + Self::get_array_idx_from_subarray_idx(cur_subarray_idx);
            let next_max = self.data[max_idx];
            self.data[max_idx] = prev_max;
            *pivot_offset_ref = max_offset;
            self.min_data[cur_subarray_idx] = prev_max;
            prev_max = next_max;
        }
        // if the last subarray was full, append current max to a new subarray, otherwise insert max in sorted order
        if last_subarray_full {
            self.data.push(prev_max);
            self.min_indexes.push(0);
            self.min_data.push(prev_max);
        } else {
            let max_subarray_offset = Self::get_array_idx_from_subarray_idx(max_subarray_idx);
            // since `max` is guaranteed to be <= the pivot value, we always insert it at the pivot location
            debug_assert!(prev_max <= self.data[max_subarray_offset]);
            self.data.insert(max_subarray_offset, prev_max);
            self.min_data[max_subarray_idx] = prev_max;
        }
        debug_assert!(self.find_raw_index(&value).is_ok());
        debug_assert!(self.assert_invariants());
        true
    }

    /// Removes a value from the set. Returns whether the value was
    /// present in the set.
    ///
    /// This is an `O(√n)` operation.
    ///
    /// # Examples
    ///
    /// ```
    /// use rotated_array_set::RotatedArraySet;
    ///
    /// let mut set = RotatedArraySet::new();
    ///
    /// set.insert(2);
    /// assert_eq!(set.remove(&2), true);
    /// assert_eq!(set.remove(&2), false);
    /// ```
    pub fn remove(&mut self, value: &T) -> bool {
        let mut remove_idx = match self.find_raw_index(&value).ok() {
            Some(idx) => idx,
            None => return false,
        };
        let max_subarray_idx = self.min_indexes.len() - 1;
        let max_subarray_offset = Self::get_array_idx_from_subarray_idx(max_subarray_idx);
        // find subarray containing the element to remove
        let subarray_idx = Self::get_subarray_idx_from_array_idx(remove_idx);
        debug_assert!(subarray_idx <= max_subarray_idx);
        let subarray_offset = Self::get_array_idx_from_subarray_idx(subarray_idx);
        // if we're not removing an element in the last subarray, then we end up deleting its minimum,
        // which is always at the first offset since it's sorted
        let mut max_subarray_remove_idx = if subarray_idx == max_subarray_idx {
            remove_idx
        } else {
            max_subarray_offset
        };
        // if the last subarray was rotated, sort it to maintain insert invariant
        if self.is_last_subarray_full() {
            let last_min_offset = self.min_indexes[max_subarray_idx];
            // rotate left by the min offset instead of sorting
            self.data[max_subarray_offset..].rotate_left(last_min_offset);
            self.min_indexes[max_subarray_idx] = 0;
            // the remove index might change after sorting the last subarray
            if subarray_idx == max_subarray_idx {
                remove_idx = self
                    .find_raw_index(&value)
                    .expect("recalculating remove index after sorting");
                max_subarray_remove_idx = remove_idx;
            }
        }
        // if insertion point is not in last subarray, perform a "hard exchange"
        if subarray_idx < max_subarray_idx {
            // From now on, we can assume that the subarray we're removing from is full.
            let next_subarray_offset = Self::get_array_idx_from_subarray_idx(subarray_idx + 1);
            let subarray = &mut self.data[subarray_offset..next_subarray_offset];
            let pivot_offset = self.min_indexes[subarray_idx];
            let remove_offset = remove_idx - subarray_offset;
            let max_offset = if pivot_offset == 0 {
                subarray.len() - 1
            } else {
                pivot_offset - 1
            };
            // this logic is best understood with a diagram of a rotated array, e.g.:
            //
            // ------------------------------------------------------------------------
            // | 12 | 13 | 14 | 15 | 16 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 |
            // ------------------------------------------------------------------------
            //
            let mut prev_max_offset = if max_offset < pivot_offset && remove_offset >= pivot_offset
            {
                subarray.copy_within(pivot_offset..remove_offset, pivot_offset + 1);
                let new_pivot_offset = if pivot_offset == subarray.len() - 1 {
                    0
                } else {
                    pivot_offset + 1
                };
                self.min_indexes[subarray_idx] = new_pivot_offset;
                self.min_data[subarray_idx] = subarray[new_pivot_offset];
                pivot_offset
            } else {
                subarray.copy_within(remove_offset + 1..=max_offset, remove_offset);
                if remove_offset == pivot_offset {
                    self.min_data[subarray_idx] = subarray[pivot_offset];
                }
                max_offset
            };
            let next_subarray_idx = min(max_subarray_idx, subarray_idx + 1);
            // now perform an "easy exchange" in all remaining subarrays except the last,
            // setting the max of each to the min of its successor.
            for (i, pivot_offset_ref) in self.min_indexes[next_subarray_idx..max_subarray_idx]
                .iter_mut()
                .enumerate()
            {
                let cur_subarray_idx = next_subarray_idx + i;
                let cur_subarray_offset = Self::get_array_idx_from_subarray_idx(cur_subarray_idx);
                let prev_max_idx =
                    prev_max_offset + Self::get_array_idx_from_subarray_idx(cur_subarray_idx - 1);
                self.data[prev_max_idx] = self.data[cur_subarray_offset + *pivot_offset_ref];
                // the min_data array needs to be updated when the previous subarray's max offset
                // coincides with its min offset, i.e., when it is subarray 0
                if cur_subarray_idx == 1 {
                    self.min_data[0] = self.data[0];
                    debug_assert!(IsSorted::is_sorted(&mut self.min_data.iter()));
                }
                prev_max_offset = *pivot_offset_ref;
                let new_min_offset = if *pivot_offset_ref == cur_subarray_idx {
                    0
                } else {
                    *pivot_offset_ref + 1
                };
                *pivot_offset_ref = new_min_offset;
                self.min_data[cur_subarray_idx] = self.data[cur_subarray_offset + new_min_offset];
                debug_assert!(IsSorted::is_sorted(&mut self.min_data.iter()));
            }
            // now we fix up the last subarray. if it was initially full, we need to sort it to maintain the insert invariant.
            // if the removed element is in the last subarray, we just sort and remove() on the vec, updating auxiliary arrays.
            // otherwise, we copy the minimum to the max position of the previous subarray, then remove it and fix up
            // auxiliary arrays.
            let prev_max_idx =
                prev_max_offset + Self::get_array_idx_from_subarray_idx(max_subarray_idx - 1);
            // since the last subarray is always sorted, its minimum element is always on the first offset
            self.data[prev_max_idx] = self.data[max_subarray_offset];
            // the min_data array needs to be updated when the previous subarray's max offset
            // coincides with its min offset, i.e., when it is subarray 0
            if max_subarray_idx == 1 {
                self.min_data[0] = self.data[0];
                debug_assert!(IsSorted::is_sorted(&mut self.min_data.iter()));
            }
        }
        self.data.remove(max_subarray_remove_idx);
        // if last subarray is now empty, trim the auxiliary arrays
        if max_subarray_offset == self.data.len() {
            self.min_indexes.pop();
            self.min_data.pop();
        } else {
            // since the last subarray is always sorted, its minimum is always on the first offset
            self.min_data[max_subarray_idx] = self.data[max_subarray_offset];
            debug_assert!(IsSorted::is_sorted(&mut self.min_data.iter()));
        }
        debug_assert!(self.find_raw_index(&value).is_err());
        debug_assert!(self.assert_invariants());
        true
    }

    /// Removes and returns the value in the set, if any, that is equal to the given one.
    ///
    /// This is an `O(√n)` operation.
    ///
    /// # Examples
    ///
    /// ```
    /// use rotated_array_set::RotatedArraySet;
    ///
    /// let mut set: RotatedArraySet<_> = vec![1, 2, 3].into();
    /// assert_eq!(set.take(&2), Some(2));
    /// assert_eq!(set.take(&2), None);
    /// ```
    pub fn take(&mut self, value: &T) -> Option<T> {
        let ret = self.get(value).copied();
        if ret.is_some() {
            self.remove(value);
        }
        ret
    }

    /// Moves all elements from `other` into `Self`, leaving `other` empty.
    ///
    /// # Examples
    ///
    /// ```
    /// use rotated_array_set::RotatedArraySet;
    ///
    /// let mut a = RotatedArraySet::new();
    /// a.insert(1);
    /// a.insert(2);
    /// a.insert(3);
    ///
    /// let mut b = RotatedArraySet::new();
    /// b.insert(3);
    /// b.insert(4);
    /// b.insert(5);
    ///
    /// a.append(&mut b);
    ///
    /// assert_eq!(a.len(), 5);
    /// assert_eq!(b.len(), 0);
    ///
    /// assert!(a.contains(&1));
    /// assert!(a.contains(&2));
    /// assert!(a.contains(&3));
    /// assert!(a.contains(&4));
    /// assert!(a.contains(&5));
    /// ```
    pub fn append(&mut self, other: &mut Self) {
        // allocate new set and copy union into it
        let mut union: Self = self.union(other).cloned().collect();
        // empty `other`
        other.clear();
        // steal data from new set and drop data from old set
        mem::swap(self, &mut union);
    }

    /// Splits the collection into two at `value`. Returns everything after `value`,
    /// including `value` itself.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use rotated_array_set::RotatedArraySet;
    ///
    /// let mut a = RotatedArraySet::new();
    /// a.insert(1);
    /// a.insert(2);
    /// a.insert(3);
    /// a.insert(17);
    /// a.insert(41);
    ///
    /// let b = a.split_off(&3);
    ///
    /// assert_eq!(a.len(), 2);
    /// assert_eq!(b.len(), 3);
    ///
    /// assert!(a.contains(&1));
    /// assert!(a.contains(&2));
    ///
    /// assert!(b.contains(&3));
    /// assert!(b.contains(&17));
    /// assert!(b.contains(&41));
    /// ```
    pub fn split_off(&mut self, value: &T) -> Self {
        let tail = self.range((Included(value), Unbounded));
        if tail.len() == 0 {
            // if key follows everything in set, just return empty set
            Self::default()
        } else if tail.len() == self.len() {
            // if key precedes everything in set, just return moved self
            mem::replace(self, Self::default())
        } else {
            // return tail and truncate self
            let new_len = self.len() - tail.len();
            let tail_set: Self = tail.cloned().collect();
            self.truncate(new_len);
            tail_set
        }
    }

    /// Truncates the sorted sequence, keeping the first `len` elements and dropping
    /// the rest.
    ///
    /// If `len` is greater than the set's current length, this has no
    /// effect.
    ///
    /// # Examples
    ///
    /// Truncating a five-element set to two elements:
    ///
    /// ```
    /// use rotated_array_set::RotatedArraySet;
    ///
    /// let mut set: RotatedArraySet<_> = vec![1, 2, 3, 4, 5].into();
    /// set.truncate(2);
    /// assert_eq!(set, vec![1, 2].into());
    /// ```
    ///
    /// No truncation occurs when `len` is greater than the vector's current
    /// length:
    ///
    /// ```
    /// use rotated_array_set::RotatedArraySet;
    ///
    /// let mut set: RotatedArraySet<_> = vec![1, 2, 3].into();
    /// set.truncate(8);
    /// assert_eq!(set, vec![1, 2, 3].into());
    /// ```
    ///
    /// Truncating when `len == 0` is equivalent to calling the [`clear`]
    /// method.
    ///
    /// ```
    /// use rotated_array_set::RotatedArraySet;
    ///
    /// let mut set: RotatedArraySet<_> = vec![1, 2, 3].into();
    /// set.truncate(0);
    /// assert_eq!(set, vec![].into());
    /// ```
    pub fn truncate(&mut self, len: usize) {
        if len == 0 {
            self.clear();
        // if len >= self.len(), do nothing
        } else if len < self.len() {
            // logical index corresponding to truncated length
            let index = len - 1;
            // find subarray containing logical index (we don't need to translate to raw index for this)
            let subarray_idx = Self::get_subarray_idx_from_array_idx(index);
            let subarray_offset = Self::get_array_idx_from_subarray_idx(subarray_idx);
            let next_subarray_offset = if subarray_idx == self.min_indexes.len() - 1 {
                self.data.len()
            } else {
                Self::get_array_idx_from_subarray_idx(subarray_idx + 1)
            };
            let subarray = &mut self.data[subarray_offset..next_subarray_offset];
            // sort subarray and update auxiliary arrays
            let min_offset = self.min_indexes[subarray_idx];
            subarray.rotate_left(min_offset);
            self.min_indexes[subarray_idx] = 0;
            // now we can truncate the whole data array at the logical index
            self.data.truncate(len);
            // trim auxiliary arrays
            self.min_indexes.truncate(subarray_idx + 1);
            self.min_data.truncate(subarray_idx + 1);
        }
        debug_assert!(self.assert_invariants());
    }

    /// Returns the number of elements in the set.
    ///
    /// This is a constant-time operation.
    ///
    /// # Examples
    ///
    /// ```
    /// use rotated_array_set::RotatedArraySet;
    ///
    /// let mut v = RotatedArraySet::new();
    /// assert_eq!(v.len(), 0);
    /// v.insert(1);
    /// assert_eq!(v.len(), 1);
    /// ```
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// Returns `true` if the set contains no elements.
    ///
    /// This is a constant-time operation.
    ///
    /// # Examples
    ///
    /// ```
    /// use rotated_array_set::RotatedArraySet;
    ///
    /// let mut v = RotatedArraySet::new();
    /// assert!(v.is_empty());
    /// v.insert(1);
    /// assert!(!v.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// Gets a double-ended iterator that visits the values in the `RotatedArraySet` in ascending (descending) order.
    ///
    /// # Examples
    ///
    /// ```
    /// use rotated_array_set::RotatedArraySet;
    ///
    /// let set: RotatedArraySet<usize> = RotatedArraySet::new();
    /// let mut set_iter = set.iter();
    /// assert_eq!(set_iter.next(), None);
    /// ```
    ///
    /// ```
    /// use rotated_array_set::RotatedArraySet;
    ///
    /// let set: RotatedArraySet<usize> = vec![1, 2, 3].into();
    /// let mut set_iter = set.iter();
    /// assert_eq!(set_iter.next(), Some(&1));
    /// assert_eq!(set_iter.next(), Some(&2));
    /// assert_eq!(set_iter.next(), Some(&3));
    /// assert_eq!(set_iter.next(), None);
    /// ```
    ///
    /// Values returned by the iterator are returned in ascending order:
    ///
    /// ```
    /// use rotated_array_set::RotatedArraySet;
    ///
    /// let set: RotatedArraySet<usize> = vec![3, 1, 2].into();
    /// let mut set_iter = set.iter();
    /// assert_eq!(set_iter.next(), Some(&1));
    /// assert_eq!(set_iter.next(), Some(&2));
    /// assert_eq!(set_iter.next(), Some(&3));
    /// assert_eq!(set_iter.next(), None);
    /// ```
    pub fn iter(&self) -> Iter<'_, T> {
        Iter::new(Range::new(self))
    }

    /// Constructs a double-ended iterator over a sub-range of elements in the set.
    /// The simplest way is to use the range syntax `min..max`, thus `range(min..max)` will
    /// yield elements from `min` (inclusive) to `max` (exclusive).
    /// The range may also be entered as `(Bound<T>, Bound<T>)`, so for example
    /// `range((Excluded(4), Included(10)))` will yield a left-exclusive, right-inclusive
    /// range from 4 to 10.
    ///
    /// # Examples
    ///
    /// ```
    /// use rotated_array_set::RotatedArraySet;
    /// use std::ops::Bound::Included;
    ///
    /// let mut set = RotatedArraySet::new();
    /// set.insert(3);
    /// set.insert(5);
    /// set.insert(8);
    /// for &elem in set.range((Included(&4), Included(&8))) {
    ///     println!("{}", elem);
    /// }
    /// assert_eq!(Some(&5), set.range(4..).next());
    /// ```
    ///
    /// ```
    /// use rotated_array_set::RotatedArraySet;
    /// use std::ops::Bound::{Included, Excluded};
    ///
    /// let mut set: RotatedArraySet<_> = (1..10).collect();
    /// let range: Vec<_> = set.range((Included(&4), Excluded(&8))).cloned().collect();
    /// assert_eq!(range, vec![4, 5, 6, 7]);
    /// ```
    pub fn range<R>(&self, range: R) -> Iter<'_, T>
    where
        R: RangeBounds<T>,
    {
        let range = self.get_range(range);
        Iter::new(range)
    }

    fn get_range<R>(&self, range: R) -> Range<'_, T>
    where
        R: RangeBounds<T>,
    {
        match (range.start_bound(), range.end_bound()) {
            (Excluded(s), Excluded(e)) if s == e => {
                panic!("range start and end are equal and excluded in RotatedArraySet")
            }
            (Included(s), Included(e))
            | (Included(s), Excluded(e))
            | (Excluded(s), Included(e))
            | (Excluded(s), Excluded(e))
                if s > e =>
            {
                panic!("range start is greater than range end in RotatedArraySet")
            }
            _ => {}
        };
        let start_index_inclusive = match range.start_bound() {
            Unbounded => 0,
            Included(s) => match self.find_raw_index(s) {
                Ok(index) => index,
                Err(index) => index,
            },
            Excluded(s) => match self.find_raw_index(s) {
                Ok(index) => index + 1,
                Err(index) => index,
            },
        };
        let end_index_exclusive = match range.end_bound() {
            Unbounded => self.len(),
            Included(e) => match self.find_raw_index(e) {
                Ok(index) => index + 1,
                Err(index) => index,
            },
            Excluded(e) => match self.find_raw_index(e) {
                Ok(index) => index,
                Err(index) => index,
            },
        };
        Range::with_bounds(self, start_index_inclusive, end_index_exclusive)
    }

    /// Visits the values representing the difference,
    /// i.e., the values that are in `self` but not in `other`,
    /// in ascending order.
    ///
    /// # Examples
    ///
    /// ```
    /// use rotated_array_set::RotatedArraySet;
    ///
    /// let mut a = RotatedArraySet::new();
    /// a.insert(1);
    /// a.insert(2);
    ///
    /// let mut b = RotatedArraySet::new();
    /// b.insert(2);
    /// b.insert(3);
    ///
    /// let diff: Vec<_> = a.difference(&b).cloned().collect();
    /// assert_eq!(diff, [1]);
    /// ```
    pub fn difference<'a>(&'a self, other: &'a RotatedArraySet<T>) -> Difference<'a, T> {
        Difference {
            self_iter: self.iter(),
            other_set: other,
        }
    }

    /// Visits the values representing the symmetric difference,
    /// i.e., the values that are in `self` or in `other` but not in both,
    /// in ascending order.
    ///
    /// # Examples
    ///
    /// ```
    /// use rotated_array_set::RotatedArraySet;
    ///
    /// let mut a = RotatedArraySet::new();
    /// a.insert(1);
    /// a.insert(2);
    ///
    /// let mut b = RotatedArraySet::new();
    /// b.insert(2);
    /// b.insert(3);
    ///
    /// let sym_diff: Vec<_> = a.symmetric_difference(&b).cloned().collect();
    /// assert_eq!(sym_diff, [1, 3]);
    /// ```
    pub fn symmetric_difference<'a>(
        &'a self,
        other: &'a RotatedArraySet<T>,
    ) -> SymmetricDifference<'a, T> {
        SymmetricDifference {
            a: self.iter().peekable(),
            b: other.iter().peekable(),
        }
    }

    /// Visits the values representing the intersection,
    /// i.e., the values that are both in `self` and `other`,
    /// in ascending order.
    ///
    /// # Examples
    ///
    /// ```
    /// use rotated_array_set::RotatedArraySet;
    ///
    /// let mut a = RotatedArraySet::new();
    /// a.insert(1);
    /// a.insert(2);
    ///
    /// let mut b = RotatedArraySet::new();
    /// b.insert(2);
    /// b.insert(3);
    ///
    /// let intersection: Vec<_> = a.intersection(&b).cloned().collect();
    /// assert_eq!(intersection, [2]);
    /// ```
    pub fn intersection<'a>(&'a self, other: &'a RotatedArraySet<T>) -> Intersection<'a, T> {
        let (small, other) = if self.len() <= other.len() {
            (self, other)
        } else {
            (other, self)
        };
        // Iterate the small set, searching for matches in the large set.
        Intersection {
            small_iter: small.iter(),
            large_set: other,
        }
    }

    /// Visits the values representing the union,
    /// i.e., all the values in `self` or `other`, without duplicates,
    /// in ascending order.
    ///
    /// # Examples
    ///
    /// ```
    /// use rotated_array_set::RotatedArraySet;
    ///
    /// let mut a = RotatedArraySet::new();
    /// a.insert(1);
    /// a.insert(2);
    ///
    /// let mut b = RotatedArraySet::new();
    /// b.insert(2);
    /// b.insert(3);
    ///
    /// let union: Vec<_> = a.union(&b).cloned().collect();
    /// assert_eq!(union, [1, 2, 3]);
    /// ```
    pub fn union<'a>(&'a self, other: &'a RotatedArraySet<T>) -> Union<'a, T> {
        Union {
            a: self.iter().peekable(),
            b: other.iter().peekable(),
        }
    }

    fn integer_sum(n: usize) -> usize {
        // I learned this from a 10-year-old named Gauss
        (n * (n + 1)) / 2
    }

    fn integer_sum_inverse(n: usize) -> usize {
        // y = (x * (x + 1)) / 2
        // x = (sqrt(8 * y + 1) - 1) / 2
        let floaty = ((n as f64 * 8.0 + 1.0).sqrt() - 1.0) / 2.0;
        let tmp = floaty as usize;
        let sum = Self::integer_sum(tmp);
        if sum <= n {
            tmp
        } else {
            tmp - 1
        }
    }

    fn get_subarray_idx_from_array_idx(idx: usize) -> usize {
        if idx == 0 {
            0
        } else {
            Self::integer_sum_inverse(idx)
        }
    }

    fn get_array_idx_from_subarray_idx(idx: usize) -> usize {
        if idx == 0 {
            0
        } else {
            Self::integer_sum(idx)
        }
    }

    fn is_last_subarray_full(&self) -> bool {
        self.data.len() == Self::get_array_idx_from_subarray_idx(self.min_indexes.len())
    }

    // Returns either (raw) index of element if it exists, or (raw) insertion point if it doesn't exist.
    fn find_raw_index(&self, value: &T) -> Result<usize, usize> {
        if self.data.is_empty() {
            return Err(0);
        }
        // find two candidate subarrays by binary searching self.min_data,
        // then compare value to max value of first subarray, if it's smaller
        // then binary search first subarray, otherwise second subarray
        // TODO: actually we only need to binary search first subarray, max
        // comparison is just to determine insertion point (to preserve invariant
        // that we never insert element into a subarray greater than its current max).
        // if element greater than max of first subarray but less than min of
        // second subarray, just return insertion point on min index of second subarray.
        debug_assert!(self.assert_invariants());
        match self.min_data.binary_search(value) {
            Ok(idx) => {
                // `value` is located directly on a pivot index
                let found_idx = Self::get_array_idx_from_subarray_idx(idx) + self.min_indexes[idx];
                debug_assert!(found_idx < self.len());
                Ok(found_idx)
            }
            Err(idx) => {
                // The element might be in either the subarray corresponding to the insertion point,
                // or in its predecessor; compare to max value of predecessor to decide.
                // A special case is when the insertion point is after the last subarray and the last subarray isn't full.
                // In that case, we want to insert into the existing last subarray, not create a new one.
                let subarray_idx = if idx == 0 {
                    0
                } else if idx == self.min_indexes.len() && !self.is_last_subarray_full() {
                    // partially full final subarray
                    idx - 1
                } else {
                    // we can assume the predecessor subarray is full
                    let prev_max_idx = if self.min_indexes[idx - 1] == 0 {
                        Self::get_array_idx_from_subarray_idx(idx) - 1
                    } else {
                        Self::get_array_idx_from_subarray_idx(idx - 1) + self.min_indexes[idx - 1]
                            - 1
                    };
                    if *value <= self.data[prev_max_idx] {
                        idx - 1
                    } else {
                        idx
                    }
                };
                let subarray_offset = Self::get_array_idx_from_subarray_idx(subarray_idx);
                // we may need to create a new subarray to insert this element
                debug_assert!(subarray_offset <= self.data.len());
                if subarray_offset == self.data.len() {
                    return Err(subarray_offset);
                }
                // if our last subarray is truncated, then account for that
                let next_subarray_offset = if subarray_idx == self.min_indexes.len() - 1 {
                    self.data.len()
                } else {
                    Self::get_array_idx_from_subarray_idx(subarray_idx + 1)
                };
                // split subarray into two slices separated by pivot,
                // and search both separately.
                let subarray = &self.data[subarray_offset..next_subarray_offset];
                let pivot_offset = self.min_indexes[subarray_idx];
                let subarray_pivot = subarray_offset + pivot_offset;
                let (left, right) = subarray.split_at(pivot_offset);
                debug_assert!(
                    IsSorted::is_sorted(&mut left.iter()) && IsSorted::is_sorted(&mut right.iter())
                );
                match (left.binary_search(value), right.binary_search(value)) {
                    (Ok(idx), _) => Ok(subarray_offset + idx),
                    (_, Ok(idx)) => Ok(subarray_pivot + idx),
                    // if right insertion point is past right subarray, and left subarray is not empty, then true insertion point must be on left
                    (Err(left_idx), Err(right_idx))
                        if right_idx == right.len() && !left.is_empty() =>
                    {
                        Err(subarray_offset + left_idx)
                    }
                    // if right insertion point is within right subarray, or left subarray is empty, then true insertion point must be on right
                    (Err(_left_idx), Err(right_idx))
                        if right_idx < right.len() || left.is_empty() =>
                    {
                        Err(subarray_pivot + right_idx)
                    }
                    (Err(_), Err(_)) => unreachable!(),
                }
            }
        }
    }

    #[inline(always)]
    fn assert_invariants(&self) -> bool {
        // assert order
        assert!(IsSorted::is_sorted(&mut self.min_data.iter()));
        let mut min_data_dedup = self.min_data.clone();
        min_data_dedup.dedup();
        // assert uniqueness
        assert!(self.min_data[..] == min_data_dedup[..]);
        // assert index of each subarray's minimum lies within the subarray
        assert!(self
            .min_indexes
            .iter()
            .enumerate()
            .all(|(idx, &offset)| offset <= idx));
        // assert min_data is properly synchronized with min_indexes and self.data
        assert!(self
            .min_indexes
            .iter()
            .enumerate()
            .all(|(idx, &offset)| self.min_data[idx]
                == self.data[Self::get_array_idx_from_subarray_idx(idx) + offset]));
        // assert min_indexes holds the index of the actual minimum of each subarray
        for i in 0..self.min_indexes.len() {
            let subarray_begin_idx = Self::get_array_idx_from_subarray_idx(i);
            let subarray_end_idx = min(
                self.data.len(),
                Self::get_array_idx_from_subarray_idx(i + 1),
            );
            let subarray = &self.data[subarray_begin_idx..subarray_end_idx];
            let min_idx = subarray
                .iter()
                .enumerate()
                .min_by(|&(_, v1), &(_, v2)| v1.cmp(v2))
                .unwrap()
                .0;
            assert!(min_idx == self.min_indexes[i]);
        }
        true
    }

    // given data array, initialize auxiliary arrays
    fn init(&mut self) {
        debug_assert!(self.min_indexes.is_empty() && self.min_data.is_empty());
        if !self.data.is_empty() {
            self.data.sort_unstable(); // don't want to allocate
            let last_subarray_idx = Self::get_subarray_idx_from_array_idx(self.data.len() - 1);
            self.min_indexes = vec![0; last_subarray_idx + 1];
            for subarray_idx in 0..=last_subarray_idx {
                let subarray_offset = Self::get_array_idx_from_subarray_idx(subarray_idx);
                self.min_data.push(self.data[subarray_offset]);
            }
        }
    }
}

impl<T> PartialEq for RotatedArraySet<T>
where
    T: Ord + Copy + Default + Debug,
{
    fn eq(&self, other: &Self) -> bool {
        if self.len() != other.len() {
            return false;
        }
        for i in 0..self.len() {
            if self.select(i).unwrap() != other.select(i).unwrap() {
                return false;
            }
        }
        true
    }
}

impl<T> Eq for RotatedArraySet<T> where T: Ord + Copy + Default + Debug {}

impl<T> Hash for RotatedArraySet<T>
where
    T: Ord + Copy + Default + Debug + Hash,
{
    fn hash<H: Hasher>(&self, state: &mut H) {
        for i in 0..self.len() {
            self.select(i).hash(state);
        }
    }
}

impl<'a, T> Iterator for Iter<'a, T>
where
    T: Ord + Copy + Default + Debug,
{
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.len() == 0 || self.next_index > self.next_rev_index {
            None
        } else {
            let current = self.range.at(self.next_index);
            self.next_index += 1;
            debug_assert!(self.assert_invariants());
            current
        }
    }

    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        self.next_index = min(self.next_index + n, self.len());
        let ret = if self.len() == 0 || self.next_index > self.next_rev_index {
            None
        } else {
            let nth = self.range.at(self.next_index);
            self.next_index += 1;
            nth
        };
        debug_assert!(self.assert_invariants());
        ret
    }

    fn count(self) -> usize {
        self.len() - self.next_index
    }

    fn last(self) -> Option<Self::Item> {
        if self.len() == 0 {
            None
        } else {
            self.range.at(self.len() - 1)
        }
    }

    fn max(self) -> Option<Self::Item> {
        if self.len() == 0 {
            None
        } else {
            self.range.at(self.len() - 1)
        }
    }

    fn min(self) -> Option<Self::Item> {
        self.range.at(0)
    }

    // FIXME: uncomment when Iterator::is_sorted is stabilized
    // fn is_sorted(self) -> bool {
    //     true
    // }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining_count = self.len() - self.next_index;
        (remaining_count, Some(remaining_count))
    }
}

impl<'a, T> DoubleEndedIterator for Iter<'a, T>
where
    T: Ord + Copy + Default + Debug,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.len() == 0 || self.next_rev_index < self.next_index {
            None
        } else {
            let current = self.range.at(self.next_rev_index);
            // We can't decrement next_rev_index past 0, so we cheat and move next_index
            // ahead instead. That works since next() must return None once next_rev_index
            // has crossed next_index.
            if self.next_rev_index == 0 {
                self.next_index += 1;
            } else {
                self.next_rev_index -= 1;
            }
            debug_assert!(self.assert_invariants());
            current
        }
    }

    fn nth_back(&mut self, n: usize) -> Option<Self::Item> {
        self.next_rev_index = self.next_rev_index.saturating_sub(n);
        let ret = if self.len() == 0 || self.next_rev_index < self.next_index {
            None
        } else {
            let nth = self.range.at(self.next_rev_index);
            // We can't decrement next_rev_index past 0, so we cheat and move next_index
            // ahead instead. That works since next() must return None once next_rev_index
            // has crossed next_index.
            if self.next_rev_index == 0 {
                self.next_index += 1;
            } else {
                self.next_rev_index -= 1;
            }
            nth
        };
        debug_assert!(self.assert_invariants());
        ret
    }
}

impl<T> ExactSizeIterator for Iter<'_, T>
where
    T: Ord + Copy + Default + Debug,
{
    fn len(&self) -> usize {
        self.range.len()
    }
}

impl<T> FusedIterator for Iter<'_, T> where T: Ord + Copy + Default + Debug {}

impl<'a, T> IntoIterator for &'a RotatedArraySet<T>
where
    T: Ord + Copy + Default + Debug,
{
    type Item = &'a T;
    type IntoIter = Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<T> IntoIterator for RotatedArraySet<T>
where
    T: Ord + Copy + Default + Debug,
{
    type Item = T;
    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter {
            vec: self.into(),
            next_index: 0,
        }
    }
}

impl<'a, T> Iterator for IntoIter<T>
where
    T: Ord + Copy + Default + Debug,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.next_index == self.vec.len() {
            None
        } else {
            let current = self.vec[self.next_index];
            self.next_index += 1;
            debug_assert!(self.next_index <= self.vec.len());
            Some(current)
        }
    }
}

/// From https://doc.rust-lang.org/src/alloc/collections/btree/set.rs.html
/// Compares `x` and `y`, but return `short` if x is None and `long` if y is None
fn cmp_opt<T: Ord>(x: Option<&T>, y: Option<&T>, short: Ordering, long: Ordering) -> Ordering {
    match (x, y) {
        (None, _) => short,
        (_, None) => long,
        (Some(x1), Some(y1)) => x1.cmp(y1),
    }
}

impl<'a, T> Iterator for Difference<'a, T>
where
    T: Ord + Copy + Default + Debug,
{
    type Item = &'a T;

    fn next(&mut self) -> Option<&'a T> {
        // Just use a simple lookup from `self_iter` to `other_set` for now,
        // later add a proper linear merge for size ratios close to 1 if benchmarks warrant.
        // (A point lookup has much better worst-case performance than linear merge.)
        // NB: For a single algorithm optimal over all size ratios, see
        // "A simple algorithm for merging two disjoint linearly-ordered sets".
        loop {
            let self_next = self.self_iter.next()?;
            if !self.other_set.contains(&self_next) {
                return Some(self_next);
            }
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let (self_len, other_len) = (self.self_iter.len(), self.other_set.len());
        (self_len.saturating_sub(other_len), Some(self_len))
    }
}

impl<T> FusedIterator for Difference<'_, T> where T: Ord + Copy + Default + Debug {}

impl<'a, T> Iterator for SymmetricDifference<'a, T>
where
    T: Ord + Copy + Default + Debug,
{
    type Item = &'a T;

    fn next(&mut self) -> Option<&'a T> {
        loop {
            match cmp_opt(self.a.peek(), self.b.peek(), Greater, Less) {
                Less => return self.a.next(),
                Equal => {
                    self.a.next();
                    self.b.next();
                }
                Greater => return self.b.next(),
            }
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, Some(self.a.len() + self.b.len()))
    }
}

impl<T> FusedIterator for SymmetricDifference<'_, T> where T: Ord + Copy + Default + Debug {}

impl<'a, T> Iterator for Intersection<'a, T>
where
    T: Ord + Copy + Default + Debug,
{
    type Item = &'a T;

    fn next(&mut self) -> Option<&'a T> {
        // Just use a simple lookup from `self_iter` to `other_set` for now,
        // later add a proper linear merge for size ratios close to 1 if benchmarks warrant.
        // (A point lookup has much better worst-case performance than linear merge.)
        // NB: For a single algorithm optimal over all size ratios, see
        // "A simple algorithm for merging two disjoint linearly-ordered sets".
        loop {
            let small_next = self.small_iter.next()?;
            if self.large_set.contains(&small_next) {
                return Some(small_next);
            }
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let min_len = self.small_iter.len();
        (0, Some(min_len))
    }
}

impl<T> FusedIterator for Intersection<'_, T> where T: Ord + Copy + Default + Debug {}

impl<'a, T> Iterator for Union<'a, T>
where
    T: Ord + Copy + Default + Debug,
{
    type Item = &'a T;

    fn next(&mut self) -> Option<&'a T> {
        match cmp_opt(self.a.peek(), self.b.peek(), Greater, Less) {
            Less => self.a.next(),
            Equal => {
                self.b.next();
                self.a.next()
            }
            Greater => self.b.next(),
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let a_len = self.a.len();
        let b_len = self.b.len();
        (max(a_len, b_len), Some(a_len + b_len))
    }
}

impl<T> FusedIterator for Union<'_, T> where T: Ord + Copy + Default + Debug {}

impl<'a, T> From<&'a [T]> for RotatedArraySet<T>
where
    T: Ord + Copy + Default + Debug,
{
    fn from(slice: &[T]) -> Self {
        let mut this = RotatedArraySet {
            data: slice.to_vec(),
            min_indexes: Vec::new(),
            min_data: Vec::new(),
        };
        this.init();
        this
    }
}

impl<T> From<Vec<T>> for RotatedArraySet<T>
where
    T: Ord + Copy + Default + Debug,
{
    fn from(vec: Vec<T>) -> Self {
        let mut this = RotatedArraySet {
            data: vec,
            min_indexes: Vec::new(),
            min_data: Vec::new(),
        };
        this.init();
        this
    }
}

impl<T> Into<Vec<T>> for RotatedArraySet<T>
where
    T: Ord + Copy + Default + Debug,
{
    fn into(mut self) -> Vec<T> {
        // sort the data array in-place and steal it from self
        for (i, &pivot_offset) in self.min_indexes.iter().enumerate() {
            let subarray_start_idx = Self::get_array_idx_from_subarray_idx(i);
            let subarray_len = if i == self.min_indexes.len() - 1 {
                self.data.len() - subarray_start_idx
            } else {
                i + 1
            };
            let subarray_end_idx = subarray_start_idx + subarray_len;
            let subarray = &mut self.data[subarray_start_idx..subarray_end_idx];
            // sort subarray in-place
            subarray.rotate_left(pivot_offset);
        }
        // steal data array
        self.data
    }
}

impl<T> FromIterator<T> for RotatedArraySet<T>
where
    T: Ord + Copy + Default + Debug,
{
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut this = RotatedArraySet {
            data: Vec::from_iter(iter.into_iter()),
            min_indexes: Vec::new(),
            min_data: Vec::new(),
        };
        this.init();
        this
    }
}

impl<T> Default for RotatedArraySet<T>
where
    T: Ord + Copy + Default + Debug,
{
    fn default() -> RotatedArraySet<T> {
        RotatedArraySet::new()
    }
}

#[cfg(test)]
mod test {
    use super::RotatedArraySet;
    use proptest::prelude::*;

    fn assert_sum_invariant(n: usize) -> Result<(), TestCaseError> {
        let sum = RotatedArraySet::<u8>::integer_sum(n);
        let inv = RotatedArraySet::<u8>::integer_sum_inverse(sum);
        prop_assert_eq!(n, inv);
        Ok(())
    }

    fn assert_inverse_invariant(n: usize) -> Result<(), TestCaseError> {
        let inv = RotatedArraySet::<u8>::integer_sum_inverse(n);
        let sum_lower = RotatedArraySet::<u8>::integer_sum(inv);
        let sum_upper = RotatedArraySet::<u8>::integer_sum(inv + 1);
        prop_assert!(sum_lower <= n);
        prop_assert!(n < sum_upper);
        Ok(())
    }

    prop_compose! {
        /// generates integer_sum(?) - 1, integer_sum(?), and integer_sum(?) + 1.
        fn inverse_boundary()(n in 0..(((usize::MAX/2) as f64).sqrt() as usize), d in 0usize..3) -> usize {
            RotatedArraySet::<u8>::integer_sum(n)
                .wrapping_add(d)
                .wrapping_sub(1)
        }
    }

    proptest! {
        #[test]
        fn take_sum_and_inverse(n in 0..(((usize::MAX/2) as f64).sqrt() as usize)) {
            assert_sum_invariant(n)?
        }

        #[test]
        fn take_inverse_and_sum(n in 0..usize::MAX/2) {
            assert_inverse_invariant(n)?
        }

        #[test]
        fn take_inverse_and_sum_on_inverse_boundary(n in inverse_boundary()) {
            assert_inverse_invariant(n)?
        }
    }
}
