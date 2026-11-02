// inline: leventInit
#[inline(always)]
pub fn leventInit(le: &mut LEvent, signaled: bool, autoclear: bool) {
    le.counter = if signaled { 2 } else { 0 };
    le.autoclear = autoclear;
}
