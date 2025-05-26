use crate::models::{d_window_event_scope::DWindowEventScope, d_window_state::DWindowState};

pub fn get_scopes_for_diff(prev: &DWindowState, next: &DWindowState) -> Vec<DWindowEventScope> {
    let mut scopes = Vec::new();

    if prev.file_list != next.file_list {
        scopes.push(DWindowEventScope::FileList);
    }
    if prev.tabs != next.tabs {
        scopes.push(DWindowEventScope::Tabs);
    }
    if prev.content != next.content {
        scopes.push(DWindowEventScope::Content);
    }

    scopes
}
