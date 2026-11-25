# Module 8 checkpoint

Write your answers in your own words before checking the `solutions` branch.

### 1. What does `RefCell` give up compared with `&mut`?

<!-- ANSWER-START -->Compile-time certainty. Borrow violations become runtime panics, and each borrow pays a small counter check.<!-- ANSWER-END -->

### 2. Why can't `Rc` be sent to another thread?

<!-- ANSWER-START -->Its reference count is updated with plain, non-atomic increments. Two threads cloning or dropping at once would corrupt the count, so `Rc` is not `Send`. `Arc` uses atomics.<!-- ANSWER-END -->

### 3. Why does the default `Drop` of a linked list overflow the stack on long lists?

<!-- ANSWER-START -->Dropping a `Box<Node>` drops its `next` field, which drops the next box, and so on: recursion one level deep per node. An iterative `Drop` that `take()`s each `next` avoids it.<!-- ANSWER-END -->

### 4. What does an arena trade away compared with `Rc<RefCell<_>>`?

<!-- ANSWER-START -->Index validity is no longer checked by the type system (a stale `NodeId` is a logic bug), and removing nodes needs extra design work. In exchange you get no reference counting, no runtime borrow panics, cache-friendly memory and plain `&mut self` mutation.<!-- ANSWER-END -->
