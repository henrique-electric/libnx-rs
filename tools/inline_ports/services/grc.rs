// inline: grcMovieMakerGetNWindow
#[inline(always)]
pub fn grcMovieMakerGetNWindow(m: &mut GrcMovieMaker) -> &mut NWindow {
    &mut m.win
}
