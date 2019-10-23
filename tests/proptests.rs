// adapted from https://github.com/ssomers/rust_bench_btreeset_intersection/blob/master/src/tests/set.rs
extern crate proptest;
use self::proptest::prelude::*;
use sorted_vec::SortedVec;
use std::collections::BTreeSet;
use std::cmp::min;

fn assert_difference<'a, I: Iterator<Item = &'a u8>>(
    mut it: I,
    s1: &'a SortedVec<u8>,
    s2: &'a SortedVec<u8>,
) -> Result<(), TestCaseError> {
    let mut count: usize = 0;
    let mut previous: i32 = -1;
    while let Some(&elt) = it.next() {
        prop_assert!(s1.contains(&elt));
        prop_assert!(!s2.contains(&elt));
        prop_assert!(i32::from(elt) > previous);
        count += 1;
        previous = i32::from(elt);
    }
    for _ in 0..42 {
        prop_assert!(it.next().is_none()); // it's announced to be a fused iterator
    }
    prop_assert_eq!(count, s1.iter().filter(|elt| !s2.contains(elt)).count());
    Ok(())
}

fn assert_intersection<'a, I: Iterator<Item = &'a u8>>(
    mut it: I,
    s1: &'a SortedVec<u8>,
    s2: &'a SortedVec<u8>,
) -> Result<(), TestCaseError> {
    let mut count: usize = 0;
    let mut previous: i32 = -1;
    while let Some(&elt) = it.next() {
        prop_assert!(s1.contains(&elt));
        prop_assert!(s2.contains(&elt));
        prop_assert!(i32::from(elt) > previous);
        count += 1;
        previous = i32::from(elt);
    }
    for _ in 0..42 {
        prop_assert!(it.next().is_none()); // it's announced to be a fused iterator
    }
    prop_assert_eq!(count, s1.iter().filter(|elt| s2.contains(elt)).count());
    Ok(())
}

fn assert_symmetric_difference<'a, I: Iterator<Item = &'a u8>>(
    mut it: I,
    s1: &'a SortedVec<u8>,
    s2: &'a SortedVec<u8>,
) -> Result<(), TestCaseError> {
    let mut count: usize = 0;
    let mut previous: i32 = -1;
    while let Some(&elt) = it.next() {
        prop_assert_eq!(s1.contains(&elt), !s2.contains(&elt));
        prop_assert!(i32::from(elt) > previous);
        count += 1;
        previous = i32::from(elt);
    }
    for _ in 0..42 {
        prop_assert!(it.next().is_none()); // it's announced to be a fused iterator
    }
    prop_assert_eq!(
        count,
        s1.len() + s2.len() - 2 * s1.iter().filter(|elt| s2.contains(elt)).count()
    );
    Ok(())
}

fn assert_union<'a, I: Iterator<Item = &'a u8>>(
    mut it: I,
    s1: &'a SortedVec<u8>,
    s2: &'a SortedVec<u8>,
) -> Result<(), TestCaseError> {
    let mut count: usize = 0;
    let mut previous: i32 = -1;
    while let Some(&elt) = it.next() {
        prop_assert!(s1.contains(&elt) || s2.contains(&elt));
        prop_assert!(i32::from(elt) > previous);
        count += 1;
        previous = i32::from(elt);
    }
    for _ in 0..42 {
        prop_assert!(it.next().is_none()); // it's announced to be a fused iterator
    }
    prop_assert_eq!(
        count,
        s1.len() + s2.len() - s1.iter().filter(|elt| s2.contains(elt)).count()
    );
    Ok(())
}

prop_compose! {
    fn arbitrary_instance()
                    (set: BTreeSet<u8>)
                    -> SortedVec<u8>
    {
        set.iter().cloned().collect()
    }
}

// note that we can return an index up to len() inclusive.
// this is necessary to provide a valid range to the RNG for empty instances.
prop_compose! {
    fn arbitrary_instance_with_index()
                    (set in any::<BTreeSet<u8>>())
                    (index in 0..=set.len(), set in Just(set))
                    -> (SortedVec<u8>, usize)
    {
        (set.iter().cloned().collect(), index)
    }
}

prop_compose! {
    fn aligned_ranges()
                     (mut s1 in arbitrary_instance(),
                      mut s2 in arbitrary_instance())
                     -> (SortedVec<u8>, SortedVec<u8>)
    {
        s1.insert(u8::min_value());
        s2.insert(u8::min_value());
        s1.insert(u8::max_value());
        s2.insert(u8::max_value());
        (s1, s2)
    }
}

