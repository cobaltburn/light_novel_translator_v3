use crate::page::{PageError, Section};
use serde::{Deserialize, Serialize};
use std::{
    borrow::Cow,
    path::{Path, PathBuf},
};

pub const TOC_PAGE_PREFIX: &str = "__toc__";

#[derive(Debug, Clone, Copy, Default, Hash, PartialEq, Eq)]
pub enum Activity {
    #[default]
    Incomplete,
    Complete,
    Active(usize),
    Error(usize),
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Page {
    pub path: PathBuf,
    pub sections: Vec<Section>,
    #[serde(skip)]
    pub activity: Activity,
    #[serde(skip)]
    pub errors: Vec<PageError>,
}

impl Page {
    pub fn file_stem(&self) -> Option<Cow<'_, str>> {
        self.path.file_stem().map(|p| p.to_string_lossy())
    }

    pub fn new(path: PathBuf, sections: Vec<String>) -> Self {
        let sections = sections.into_iter().map(Section::new).collect();
        Page {
            path: absolute_page_path(path),
            sections,
            ..Default::default()
        }
    }

    pub fn clear(&mut self) {
        self.sections.iter_mut().for_each(|e| e.content.clear());
        self.errors.clear();
    }

    pub fn file_prefix(&self) -> Option<Cow<'_, str>> {
        self.path.file_prefix().map(|p| p.to_string_lossy())
    }

    pub fn active(&self) -> bool {
        matches!(self.activity, Activity::Active(_))
    }

    pub fn error_indexes(&self) -> Vec<usize> {
        let mut indexes: Vec<_> = self.errors.iter().map(|p| p.index).collect();
        indexes.sort_unstable();
        indexes.dedup();
        indexes
    }
}

pub fn absolute_page_path(path: PathBuf) -> PathBuf {
    if path.is_relative() {
        Path::new("/").join(path)
    } else {
        path
    }
}
