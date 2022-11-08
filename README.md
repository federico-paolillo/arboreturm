# arboreturm

Binary search trees n' shit

## Binary Search Tree

Binary Search Tree is implemented as `arboretum::Bst<V: PartialOrd>`.

Usage is pretty simple:

```rust
let bst = &mut Bst::empty();

bst.insert(5);

assert_eq!(true, bst.contains(5));
```
