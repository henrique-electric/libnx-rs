// inline: capsGetDefaultStartDateTime
#[inline(always)]
pub fn capsGetDefaultStartDateTime() -> CapsAlbumFileDateTime {
    CapsAlbumFileDateTime { year: 1970, month: 1, day: 1, ..Default::default() }
}
// inline: capsGetDefaultEndDateTime
#[inline(always)]
pub fn capsGetDefaultEndDateTime() -> CapsAlbumFileDateTime {
    CapsAlbumFileDateTime { year: 3000, month: 1, day: 1, ..Default::default() }
}
// inline: capsConvertApplicationAlbumFileEntryToApplicationAlbumEntry
#[inline(always)]
pub fn capsConvertApplicationAlbumFileEntryToApplicationAlbumEntry(out: &mut CapsApplicationAlbumEntry, in_: &CapsApplicationAlbumFileEntry) {
    *out = in_.entry;
}
// inline: capsConvertApplicationAlbumEntryToApplicationAlbumFileEntry
#[inline(always)]
pub fn capsConvertApplicationAlbumEntryToApplicationAlbumFileEntry(out: &mut CapsApplicationAlbumFileEntry, in_: &CapsApplicationAlbumEntry) {
    out.entry = *in_;
    // SAFETY: every bit pattern of the entry union is a valid `v1`.
    out.datetime = unsafe { in_.anon0.v1.datetime };
    out.unk_x28 = 0;
}
