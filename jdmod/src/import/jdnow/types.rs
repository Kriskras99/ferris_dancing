//! Types used to work with Just Dance Now data

use std::path::{Path, PathBuf};

use bluestar_toolkit::SongDetails;
use dotstar_toolkit_utils::vfs::{VirtualFileSystem, VirtualPath, VirtualPathBuf};

use crate::{import::TranscodeSettings, types::song::SongDirectoryTree};

/// Directory and file structure of a song from Just Dance Now
pub struct NowTree {
    /// Root song dir
    dir_root: PathBuf,
    /// Video file (with audio)
    file_video: PathBuf,
    /// Cover file
    file_cover: PathBuf,
    /// File with more details
    file_detail: PathBuf,
    /// Map background file
    file_map_bkg: PathBuf,
    /// File with all the pictos
    file_atlas: PathBuf,
    /// File describing the locations of the pictos
    file_atlas_desc: PathBuf,
    /// File with the classifiers and moves
    file_bundle: PathBuf,
    /// The lowercase name of the song
    lower_song_name: String,
    /// The tree of `file_bundle`
    tree_bundle: NowBundleTree,
}

impl NowTree {
    /// Create the tree
    pub fn new(dir_root: &Path, song_name: &str) -> Self {
        let lower = song_name.to_lowercase();
        Self {
            dir_root: dir_root.to_owned(),
            file_video: dir_root.join(format!("{song_name}.mp4")),
            file_cover: dir_root.join(format!("{lower}.jpg")),
            file_detail: dir_root.join(format!("{song_name}.extra.json")),
            file_map_bkg: dir_root.join(format!("{song_name}_map_bkg.jpg")),
            file_atlas: dir_root.join("pictos-atlas.png"),
            file_atlas_desc: dir_root.join("pictos-atlas.json"),
            file_bundle: dir_root.join("bundle.zip"),
            lower_song_name: lower,
            tree_bundle: NowBundleTree::new(song_name),
        }
    }

    /// Get the path to the picture of coach `n`
    pub fn coach(&self, n: u8) -> PathBuf {
        self.dir_root
            .join(format!("{}_coach_{n}_big.png", self.lower_song_name))
    }

    /// Get the path to the video file (with audio)
    pub fn video(&self) -> &Path {
        &self.file_video
    }
    /// Get the path to the cover file
    pub fn cover(&self) -> &Path {
        &self.file_cover
    }

    /// Get the path to the file with more details
    pub fn detail(&self) -> &Path {
        &self.file_detail
    }

    /// Get the path to the map background file
    pub fn map_bkg(&self) -> &Path {
        &self.file_map_bkg
    }

    /// Get the path to the pictos atlas file
    pub fn atlas(&self) -> &Path {
        &self.file_atlas
    }

    /// Get the path to the description of the pictos atlas file
    pub fn atlas_desc(&self) -> &Path {
        &self.file_atlas_desc
    }

    /// Get the path to the bundle files
    pub fn bundle(&self) -> &Path {
        &self.file_bundle
    }

    /// The file and directory tree that descibes a Just Dance Now bundle
    pub const fn bundle_tree(&self) -> &NowBundleTree {
        &self.tree_bundle
    }
}

/// File and directory tree that descripes a Just Dance Now bundle
pub struct NowBundleTree {
    /// Directory with the moves descriptions
    dir_moves: VirtualPathBuf,
    /// Directory with the msm classifiers
    dir_classifiers: VirtualPathBuf,
    /// The properly-cased song name
    song_name: String,
}

impl NowBundleTree {
    /// Create the bundle tree
    pub fn new(song_name: &str) -> Self {
        Self {
            dir_moves: VirtualPathBuf::from("moves"),
            dir_classifiers: VirtualPathBuf::from("classifiers_WIIU"),
            song_name: song_name.to_owned(),
        }
    }

    /// Get the path to the move descriptions file of coach `n`
    pub fn classifiers_desc(&self, n: u8) -> VirtualPathBuf {
        self.dir_moves
            .join(format!("{}_moves{n}.json", self.song_name))
    }

    /// Get the path to the directory with the classifiers
    pub fn classifiers(&self) -> &VirtualPath {
        &self.dir_classifiers
    }
}

/// State needed for parsing a JDNow song
pub struct NowState<'a> {
    /// Filesystem with the import files
    pub bundle: &'a dyn VirtualFileSystem,
    /// Directory tree of the Now directory
    pub now: NowTree,
    /// Directory tree for the song
    pub song: SongDirectoryTree,
    /// Transcoding settings
    pub transcode: TranscodeSettings,
    /// Codename for the map
    pub details: SongDetails<'a>,
    /// The converted beats
    pub beats: Vec<u32>,
}
