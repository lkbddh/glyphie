pub(crate) fn active_copy_index(
    highlighted_index: Option<usize>,
    cached_indices: &[usize],
) -> Option<usize> {
    highlighted_index
        .filter(|idx| cached_indices.contains(idx))
        .or_else(|| cached_indices.first().copied())
}

pub(crate) fn next_highlight_position(
    len: usize,
    current_pos: Option<usize>,
    delta: isize,
) -> Option<usize> {
    if len == 0 {
        return None;
    }

    let Some(current_pos) = current_pos else {
        return Some(if delta < 0 { len - 1 } else { 0 });
    };

    Some(
        current_pos
            .saturating_add_signed(delta)
            .min(len.saturating_sub(1)),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn active_copy_prefers_visible_highlight() {
        assert_eq!(active_copy_index(Some(3), &[1, 3, 5]), Some(3));
    }

    #[test]
    fn active_copy_falls_back_to_first_visible_item() {
        assert_eq!(active_copy_index(None, &[2, 4, 6]), Some(2));
    }

    #[test]
    fn active_copy_ignores_stale_highlight() {
        assert_eq!(active_copy_index(Some(9), &[2, 4, 6]), Some(2));
    }

    #[test]
    fn active_copy_returns_none_when_empty() {
        assert_eq!(active_copy_index(Some(9), &[]), None);
    }

    #[test]
    fn highlight_returns_none_when_empty() {
        assert_eq!(next_highlight_position(0, None, 1), None);
    }

    #[test]
    fn highlight_starts_at_first_item_when_moving_forward() {
        assert_eq!(next_highlight_position(10, None, 6), Some(0));
    }

    #[test]
    fn highlight_starts_at_last_item_when_moving_backward() {
        assert_eq!(next_highlight_position(10, None, -1), Some(9));
    }

    #[test]
    fn highlight_clamps_to_bounds() {
        assert_eq!(next_highlight_position(10, Some(8), 6), Some(9));
        assert_eq!(next_highlight_position(10, Some(1), -6), Some(0));
    }
}
