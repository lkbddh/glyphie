use crate::theme::{EMOJI_BUTTON_SIZE, GRID_SPACING, SPACING_SM, SUBCATEGORY_HEADER_HEIGHT};

/// One visual row of the emoji grid: positions `start..start + len` of
/// `cached_indices`, inside subcategory group `group`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct GridRow {
    pub(crate) group: usize,
    pub(crate) start: usize,
    pub(crate) len: usize,
}

/// Splits each group into rows of at most `columns`; a group always starts a
/// new row. The view draws exactly these rows (`layout_rows`).
pub(crate) fn grid_rows(group_sizes: &[usize], columns: usize) -> Vec<GridRow> {
    let mut rows = Vec::new();
    let mut start = 0;
    for (group, &size) in group_sizes.iter().enumerate() {
        let group_end = start + size;
        while start < group_end {
            let len = columns.min(group_end - start);
            rows.push(GridRow { group, start, len });
            start += len;
        }
    }
    rows
}

/// Moves `delta` rows from `current_pos`, keeping the column when the target
/// row is long enough. Past either end it stops on the first or last emoji.
pub(crate) fn vertical_highlight_position(
    rows: &[GridRow],
    current_pos: Option<usize>,
    delta: isize,
) -> Option<usize> {
    let last_row = rows.last()?;
    let last_pos = last_row.start + last_row.len - 1;
    let Some(current_pos) = current_pos else {
        return Some(if delta < 0 { last_pos } else { 0 });
    };

    let row = rows.iter().position(|r| current_pos < r.start + r.len)?;
    let column = current_pos - rows[row].start;
    let Some(target) = row.checked_add_signed(delta).and_then(|t| rows.get(t)) else {
        return Some(if delta < 0 { 0 } else { last_pos });
    };

    Some(target.start + column.min(target.len - 1))
}

/// Top edge of `row` inside the scrollable grid, mirroring the view: top
/// padding, then per group an optional header and its rows, `SPACING_SM` apart.
pub(crate) fn row_top(rows: &[GridRow], row: usize, with_headers: bool) -> f32 {
    let header = if with_headers {
        SUBCATEGORY_HEADER_HEIGHT + SPACING_SM
    } else {
        0.0
    };
    let mut top = SPACING_SM + header;
    for pair in rows[..rows.len().min(row + 1)].windows(2) {
        top += EMOJI_BUTTON_SIZE;
        top += if pair[0].group == pair[1].group {
            GRID_SPACING
        } else {
            SPACING_SM + header
        };
    }
    top
}

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

    // Groups of 8 and 3 at 6 columns: rows [0..6] [6..8] | [8..11].
    fn sample_rows() -> Vec<GridRow> {
        grid_rows(&[8, 3], 6)
    }

    #[test]
    fn each_group_starts_a_new_row() {
        let starts: Vec<_> = sample_rows()
            .iter()
            .map(|r| (r.group, r.start, r.len))
            .collect();
        assert_eq!(starts, vec![(0, 0, 6), (0, 6, 2), (1, 8, 3)]);
    }

    #[test]
    fn vertical_moves_keep_the_column_across_groups() {
        let rows = sample_rows();
        // Column 1 of row 0 → column 1 of the short row → column 1 of the next group.
        assert_eq!(vertical_highlight_position(&rows, Some(1), 1), Some(7));
        assert_eq!(vertical_highlight_position(&rows, Some(7), 1), Some(9));
        assert_eq!(vertical_highlight_position(&rows, Some(9), -1), Some(7));
    }

    #[test]
    fn vertical_moves_clamp_to_short_rows_and_ends() {
        let rows = sample_rows();
        assert_eq!(vertical_highlight_position(&rows, Some(5), 1), Some(7));
        assert_eq!(vertical_highlight_position(&rows, Some(9), 1), Some(10));
        assert_eq!(vertical_highlight_position(&rows, Some(3), -1), Some(0));
        assert_eq!(vertical_highlight_position(&rows, None, -1), Some(10));
        assert_eq!(vertical_highlight_position(&[], None, 1), None);
    }

    #[test]
    fn row_top_adds_headers_and_group_gaps() {
        let rows = sample_rows();
        let header = SUBCATEGORY_HEADER_HEIGHT + SPACING_SM;
        let step = EMOJI_BUTTON_SIZE + GRID_SPACING;
        assert_eq!(row_top(&rows, 0, false), SPACING_SM);
        assert_eq!(row_top(&rows, 1, true), SPACING_SM + header + step);
        assert_eq!(
            row_top(&rows, 2, true),
            SPACING_SM + header + step + EMOJI_BUTTON_SIZE + SPACING_SM + header
        );
    }
}
