# sorted-vec

## A 2-Level Rotated Array Implemented in Rust
This repository contains implementations, unit tests, and benchmark code for the "2-level rotated array" structure, first published in Munro and Suwanda's 1979 paper <a href="https://doi.org/10.1016%2F0022-0000%2880%2990037-9">"Implicit Data Structures for Fast Search and Update"</a> (which also introduced the much better-known <a href="https://en.wikipedia.org/wiki/Beap">beap</a> data structure). This structure is further developed and discussed in <a href="https://doi.org/10.1145/322358.322364">"Implicit Data Structures for the Dictionary Problem"</a> (1983) and <a href="http://dl.acm.org/citation.cfm?id=645933.673366">"Succinct Dynamic Data Structures"</a> (2001). (The latter generalizes the idea to the dynamic array abstract data type, rather than a sorted array.)

The theoretical advantage of a 2-level rotated array over an ordinary sorted array is that it provides the same search performance (O(log n)), with much better insert and delete performance (O(√n), compared to O(n) for a sorted array), in exactly the same amount of space (i.e., no more than the data itself). (For the purely implicit structure, inserts and deletes are O(√n * log n), but can be reduced to O(√n) using O(√n) extra space, a negligible amount.) This is considerably worse than the O(log N) insert/delete performance of a balanced tree (e.g., a red-black tree or B-tree), but takes up a fraction of the space (less than half the space of a B-tree).

Here is a detailed description of the 2-level array data structure, from <a href=https://doi.org/10.1016/j.ipl.2010.08.007>"A compact data structure for representing a dynamic multiset"</a>:

> We use a two-level rotated array to store the elements, with additional
> pointers to point to the beginning of each rotated array. A rotated array
> is an arbitrary cyclic shift of a sorted array. In a two-level rotated
> array, the elements are stored in an array divided into ⌈√(2n)⌉
> blocks, where the ith block is a rotated array of length i. All the
> elements in the ith block are less than or equal to any element in the
> (i + 1)st block. We store the starting position of each block (rotated
> array) explicitly. To search for an element e, we first perform a binary
> search to find two adjacent blocks where the first (last) occurrence
> of e can lie. We then perform a binary search in those two blocks to
> find the answer. Thus element-based searches can be supported in O(lg n)
> worst-case time. The locator of an element can be realized by recording
> the block in which the element is stored together with the offset from the
> beginning of the block. Given a locator, the locator to its predecessor
> or successor can be easily obtained in O(1) worst-case time.  To insert
> an element e, we first find the location where e should be inserted (using
> upper-bound). We perform the insertion into the block by removing the last
> element of the rotated array, and shifting all the elements between the
> new element and the last element, one position forward. We then have to
> update every block following the block in which the insertion was made as
> follows: remove the last element of the current rotated array and insert
> the last element from the previous rotated array which now becomes the new
> first element of the rotated array. Also, we update the starting position
> of each block in O(1) time per block. Thus, insertions can be supported
> in O(√n) worst-case time. Deletions can be performed analogously.

All implementations are in Rust, and are benchmarked using the <a href="https://github.com/bheisler/criterion.rs">Criterion</a> benchmark framework.

A dynamic array implementation of the same data structure (roughly a drop-in replacement for `Vec`, except that it doesn't support deref to a slice) is available at https://github.com/senderista/rotated-vec.
