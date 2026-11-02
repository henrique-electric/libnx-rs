// inline: errorCodeCreate
#[inline(always)]
pub const fn errorCodeCreate(low: u32, desc: u32) -> ErrorCode {
    ErrorCode { low, desc }
}
// inline: errorCodeCreateResult
#[inline(always)]
pub const fn errorCodeCreateResult(res: Result) -> ErrorCode {
    errorCodeCreate(2000 + R_MODULE(res), R_DESCRIPTION(res))
}
// inline: errorCodeCreateInvalid
#[inline(always)]
pub const fn errorCodeCreateInvalid() -> ErrorCode {
    ErrorCode { low: 0, desc: 0 }
}
// inline: errorCodeIsValid
#[inline(always)]
pub const fn errorCodeIsValid(errorCode: ErrorCode) -> bool {
    errorCode.low != 0
}
// inline: errorResultRecordShow
/// # Safety
/// Requires the applet service to be initialized.
#[inline(always)]
pub unsafe fn errorResultRecordShow(res: Result, timestamp: u64) -> Result {
    unsafe { errorCodeRecordShow(errorCodeCreateResult(res), timestamp) }
}
// inline: errorSystemSetCode
#[inline(always)]
pub fn errorSystemSetCode(c: &mut ErrorSystemConfig, errorCode: ErrorCode) {
    c.arg.errorCode = errorCode;
}
// inline: errorSystemSetResult
#[inline(always)]
pub fn errorSystemSetResult(c: &mut ErrorSystemConfig, res: Result) {
    errorSystemSetCode(c, errorCodeCreateResult(res));
}
// inline: errorSystemSetLanguageCode
#[inline(always)]
pub fn errorSystemSetLanguageCode(c: &mut ErrorSystemConfig, LanguageCode: u64) {
    c.arg.languageCode = LanguageCode;
}
// inline: errorApplicationSetNumber
#[inline(always)]
pub fn errorApplicationSetNumber(c: &mut ErrorApplicationConfig, errorNumber: u32) {
    c.arg.errorNumber = errorNumber;
}
// inline: errorApplicationSetLanguageCode
#[inline(always)]
pub fn errorApplicationSetLanguageCode(c: &mut ErrorApplicationConfig, LanguageCode: u64) {
    c.arg.languageCode = LanguageCode;
}
