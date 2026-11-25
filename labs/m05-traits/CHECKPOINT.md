# Module 5 checkpoint

Write your answers in your own words before checking the `solutions` branch.

### 1. What does monomorphization cost you?

<!-- ANSWER-START -->Compile time and binary size: each concrete type used with a generic function gets its own copy of the machine code. In return, calls can be inlined and there's no runtime dispatch.<!-- ANSWER-END -->

### 2. Why does `Vec<dyn Shape>` not compile while `Vec<Box<dyn Shape>>` does?

<!-- ANSWER-START -->`dyn Shape` has no size known at compile time, because different implementors have different sizes. `Box<dyn Shape>` is a fixed-size fat pointer (data pointer + vtable pointer).<!-- ANSWER-END -->

### 3. Why must `Ord` be consistent with `PartialEq`?

<!-- ANSWER-START -->Sorting, `BinaryHeap`, `BTreeMap` and deduplication assume `a.cmp(b) == Equal` exactly when `a == b`. If they disagree, collections silently misbehave.<!-- ANSWER-END -->

### 4. Why can you `impl Mul<Vec2> for f64` even though `f64` and `Mul` are both foreign?

<!-- ANSWER-START -->The orphan rule allows it because a local type (`Vec2`) appears as a type parameter of the trait, so no other crate could write a conflicting impl.<!-- ANSWER-END -->
