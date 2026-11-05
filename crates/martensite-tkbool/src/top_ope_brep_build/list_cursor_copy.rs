//! Copying owned list storage must remap cursor nodes rather than retain old addresses.
use martensite_tkernel::ncollection::{List, ListCursor};

pub(crate) fn copied_cursor<T, U>(
    original: ListCursor,
    before: &List<T>,
    after: &List<U>,
) -> ListCursor {
    if original.current_node().is_none() && original.previous_node().is_none() {
        return ListCursor::default();
    }
    let (mut left, mut right) = (before.cursor(), after.cursor());
    loop {
        if left == original {
            return right;
        }
        if !left.more() || !right.more() {
            return ListCursor::default();
        }
        left.next(before);
        right.next(after);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn copy_preserves_position_but_not_node_identity() {
        let before: List<i32> = [3, 5].into_iter().collect();
        let after = before.clone();
        let mut cursor = before.cursor();
        cursor.next(&before);
        let copied = copied_cursor(cursor, &before, &after);
        assert_eq!(copied.value(&after), Some(&5));
        assert_ne!(cursor.current_node(), copied.current_node());
    }
    #[test]
    fn copy_distinguishes_default_cursor_from_reached_end() {
        let before: List<i32> = [3].into_iter().collect();
        let after = before.clone();
        let mut end = before.cursor();
        end.next(&before);
        let copied = copied_cursor(end, &before, &after);
        assert!(!copied.more());
        assert_eq!(copied.previous_node(), after.cursor().current_node());
        assert_eq!(
            copied_cursor(ListCursor::default(), &before, &after).previous_node(),
            None
        );
    }
}
