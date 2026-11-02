//! `switch/applets/album_la.h`

#![allow(unused_imports, unused_parens)]

use core::ffi::*;

#[allow(unused_imports)]
use crate::*;
#[allow(unused_imports)]
use crate::bsd::*;
#[allow(unused_imports)]
use crate::newlib::*;

/// Arg type values pushed for the applet input storage, stored as an u8.
pub type AlbumLaArg = u32;
/// ShowAlbumFiles. Only displays AlbumFiles associated with the application which launched the Album applet, with the filter button disabled.
pub const AlbumLaArg_ShowAlbumFiles: AlbumLaArg = 0;
/// ShowAllAlbumFiles. Displays all AlbumFiles, with filtering allowed.
pub const AlbumLaArg_ShowAllAlbumFiles: AlbumLaArg = 1;
/// ShowAllAlbumFilesForHomeMenu. Similar to ::AlbumLaArg_ShowAllAlbumFiles.
pub const AlbumLaArg_ShowAllAlbumFilesForHomeMenu: AlbumLaArg = 2;

unsafe extern "C" {
    /// Launches the applet with ::AlbumLaArg_ShowAlbumFiles and playStartupSound=false.
    pub fn albumLaShowAlbumFiles() -> Result;

    /// Launches the applet with ::AlbumLaArg_ShowAllAlbumFiles and playStartupSound=false.
    pub fn albumLaShowAllAlbumFiles() -> Result;

    /// Launches the applet with ::AlbumLaArg_ShowAllAlbumFilesForHomeMenu and playStartupSound=true.
    pub fn albumLaShowAllAlbumFilesForHomeMenu() -> Result;
}
