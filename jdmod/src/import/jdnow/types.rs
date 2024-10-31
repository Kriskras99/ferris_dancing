//! Types used to work with Just Dance Now data

use std::borrow::Cow;

use dotstar_toolkit_utils::vfs::{VirtualPath, VirtualPathBuf};
use serde::Deserialize;

/// Basic description of a Just Dance Now song
#[derive(Deserialize)]
pub struct BasicInfo<'a> {
    /// Unknown
    #[serde(rename = "_id")]
    pub raw_id: HipStr<'a>,
    /// Codename of the song (capitalized)
    pub id: HipStr<'a>,
    /// Artist of this song
    pub artist: HipStr<'a>,
    /// Name of the song
    pub name: HipStr<'a>,
    /// The amount of coaches
    pub coaches: u8,
    /// Publishing status of the song
    pub status: HipStr<'a>,
    /// Song credits
    pub credits: Vec<HipStr<'a>>,
    /// Associated avatar ids
    pub avatars: Vec<u16>,
    /// Duration of the song (in ms?)
    pub duration: usize,
    /// JDN version of the song
    pub version: u64,
    /// Difficulty of the song
    pub difficulty: u8,
    /// Original Just Dance game this is from
    pub jdversion: u16,
    /// Base url of the song
    pub base: HipStr<'a>,
    /// Base url for the avatars of the song
    pub app_avatars: HipStr<'a>,
    /// Url of the background image of the song
    pub bkg_image: HipStr<'a>,
}

/// Directory and file structure of a song from Just Dance Now
pub struct NowTree {
    /// Root song dir
    dir_root: VirtualPathBuf,
    /// File with basic info
    file_basic: VirtualPathBuf,
    /// Video file (with audio)
    file_video: VirtualPathBuf,
    /// Video preview file (with audio)
    file_video_preview: VirtualPathBuf,
    /// Cover file
    file_cover: VirtualPathBuf,
    /// File with more details
    file_detail: VirtualPathBuf,
    /// Map background file
    file_map_bkg: VirtualPathBuf,
    /// File with all the pictos
    file_atlas: VirtualPathBuf,
    /// File describing the locations of the pictos
    file_atlas_desc: VirtualPathBuf,
    /// File with the moves
    file_bundle: VirtualPathBuf,
    /// The lowercase name of the song
    lower_song_name: String,
    /// The tree of `file_bundle`
    tree_bundle: NowBundleTree,
}

impl NowTree {
    /// Create the tree
    pub fn new(dir_root: &VirtualPath, song_name: &str) -> Self {
        let lower = song_name.to_lowercase();
        Self {
            dir_root: dir_root.to_owned(),
            file_basic: dir_root.join(format!("{song_name}.json")),
            file_video: dir_root.join(format!("{song_name}.mp4")),
            file_video_preview: dir_root.join(format!("{song_name}_preview.mp4")),
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
    pub fn coach(&self, n: u8) -> VirtualPathBuf {
        self.dir_root
            .join(format!("{}_coach_{n}_big.png", self.lower_song_name))
    }

    /// Get the path to the basic info
    pub fn basic(&self) -> &VirtualPath {
        &self.file_basic
    }

    /// Get the path to the video file (with audio)
    pub fn video(&self) -> &VirtualPath {
        &self.file_video
    }

    /// Get the path to the video preview file (with audio)
    pub fn video_preview(&self) -> &VirtualPath {
        &self.file_video_preview
    }

    /// Get the path to the cover file
    pub fn cover(&self) -> &VirtualPath {
        &self.file_cover
    }

    /// Get the path to the file with more details
    pub fn detail(&self) -> &VirtualPath {
        &self.file_detail
    }

    /// Get the path to the map background file
    pub fn map_bkg(&self) -> &VirtualPath {
        &self.file_map_bkg
    }

    /// Get the path to the pictos atlas file
    pub fn atlas(&self) -> &VirtualPath {
        &self.file_atlas
    }

    /// Get the path to the description of the pictos atlas file
    pub fn atlas_desc(&self) -> &VirtualPath {
        &self.file_atlas_desc
    }

    /// Get the path to the bundle files
    pub fn bundle(&self) -> &VirtualPath {
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
    pub fn move_descriptions(&self, n: u8) -> VirtualPathBuf {
        self.dir_moves
            .join(format!("{}_moves{n}.json", self.song_name))
    }

    /// Get the path to the directory with the classifiers
    pub fn classifiers(&self) -> &VirtualPath {
        &self.dir_classifiers
    }
}
