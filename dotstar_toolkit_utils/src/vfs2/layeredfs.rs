//! # Overlay Filesystem
//! Implements a filesystem that overlays two filesystems, preferring the upper filesystem for operations.
use std::{
    borrow::Cow,
    collections::HashSet,
    io::{self, ErrorKind},
    marker::PhantomData,
    ops::Deref,
    path::Path,
};

use super::{VirtualFile, VirtualFileSystem, VirtualMetadata};
use crate::bytes_newer4::read::{ReadError, TrivialClone, ZeroCopyReadAt};

/// A filesystem that overlays two filesystems, preferring the upper filesystem for operations.
pub struct OverlayFs<'fs, FsA: VirtualFileSystem, FsB: VirtualFileSystem> {
    /// The upper filesystem, checked first for paths
    upper: &'fs FsA,
    /// The lower filesystem, checked if upper doesn't know about a path
    lower: &'fs FsB,
}

impl<'fs, FsA: VirtualFileSystem, FsB: VirtualFileSystem> OverlayFs<'fs, FsA, FsB> {
    /// Create a new overlay from two filesystems
    pub fn new(upper: &'fs FsA, lower: &'fs FsB) -> Self {
        Self { upper, lower }
    }
}

impl<'fss, FsA: VirtualFileSystem, FsB: VirtualFileSystem> VirtualFileSystem
    for OverlayFs<'fss, FsA, FsB>
{
    type VirtualFile<'fs> = OverlayFile<'fs, FsA::VirtualFile<'fs>, FsB::VirtualFile<'fs>> where FsA: 'fs, FsB: 'fs, Self: 'fs;
    type VirtualMetadata = OverlayMetadata<FsA::VirtualMetadata, FsB::VirtualMetadata>;

    fn open<'fs>(&'fs self, path: &Path) -> io::Result<Self::VirtualFile<'fs>> {
        if let Ok(file) = self.upper.open(path) {
            Ok(OverlayFile::Upper(file))
        } else if let Ok(file) = self.lower.open(path) {
            Ok(OverlayFile::Lower(file))
        } else {
            Err(ErrorKind::NotFound.into())
        }
    }

    /// Get the metadata for the file at `path`
    ///
    /// # Errors
    /// Can error if the file does not exist or if file access failed
    fn metadata(&self, path: &Path) -> std::io::Result<Self::VirtualMetadata> {
        if let Ok(metadata) = self.upper.metadata(path) {
            Ok(OverlayMetadata::Upper(metadata))
        } else if let Ok(metadata) = self.lower.metadata(path) {
            Ok(OverlayMetadata::Lower(metadata))
        } else {
            Err(ErrorKind::NotFound.into())
        }
    }

    fn list_files<'fs>(&'fs self, path: &Path) -> io::Result<impl Iterator<Item = &'fs Path>> {
        let mut paths = self.upper.list_files(path)?.collect::<HashSet<_>>();
        paths.extend(self.lower.list_files(path)?);
        Ok(paths.into_iter())
    }

    fn exists(&self, path: &Path) -> bool {
        self.upper.exists(path) || self.lower.exists(path)
    }
}

#[derive(Debug, Clone)]
pub enum OverlayMetadata<VmA: VirtualMetadata, VmB: VirtualMetadata> {
    Upper(VmA),
    Lower(VmB),
}

impl<VmA: VirtualMetadata, VmB: VirtualMetadata> VirtualMetadata for OverlayMetadata<VmA, VmB> {
    fn file_size(&self) -> u64 {
        match self {
            OverlayMetadata::Upper(upper) => upper.file_size(),
            OverlayMetadata::Lower(lower) => lower.file_size(),
        }
    }

    fn created(&self) -> std::io::Result<u64> {
        match self {
            OverlayMetadata::Upper(upper) => upper.created(),
            OverlayMetadata::Lower(lower) => lower.created(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum OverlayFile<'fs, VfA: VirtualFile<'fs>, VfB: VirtualFile<'fs>> {
    Upper(VfA),
    Lower(VfB),
    Phantom(PhantomData<&'fs u8>),
}

impl<'fs, VfA: VirtualFile<'fs>, VfB: VirtualFile<'fs>> TrivialClone
    for OverlayFile<'fs, VfA, VfB>
{
}

impl<'fs, VfA: VirtualFile<'fs>, VfB: VirtualFile<'fs>> ZeroCopyReadAt
    for OverlayFile<'fs, VfA, VfB>
{
    fn read_null_terminated_string_at<'rf>(
        &'rf self,
        position: &mut u64,
    ) -> Result<Cow<'rf, str>, ReadError> {
        match self {
            OverlayFile::Upper(upper) => upper.read_null_terminated_string_at(position),
            OverlayFile::Lower(lower) => lower.read_null_terminated_string_at(position),
            OverlayFile::Phantom(_) => unreachable!(),
        }
    }

    fn read_slice_at<'rf>(
        &'rf self,
        position: &mut u64,
        len: usize,
    ) -> Result<Cow<'rf, [u8]>, ReadError> {
        match self {
            OverlayFile::Upper(upper) => upper.read_slice_at(position, len),
            OverlayFile::Lower(lower) => lower.read_slice_at(position, len),
            OverlayFile::Phantom(_) => unreachable!(),
        }
    }
}

impl<'fs, VfA: VirtualFile<'fs>, VfB: VirtualFile<'fs>> Deref for OverlayFile<'fs, VfA, VfB> {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        match self {
            OverlayFile::Upper(file) => file.deref(),
            OverlayFile::Lower(file) => file.deref(),
            OverlayFile::Phantom(_) => unreachable!(),
        }
    }
}

impl<'fs, VfA: VirtualFile<'fs>, VfB: VirtualFile<'fs>> VirtualFile<'fs>
    for OverlayFile<'fs, VfA, VfB>
{
    fn len(&self) -> usize {
        match self {
            OverlayFile::Upper(file) => file.len(),
            OverlayFile::Lower(file) => file.len(),
            OverlayFile::Phantom(_) => unreachable!(),
        }
    }
}
