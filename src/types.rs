use std::path::PathBuf;
use std::rc::Rc;
use std::ops::{Bound, RangeBounds};
use core::cell::RefCell;
use chrono::{DateTime, Utc};

/// Defines interface used for both podcasts and episodes, to be
/// used and displayed in menus.
pub trait Menuable {
    fn get_title(&self, length: usize) -> String;
}

/// Struct holding data about an individual podcast feed. This includes a
/// (possibly empty) vector of episodes.
#[derive(Debug, Clone)]
pub struct Podcast {
    pub id: Option<i32>,
    pub title: String,
    pub url: String,
    pub description: Option<String>,
    pub author: Option<String>,
    pub explicit: Option<bool>,
    pub last_checked: DateTime<Utc>,
    pub episodes: MutableVec<Episode>,
}

impl Menuable for Podcast {
    fn get_title(&self, length: usize) -> String {
        return self.title[..].substring(0, length).to_string();
    }
}

/// Struct holding data about an individual podcast episode. Most of this
/// is metadata, but if the episode has been downloaded to the local
/// machine, the filepath will be included here as well. `played` indicates
/// whether the podcast has been marked as played or unplayed.
#[derive(Debug, Clone)]
pub struct Episode {
    pub id: Option<i32>,
    pub title: String,
    pub url: String,
    pub description: String,
    pub pubdate: Option<DateTime<Utc>>,
    pub duration: Option<i32>,
    pub path: Option<PathBuf>,
    pub played: bool,
}

impl Menuable for Episode {
    fn get_title(&self, length: usize) -> String {
        return match self.path {
            Some(_) => format!("[D] {}", self.title[..].substring(0, length-4)),
            None => self.title[..].substring(0, length).to_string(),
        };
    }
}


pub type MutableVec<T> = Rc<RefCell<Vec<T>>>;




// some utilities for dealing with UTF-8 substrings that split properly
// on character boundaries. From:
// https://users.rust-lang.org/t/how-to-get-a-substring-of-a-string/1351/11
// Note that using UnicodeSegmentation::graphemes() from the
// `unicode-segmentation` crate might still end up being preferable...
pub trait StringUtils {
    fn substring(&self, start: usize, len: usize) -> &str;
    fn slice(&self, range: impl RangeBounds<usize>) -> &str;
}

impl StringUtils for str {
    fn substring(&self, start: usize, len: usize) -> &str {
        let mut char_pos = 0;
        let mut byte_start = 0;
        let mut it = self.chars();
        loop {
            if char_pos == start { break; }
            if let Some(c) = it.next() {
                char_pos += 1;
                byte_start += c.len_utf8();
            }
            else { break; }
        }
        char_pos = 0;
        let mut byte_end = byte_start;
        loop {
            if char_pos == len { break; }
            if let Some(c) = it.next() {
                char_pos += 1;
                byte_end += c.len_utf8();
            }
            else { break; }
        }
        &self[byte_start..byte_end]
    }
    fn slice(&self, range: impl RangeBounds<usize>) -> &str {
        let start = match range.start_bound() {
            Bound::Included(bound) | Bound::Excluded(bound) => *bound,
            Bound::Unbounded => 0,
        };
        let len = match range.end_bound() {
            Bound::Included(bound) => *bound + 1,
            Bound::Excluded(bound) => *bound,
            Bound::Unbounded => self.len(),
        } - start;
        self.substring(start, len)
    }
}