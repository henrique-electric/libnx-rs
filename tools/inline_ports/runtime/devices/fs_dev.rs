// inline: fsdevDirGetEntries
/// Returns the entry buffer that directly follows `dir` in memory.
#[inline(always)]
pub fn fsdevDirGetEntries(dir: *mut fsdev_dir_t) -> *mut FsDirectoryEntry {
    dir.wrapping_add(1).cast()
}
