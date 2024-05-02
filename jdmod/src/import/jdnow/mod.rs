//! Import functionality for Just Dance Now data

use anyhow::{anyhow, Error};
use dotstar_toolkit_utils::vfs::{zipfs::ZipFs, VirtualFileSystem, VirtualPath};

use self::types::BasicInfo;
use crate::{
    import::jdnow::types::NowTree,
    types::{song::SongDirectoryTree, DirectoryTree},
};
mod types;

/// Import a Song.json or published.json from Just Dance Now
pub fn import(
    vfs: &dyn VirtualFileSystem,
    path: &VirtualPath,
    tree_mod: &DirectoryTree,
) -> Result<(), Error> {
    let file = vfs.open(path)?;
    let parent = path
        .parent()
        .ok_or_else(|| anyhow!("No parent directory!"))?;

    if let Ok(published) = serde_json::from_slice::<Vec<BasicInfo>>(&file) {
        for song in published {
            import_song(vfs, &parent.join(song.id.as_ref()), song, tree_mod)?;
        }
    } else if let Ok(song) = serde_json::from_slice::<BasicInfo>(&file) {
        import_song(vfs, parent, song, tree_mod)?;
    }

    Ok(())
}

#[tracing::instrument(skip(vfs, basic, tree_mod))]
fn import_song(
    vfs: &dyn VirtualFileSystem,
    directory: &VirtualPath,
    basic: BasicInfo,
    tree_mod: &DirectoryTree,
) -> Result<(), Error> {
    let tree_jdn = NowTree::new(directory, &basic.id);
    let tree_song = SongDirectoryTree::new(tree_mod.songs(), &basic.id);
    if tree_song.exists() {
        println!("Skipping {}, song already imported!", basic.id);
        return Ok(());
    }

    tree_song.create_dir_all()?;
    // TODO: Create autodance stuff

    let zip = vfs.open(&directory.join("bundle.zip"))?;
    tracing::trace!("Opened zip file");
    let zipfs = ZipFs::new(zip)?;

    // TODO: Import moves and classifiers

    // TODO: Import lyrics

    // TODO: Check if we can get vibrations from beats or moves

    // TODO: Split and import video and audio

    // TODO: Import the menuart

    zipfs
        .walk_filesystem(VirtualPath::new(""))?
        .for_each(|p| println!("{p}"));

    Ok(())
}
