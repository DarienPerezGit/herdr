# Tickets: Subtle hover feedback for TUI interactive elements

Implementation of subtle visual hover feedback on Herdr TUI components using the existing color palette, cleared on keypress and focus loss. See `.scratch/hover-feedback/PRD.md`.

Work the **frontier**: any ticket whose blockers are all done. For a purely linear chain that means top to bottom.

## Mouse Position Tracking & Clear Logic

**What to build:** The infrastructure in `AppState` to store and update the mouse coordinates, and the logic to clear them on keypresses and window focus loss.

**Blocked by:** None — can start immediately.

- [ ] Add `last_mouse_position: Option<(u16, u16)>` to `AppState` in `src/app/state.rs`.
- [ ] Initialize `last_mouse_position` as `None` in production and test constructors of `AppState`.
- [ ] Update `route_client_events_from` in `src/app/mod.rs` to set `last_mouse_position = Some((mouse.column, mouse.row))` on mouse moves/events.
- [ ] Update `route_client_events_from` in `src/app/mod.rs` to set `last_mouse_position = None` on any key pressed or `OuterFocusLost`.
- [ ] Add unit tests in `src/app/input/mouse.rs` verifying tracking and clear logic works as expected.

## Sidebar Hover Indicators (Footer, Workspaces, Agents)

**What to build:** Hit-testing helper methods on `AppState` for sidebar elements, and rendering styling in the sidebar components to light up hovered items to soft white (`p.text`).

**Blocked by:** Mouse Position Tracking & Clear Logic

- [ ] Implement `is_new_button_hovered` and `is_menu_button_hovered` on `AppState`.
- [ ] Implement `hovered_workspace_idx` on `AppState` using `workspace_card_areas`.
- [ ] Implement `hovered_agent_pane_id` on `AppState` using `agent_panel_rect` and `agent_detail_target_at`.
- [ ] Update `src/ui/sidebar.rs` to draw `new`, `menu`, inactive workspaces, and agents with `p.text` when hovered.
- [ ] Add tests in `src/ui/sidebar.rs` or `src/app/input/mouse.rs` asserting render styling when hover is simulated.

## Tab Bar Hover Indicators (New Tab, Scroll Controls, Tabs)

**What to build:** Hit-testing helper methods on `AppState` for tab bar elements, and rendering styling in the tab bar components to light up hovered items (`p.text` for tabs/plus, `p.surface1` for scroll backgrounds).

**Blocked by:** Mouse Position Tracking & Clear Logic

- [ ] Implement `is_new_tab_hovered`, `is_tab_scroll_left_hovered`, and `is_tab_scroll_right_hovered` on `AppState`.
- [ ] Implement `hovered_tab_idx` on `AppState` using `tab_hit_areas`.
- [ ] Update `src/ui/tabs.rs` to draw `+` and inactive tabs with `p.text` when hovered.
- [ ] Update `src/ui/tabs.rs` to draw `<` and `>` with `p.surface1` background when hovered.
- [ ] Add tests in `src/ui/tabs.rs` or `src/app/input/mouse.rs` asserting render styling when hover is simulated.