prop_compose! {
    fn left_aligned_ranges()
                          (mut s1 in arbitrary_instance(),
                           mut s2 in arbitrary_instance())
                          -> (SortedVec<u8>, SortedVec<u8>)
    {
        s1.insert(u8::min_value());
        s2.insert(u8::min_value());
        (s1, s2)
    }
}

prop_compose! {
    fn right_aligned_ranges()
                           (mut s1 in arbitrary_instance(),
                            mut s2 in arbitrary_instance())
                           -> (SortedVec<u8>, SortedVec<u8>)
    {
        s1.insert(u8::max_value());
        s2.insert(u8::max_value());
        (s1, s2)
    }
}

prop_compose! {
    fn disjoint_ranges()
                      (mut s1 in arbitrary_instance(),
                       right_then_left: bool)
                      -> (SortedVec<u8>, SortedVec<u8>)
    {
        let split = (u8::max_value() - u8::min_value()) / 2;
        let mut s2 = s1.split_off(&split);
        s1.insert(u8::min_value());
        s2.insert(u8::max_value());
        if right_then_left { (s2, s1) } else { (s1, s2) }
    }
}

prop_compose! {
    fn touching_ranges()
                      (mut s1 in arbitrary_instance(),
                       right_then_left: bool)
                      -> (SortedVec<u8>, SortedVec<u8>)
    {
        let split = (u8::max_value() - u8::min_value()) / 2;
        let mut s2 = s1.split_off(&split);
        s1.insert(split);
        s2.insert(split);
        if right_then_left { (s2, s1) } else { (s1, s2) }
    }
}

