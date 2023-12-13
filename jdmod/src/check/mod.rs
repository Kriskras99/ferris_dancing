//! Contains functions for verifying the mod

use std::path::PathBuf;

use anyhow::{bail, Error};
use clap::Args;
use serde_json::Value;

use crate::{check::mediawiki::MediaWiki, types::DirectoryTree};

mod mediawiki;

/// Extract a UbiArt archive
#[derive(Args, Clone)]
pub struct Check {
    /// Mod directory
    mod_dir: PathBuf,
}

/// Wrapper around [`check`]
pub fn main(data: &Check) -> Result<(), Error> {
    let dir_tree = DirectoryTree::new(&data.mod_dir);
    if !dir_tree.exists() {
        bail!("Did not find expected directories in the directory tree!")
    }
    check(dir_tree)?;
    Ok(())
}

/// Checks which songs are in the mod and which ones are missing
///
/// Maybe host a file on Github as the parsing is finicky, and some songs have multiple codenames
///
/// TODO: Also check song correctness (missing files)
pub fn check(_dir_tree: DirectoryTree) -> Result<(), Error> {
    let mediawiki = MediaWiki::new("justdance.fandom.com");
    for song in mediawiki.category_members("Category:Songs") {
        let title = song["title"].as_str().unwrap();
        if let Some(pageprops) = song.get("pageprops") {
            let infoboxes: Vec<Value> =
                serde_json::from_str(pageprops["infoboxes"].as_str().unwrap())?;
            let codenames = infoboxes
                .first()
                .and_then(Value::as_object)
                .and_then(|o| o.get("data"))
                .and_then(Value::as_array)
                .and_then(|a| {
                    a.iter()
                        .filter_map(Value::as_object)
                        .filter(|o| o["type"] == "data")
                        .map(|o| &o["data"])
                        .filter_map(Value::as_object)
                        .find(|data| {
                            let label = data["label"].as_str().unwrap();
                            let mut label = label.to_ascii_lowercase();
                            label.retain(|c| !c.is_whitespace());
                            label.contains("codename")
                        })
                });
            if let Some(codenames) = codenames {
                let codenames = codenames["value"].as_str().unwrap().replace("<br>", " | ");
                println!("{title}: {codenames}");
            } else {
                println!("{title}: No codename!");
            }
        }
    }
    Ok(())
}
