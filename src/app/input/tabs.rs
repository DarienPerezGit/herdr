use crate::app::state::AppState;

impl AppState {
    pub(crate) fn is_new_tab_hovered(&self) -> bool {
        let Some((col, row)) = self.last_mouse_position else {
            return false;
        };
        let rect = self.view.new_tab_hit_area;
        rect.width > 0
            && col >= rect.x
            && col < rect.x + rect.width
            && row >= rect.y
            && row < rect.y + rect.height
    }

    pub(crate) fn is_tab_scroll_left_hovered(&self) -> bool {
        let Some((col, row)) = self.last_mouse_position else {
            return false;
        };
        let rect = self.view.tab_scroll_left_hit_area;
        rect.width > 0
            && col >= rect.x
            && col < rect.x + rect.width
            && row >= rect.y
            && row < rect.y + rect.height
    }

    pub(crate) fn is_tab_scroll_right_hovered(&self) -> bool {
        let Some((col, row)) = self.last_mouse_position else {
            return false;
        };
        let rect = self.view.tab_scroll_right_hit_area;
        rect.width > 0
            && col >= rect.x
            && col < rect.x + rect.width
            && row >= rect.y
            && row < rect.y + rect.height
    }

    pub(crate) fn hovered_tab_idx(&self) -> Option<usize> {
        let (col, row) = self.last_mouse_position?;
        self.view
            .tab_hit_areas
            .iter()
            .enumerate()
            .find(|(_, rect)| {
                rect.width > 0
                    && col >= rect.x
                    && col < rect.x + rect.width
                    && row >= rect.y
                    && row < rect.y + rect.height
            })
            .map(|(idx, _)| idx)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ratatui::layout::Rect;

    fn test_app_with_tab_areas() -> AppState {
        let mut app = AppState::test_new();
        app.view.tab_bar_rect = Rect::new(0, 0, 80, 1);
        app.view.tab_hit_areas = vec![
            Rect::new(0, 0, 10, 1),
            Rect::new(11, 0, 10, 1),
            Rect::new(22, 0, 10, 1),
        ];
        app.view.tab_scroll_left_hit_area = Rect::new(60, 0, 3, 1);
        app.view.tab_scroll_right_hit_area = Rect::new(64, 0, 3, 1);
        app.view.new_tab_hit_area = Rect::new(68, 0, 3, 1);
        app
    }

    #[test]
    fn new_tab_hovered_inside() {
        let mut app = test_app_with_tab_areas();
        app.last_mouse_position = Some((69, 0));
        assert!(app.is_new_tab_hovered());
    }

    #[test]
    fn new_tab_hovered_outside_or_no_mouse() {
        let mut app = test_app_with_tab_areas();
        app.last_mouse_position = Some((0, 0));
        assert!(!app.is_new_tab_hovered());
        app.last_mouse_position = None;
        assert!(!app.is_new_tab_hovered());
    }

    #[test]
    fn tab_scroll_left_hovered_inside() {
        let mut app = test_app_with_tab_areas();
        app.last_mouse_position = Some((61, 0));
        assert!(app.is_tab_scroll_left_hovered());
    }

    #[test]
    fn tab_scroll_right_hovered_inside() {
        let mut app = test_app_with_tab_areas();
        app.last_mouse_position = Some((65, 0));
        assert!(app.is_tab_scroll_right_hovered());
    }

    #[test]
    fn tab_scroll_buttons_not_hovered_outside() {
        let mut app = test_app_with_tab_areas();
        app.last_mouse_position = Some((50, 0));
        assert!(!app.is_tab_scroll_left_hovered());
        assert!(!app.is_tab_scroll_right_hovered());
    }

    #[test]
    fn hovered_tab_idx_returns_correct_index() {
        let mut app = test_app_with_tab_areas();
        app.last_mouse_position = Some((12, 0));
        assert_eq!(app.hovered_tab_idx(), Some(1));
    }

    #[test]
    fn hovered_tab_idx_returns_none_outside_or_no_mouse() {
        let mut app = test_app_with_tab_areas();
        app.last_mouse_position = Some((100, 0));
        assert_eq!(app.hovered_tab_idx(), None);
        app.last_mouse_position = None;
        assert_eq!(app.hovered_tab_idx(), None);
    }

    #[test]
    fn hovered_tab_idx_ignores_zero_width_areas() {
        let mut app = test_app_with_tab_areas();
        app.view.tab_hit_areas[1] = Rect::new(11, 0, 0, 1);
        app.last_mouse_position = Some((12, 0));
        assert_eq!(app.hovered_tab_idx(), None);
    }
}
