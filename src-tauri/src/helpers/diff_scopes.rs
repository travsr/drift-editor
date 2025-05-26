use crate::models::{d_window_state::DWindowState, d_window_state_scope::DWindowStateScope};

pub fn diff_scopes(prev: &DWindowState, next: &DWindowState) -> Vec<DWindowStateScope> {
    let mut scopes = Vec::new();

    if prev.file_list != next.file_list {
        scopes.push(DWindowStateScope::FileList);
    }
    if prev.tabs != next.tabs {
        scopes.push(DWindowStateScope::Tabs);
    }
    if prev.content != next.content {
        scopes.push(DWindowStateScope::Content);
    }

    scopes
}