proptest! {
    #[test]
    fn difference_arbitrary(s1 in arbitrary_instance(), s2 in arbitrary_instance()) {
        assert_difference(SortedVec::difference(&s1, &s2), &s1, &s2)?
    }

    #[test]
    fn difference_aligned_left((s1, s2) in left_aligned_ranges()) {
        assert_difference(SortedVec::difference(&s1, &s2), &s1, &s2)?
    }

    #[test]
    fn difference_aligned_right((s1, s2) in right_aligned_ranges()) {
        assert_difference(SortedVec::difference(&s1, &s2), &s1, &s2)?
    }

    #[test]
    fn difference_aligned_both((s1, s2) in aligned_ranges()) {
        assert_difference(SortedVec::difference(&s1, &s2), &s1, &s2)?
    }

    #[test]
    fn difference_disjoint((s1, s2) in disjoint_ranges()) {
        assert_difference(SortedVec::difference(&s1, &s2), &s1, &s2)?
    }

    #[test]
    fn difference_touching((s1, s2) in touching_ranges()) {
        assert_difference(SortedVec::difference(&s1, &s2), &s1, &s2)?
    }

    #[test]
    fn intersection_arbitrary(s1 in arbitrary_instance(), s2 in arbitrary_instance()) {
        assert_intersection(SortedVec::intersection(&s1, &s2), &s1, &s2)?
    }

    #[test]
    fn intersection_aligned_left((s1, s2) in left_aligned_ranges()) {
        assert_intersection(SortedVec::intersection(&s1, &s2), &s1, &s2)?
    }

    #[test]
    fn intersection_aligned_right((s1, s2) in right_aligned_ranges()) {
        assert_intersection(SortedVec::intersection(&s1, &s2), &s1, &s2)?
    }

    #[test]
    fn intersection_aligned_both((s1, s2) in aligned_ranges()) {
        assert_intersection(SortedVec::intersection(&s1, &s2), &s1, &s2)?
    }

    #[test]
    fn intersection_disjoint1((s1, s2) in disjoint_ranges()) {
        assert_intersection(SortedVec::intersection(&s1, &s2), &s1, &s2)?
    }

    #[test]
    fn intersection_touching((s2, s1) in touching_ranges()) {
        assert_intersection(SortedVec::intersection(&s1, &s2), &s1, &s2)?
    }

    #[test]
    fn symmetric_difference_arbitrary(s1 in arbitrary_instance(), s2 in arbitrary_instance()) {
        assert_symmetric_difference(SortedVec::symmetric_difference(&s1, &s2), &s1, &s2)?
    }

    #[test]
    fn symmetric_difference_aligned_left((s1, s2) in left_aligned_ranges()) {
        assert_symmetric_difference(SortedVec::symmetric_difference(&s1, &s2), &s1, &s2)?
    }

    #[test]
    fn symmetric_difference_aligned_right((s1, s2) in right_aligned_ranges()) {
        assert_symmetric_difference(SortedVec::symmetric_difference(&s1, &s2), &s1, &s2)?
    }

    #[test]
    fn symmetric_difference_aligned_both((s1, s2) in aligned_ranges()) {
        assert_symmetric_difference(SortedVec::symmetric_difference(&s1, &s2), &s1, &s2)?
    }

    #[test]
    fn symmetric_difference_disjoint1((s1, s2) in disjoint_ranges()) {
        assert_symmetric_difference(SortedVec::symmetric_difference(&s1, &s2), &s1, &s2)?
    }

    #[test]
    fn symmetric_difference_touching((s2, s1) in touching_ranges()) {
        assert_symmetric_difference(SortedVec::symmetric_difference(&s1, &s2), &s1, &s2)?
    }

    #[test]
    fn union_arbitrary(s1 in arbitrary_instance(), s2 in arbitrary_instance()) {
        assert_union(SortedVec::union(&s1, &s2), &s1, &s2)?
    }

    #[test]
    fn union_aligned_left((s1, s2) in left_aligned_ranges()) {
        assert_union(SortedVec::union(&s1, &s2), &s1, &s2)?
    }

    #[test]
    fn union_aligned_right((s1, s2) in right_aligned_ranges()) {
        assert_union(SortedVec::union(&s1, &s2), &s1, &s2)?
    }

    #[test]
    fn union_aligned_both((s1, s2) in aligned_ranges()) {
        assert_union(SortedVec::union(&s1, &s2), &s1, &s2)?
    }

    #[test]
    fn union_disjoint1((s1, s2) in disjoint_ranges()) {
        assert_union(SortedVec::union(&s1, &s2), &s1, &s2)?
    }

    #[test]
    fn union_touching((s2, s1) in touching_ranges()) {
        assert_union(SortedVec::union(&s1, &s2), &s1, &s2)?
    }

    #[test]
    fn insert_remove(mut s in arbitrary_instance(), v: u8) {
        let c = s.contains(&v);
        prop_assert_ne!(c, s.insert(v));
        prop_assert!(s.remove(&v));
    }

    #[test]
    fn rank_select(mut s in arbitrary_instance(), v1: u8) {
        s.insert(v1);
        let i = s.rank(&v1).unwrap();
        let v2 = *s.select(i).unwrap();
        prop_assert_eq!(v1, v2);
    }

    #[test]
    fn compare_iter(s in arbitrary_instance()) {
        let iter = s.iter();
        for (i, &v) in iter.enumerate() {
            prop_assert_eq!(*s.select(i).unwrap(), v);
        }
    }

    #[test]
    fn compare_into_iter(s in arbitrary_instance()) {
        let mut iter = s.clone().into_iter();
        for i in 0..s.len() {
            prop_assert_eq!(*s.select(i).unwrap(), iter.next().unwrap());
        }
    }

    #[test]
    fn test_iter_overrides((s, i) in arbitrary_instance_with_index()) {
        let len = s.len();
        let index = if len > 0 { min(i, len - 1) } else { 0 };
        let last_index = if len > 0 { len - 1 } else { 0 };
        let iter = s.iter();
        prop_assert_eq!(iter.min(), s.select(0));
        prop_assert_eq!(iter.max(), s.select(last_index));
        prop_assert_eq!(iter.last(), s.select(last_index));
        prop_assert_eq!(iter.count(), len);
        let mut iter_nth = iter;
        prop_assert_eq!(iter_nth.nth(index), s.select(index));
        let mut iter_nth_back = iter;
        prop_assert_eq!(iter_nth_back.nth_back(index), s.select(last_index - index)
        );
        let mut iter_mut = s.iter();
        for j in 0..(len / 2) {
            prop_assert_eq!(iter_mut.next(), s.select(j));
            prop_assert_eq!(iter_mut.next_back(), s.select(last_index - j));
        }
        iter_mut.next();
        iter_mut.next_back();
        prop_assert!(iter_mut.next().is_none());
    }
}
