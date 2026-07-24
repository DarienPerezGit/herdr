# Spec: Subtle hover feedback for TUI interactive elements

## Problem Statement

The user is facing a lack of visual hover feedback when passing the mouse cursor over interactive links/buttons in the Herdr TUI (such as `new`, `menu`, tab scroll controls, and workspace/agent list cards). This makes it difficult to verify if the mouse cursor is correctly positioned before clicking, and violates the mouse-first design principle of the application.

## Solution

Introduce a subtle hover feedback mechanism using the existing TUI color palette. When the mouse moves over an interactive element:
- The `new` and `menu` buttons in the sidebar footer change color from `p.overlay0` (muted gray) to `p.text` (soft white).
- Tab bar buttons (like `+`) change from `p.overlay1` to `p.text`.
- Tab scroll controls (`<` and `>`) highlight their background from `p.surface0` to `p.surface1`.
- Sidebar workspace list cards (if not already active/selected) change their text color from `p.subtext0` to `p.text`.
- Sidebar agent list entries highlight their name and icon to `p.text`.
- To prevent hover highlights from getting "stuck" when the user switches to keyboard navigation or loses window focus, the hover state is immediately cleared when any key is pressed or when the host terminal loses focus (`OuterFocusLost`).

## User Stories

1. As a Herdr user, I want the "new" and "menu" footer links to turn soft white when my mouse hovers over them, so that I get immediate visual confirmation that they are clickable.
2. As a Herdr user, I want the "+" tab button to turn soft white on mouse hover, so that I know I am aiming correctly to create a tab.
3. As a Herdr user, I want the "<" and ">" scroll buttons in the tab bar to highlight their background slightly on hover, so that I have feedback before clicking them.
4. As a Herdr user, I want inactive workspace cards in the sidebar to light up to soft white on hover, so that I can easily tell which workspace I am about to select.
5. As a Herdr user, I want agent list entries in the sidebar to highlight their text to soft white on hover, so that I get visual feedback when pointing at them.
6. As a Herdr user, I want all hover highlights to immediately disappear when I press any key on my keyboard, so that the UI remains clean and doesn't get stuck in a hover state.
7. As a Herdr user, I want hover highlights to disappear when the Herdr window loses focus, so that the interface doesn't show outdated hover states when I am working in another application.

## Implementation Decisions

- **State Additions:**
  - Add `last_mouse_position: Option<(u16, u16)>` to `AppState` in `src/app/state.rs`.
- **Event Handling:**
  - Update `route_client_events_from` in `src/app/mod.rs` to:
    - Save the coordinates to `self.state.last_mouse_position` on `RawInputEvent::Mouse`.
    - Clear `last_mouse_position` to `None` on any key event (`RawInputEvent::Key`) or focus loss (`RawInputEvent::OuterFocusLost`).
- **Hit-Testing Helpers:**
  - Implement helper methods on `AppState`:
    - `is_new_button_hovered() -> bool`
    - `is_menu_button_hovered() -> bool`
    - `hovered_workspace_idx() -> Option<usize>`
    - `hovered_agent_pane_id() -> Option<PaneId>`
    - `hovered_tab_idx() -> Option<usize>`
    - `is_new_tab_hovered() -> bool`
    - `is_tab_scroll_left_hovered() -> bool`
    - `is_tab_scroll_right_hovered() -> bool`
- **TUI Rendering updates:**
  - Update `src/ui/sidebar.rs` to render `new`, `menu`, and inactive workspaces/agents using `p.text` if hovered.
  - Update `src/ui/tabs.rs` to render `+` using `p.text`, and `<` / `>` using `p.surface1` bg if hovered.

## Testing Decisions

- Test mouse move events: simulate mouse movement to specific component coordinates using `app.handle_mouse` and assert corresponding hover state flags in `AppState`.
- Test hover clear: verify that key presses or focus loss reset `last_mouse_position` to `None`.
- Test render integration: render sidebar and tab bar with mock hover coordinates and assert that the expected colors (`p.text` / `p.surface1`) are written to the test buffer.

## Out of Scope

- Flashy hover animations, transition delays, or layout layout reflows on hover.
- Custom color definitions (only existing colors from the theme palette will be used).
- Interactive hover elements inside the main terminal grids (ghostty terminals handle their own hover/mouse events).

## Further Notes

- Since Herdr is a mouse-first TUI, this lays the foundation for future interactive elements (like hover tooltips or click actions) by establishing standard mouse-position tracking in `AppState`.