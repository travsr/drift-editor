use std::fmt::Display;

use serde::{Deserialize, Serialize};
use typeshare::typeshare;

#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum DWindowStateScope {
    All,
    FileList,
    Tabs,
    Content,
}

impl Display for DWindowStateScope {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::All => write!(f, "all"),
            Self::FileList => write!(f, "file_list"),
            Self::Tabs => write!(f, "tabs"),
            Self::Content => write!(f, "content"),
        }
    }
}
