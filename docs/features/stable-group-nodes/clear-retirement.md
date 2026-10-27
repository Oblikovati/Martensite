# Entry-only clear retirement (#324)

Clear retires exactly the groups that belong to the structure when the call begins. Under a brief fields guard, it moves that actual sequence owner into a local retirement scope and installs an empty sequence with the same allocator. It releases the guard before destroying any entry payload.

A destructor can inspect the actual structure without blocking on the clear operation. The old entry nodes are absent from the live collection. Groups created by a callback belong to the new live sequence and remain reachable; later callbacks can see those new groups. The text and highlight flags keep their previous values throughout retirement and reset only after every entry group retires successfully.

If a payload destructor panics, the moved owner retains the remaining entry chain. An unwind guard retires those nodes in sequence order. The original panic continues after that cleanup. Callback-created groups remain in the actual structure and both flags retain their previous values because the success-only reset did not run. This follows Rust's ordinary unwinding model: a second destructor panic during unwinding aborts the process. The owned regression injects one deliberate typed panic, without a second panic in cleanup.

This is an explicit behavioral change from the previous guard-held implementation, which prevented safe destructor reentry. It has independent owned expectations and separate release notes. The earlier storage-refactor checkpoint and the #325 selected-node correction remain separate commits.

The ordinary regressions install four real groups with destructor callbacks and no external payload owners. Successful retirement records A, B, C, D, checks old flags and allocator identity at each callback, and retains the new group created by A. The panic case lets A create a group, then lets B create another and raise a named typed panic. Cleanup must still record C and D in order, preserve both new groups and old flags, and propagate B's exact panic payload. A nonblocking fields probe records the previous lock failure without hanging the pre-change test. The five baseline snapshot/ownership test bodies remain unchanged.

This source checkpoint has not been compiled or executed. Source review, the pre-change regression runs, corrected runs and required package gates remain pending. Versions and tags are unchanged.
