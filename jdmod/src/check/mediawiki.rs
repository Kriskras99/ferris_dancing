//! Contains functions for interacting with the MediaWiki API
use std::collections::VecDeque;

use serde_json::{Map, Value};
use ureq::Request;

/// A wrapper around the MediaWiki API
pub struct MediaWiki {
    /// The root site without the `https://` or `/api.php`
    root: String,
}

impl MediaWiki {
    /// Create the MediaWiki API for root
    pub fn new(root: &str) -> Self {
        Self {
            root: root.to_string(),
        }
    }

    /// Create an iterator over the pages in the category
    pub fn category_members(&self, category: &str) -> CategoryMembersIterator {
        CategoryMembersIterator::new(&self.root, category)
    }
}

/// An iterator over the pages in a category
pub struct CategoryMembersIterator {
    /// The base request, which gcmcontinue is appended to
    request: Request,
    /// The value to continue with, if None there are no pages left
    gcmcontinue: Option<String>,
    /// Stores pages so the api doesn't have to be called every iteration
    buffer: VecDeque<Map<String, Value>>,
}

impl CategoryMembersIterator {
    /// Create an new iterator over the `category`
    pub fn new(root: &str, category: &str) -> Self {
        let request = ureq::get(&format!("https://{root}/api.php"))
            .query("action", "query")
            .query("format", "json")
            .query("formatversion", "2")
            .query("generator", "categorymembers")
            .query("gcmlimit", "500")
            .query("gcmtitle", category)
            .query("gcmtype", "page")
            .query("prop", "pageprops");
        Self {
            request,
            gcmcontinue: Some(String::new()),
            buffer: VecDeque::with_capacity(500),
        }
    }
}

impl Iterator for CategoryMembersIterator {
    type Item = Map<String, Value>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.buffer.is_empty() {
            if let Some(cmcontinue) = &self.gcmcontinue {
                let mut response: Map<String, Value> = self
                    .request
                    .clone()
                    .query("gcmcontinue", cmcontinue)
                    .call()
                    .unwrap()
                    .into_json()
                    .unwrap();
                self.gcmcontinue = response
                    .get("continue")
                    .and_then(Value::as_object)
                    .and_then(|o| o.get("gcmcontinue"))
                    .and_then(Value::as_str)
                    .map(ToOwned::to_owned);
                let categorymembers = response
                    .remove("query")
                    .and_then(|v| {
                        let Value::Object(o) = v else { return None };
                        Some(o)
                    })
                    .and_then(|mut o| o.remove("pages"))
                    .and_then(|v| {
                        let Value::Array(a) = v else { return None };
                        Some(a)
                    })
                    .map(|v| {
                        v.into_iter()
                            .map(|v| {
                                let Value::Object(o) = v else {
                                    panic!("Not an object!")
                                };
                                o
                            })
                            .collect::<Vec<_>>()
                    })
                    .unwrap_or_default();
                self.buffer.extend(categorymembers);
            }
        }
        self.buffer.pop_front()
    }
}
