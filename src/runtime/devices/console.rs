//! `switch/runtime/devices/console.h`

#![allow(unused_imports, unused_parens)]

use core::ffi::*;

#[allow(unused_imports)]
use crate::*;
#[allow(unused_imports)]
use crate::bsd::*;
#[allow(unused_imports)]
use crate::newlib::*;

pub const CONSOLE_RESET: &CStr = c"\x1b[0m";
pub const CONSOLE_BLACK: &CStr = c"\x1b[30m";
pub const CONSOLE_RED: &CStr = c"\x1b[31;1m";
pub const CONSOLE_GREEN: &CStr = c"\x1b[32;1m";
pub const CONSOLE_YELLOW: &CStr = c"\x1b[33;1m";
pub const CONSOLE_BLUE: &CStr = c"\x1b[34;1m";
pub const CONSOLE_MAGENTA: &CStr = c"\x1b[35;1m";
pub const CONSOLE_CYAN: &CStr = c"\x1b[36;1m";
pub const CONSOLE_WHITE: &CStr = c"\x1b[37;1m";
/// Bold text
pub const CONSOLE_COLOR_BOLD: u32 = 1;
/// Faint text
pub const CONSOLE_COLOR_FAINT: u32 = 2;
/// Italic text
pub const CONSOLE_ITALIC: u32 = 4;
/// Underlined text
pub const CONSOLE_UNDERLINE: u32 = 8;
/// Slow blinking text
pub const CONSOLE_BLINK_SLOW: u32 = 16;
/// Fast blinking text
pub const CONSOLE_BLINK_FAST: u32 = 32;
/// Reversed color text
pub const CONSOLE_COLOR_REVERSE: u32 = 64;
/// Concealed text
pub const CONSOLE_CONCEAL: u32 = 128;
/// Crossed out text
pub const CONSOLE_CROSSED_OUT: u32 = 256;
/// Foreground custom color
pub const CONSOLE_FG_CUSTOM: u32 = 512;
/// Background custom color
pub const CONSOLE_BG_CUSTOM: u32 = 1024;
/// Bright foreground color
pub const CONSOLE_COLOR_FG_BRIGHT: u32 = 2048;
/// Bright background color
pub const CONSOLE_COLOR_BG_BRIGHT: u32 = 0x1000;

/// Renderer interface for the console.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct ConsoleRenderer {
    pub init: Option<unsafe extern "C" fn(*mut PrintConsole) -> bool>,
    pub deinit: Option<unsafe extern "C" fn(*mut PrintConsole)>,
    pub drawChar: Option<unsafe extern "C" fn(*mut PrintConsole, c_int, c_int, c_int)>,
    pub scrollWindow: Option<unsafe extern "C" fn(*mut PrintConsole)>,
    pub flushAndSwap: Option<unsafe extern "C" fn(*mut PrintConsole)>,
}

/// A font struct for the console.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct ConsoleFont {
    /// A pointer to the font graphics
    pub gfx: *const c_void,
    /// Offset to the first valid character in the font table
    pub asciiOffset: u16,
    /// Number of characters in the font graphics
    pub numChars: u16,
    pub tileWidth: u16,
    pub tileHeight: u16,
}

/// Console structure used to store the state of a console render context.
///
/// Default values from consoleGetDefault();
///
/// ```text
/// PrintConsole defaultConsole =
/// {
///     //Font:
///     {
///         default_font_bin, //font gfx
///         0, //first ascii character in the set
///         256, //number of characters in the font set
///         16, //tile width
///         16, //tile height
///    },
///    NULL, //renderer
///    0,0, //cursorX cursorY
///    0,0, //prevcursorX prevcursorY
///    80, //console width
///    45, //console height
///    0,  //window x
///    0,  //window y
///    80, //window width
///    45, //window height
///    3, //tab size
///    7, // foreground color
///    0, // background color
///    0, // flags
///    false //console initialized
/// };
/// ```
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PrintConsole {
    /// Font of the console
    pub font: ConsoleFont,
    /// Renderer of the console
    pub renderer: *mut ConsoleRenderer,
    /// Current X location of the cursor (as a tile offset by default)
    pub cursorX: c_int,
    /// Current Y location of the cursor (as a tile offset by default)
    pub cursorY: c_int,
    /// Internal state
    pub prevCursorX: c_int,
    /// Internal state
    pub prevCursorY: c_int,
    /// Width of the console hardware layer in characters
    pub consoleWidth: c_int,
    /// Height of the console hardware layer in characters
    pub consoleHeight: c_int,
    /// Window X location in characters
    pub windowX: c_int,
    /// Window Y location in characters
    pub windowY: c_int,
    /// Window width in characters
    pub windowWidth: c_int,
    /// Window height in characters
    pub windowHeight: c_int,
    /// Size of a tab
    pub tabSize: c_int,
    /// Foreground color
    pub fg: u16,
    /// Background color
    pub bg: u16,
    /// Reverse/bright flags
    pub flags: c_int,
    /// True if the console is initialized
    pub consoleInitialised: bool,
}

/// Console debug devices supported by libnx.
pub type debugDevice = u32;
/// Swallows prints to stderr
pub const debugDevice_NULL: debugDevice = 0;
/// Outputs stderr debug statements using svcOutputDebugString, which can then be captured by interactive debuggers
pub const debugDevice_SVC: debugDevice = 1;
/// Directs stderr debug statements to Switch console window
pub const debugDevice_CONSOLE: debugDevice = 2;

impl_zeroed_default!(ConsoleRenderer, ConsoleFont, PrintConsole);

unsafe extern "C" {
    /// Loads the font into the console.
    ///
    /// # Arguments
    /// - `console`: Pointer to the console to update, if NULL it will update the current console.
    /// - `font`: The font to load.
    pub fn consoleSetFont(console: *mut PrintConsole, font: *mut ConsoleFont);

    /// Sets the print window.
    ///
    /// # Arguments
    /// - `console`: Console to set, if NULL it will set the current console window.
    /// - `x`: X location of the window.
    /// - `y`: Y location of the window.
    /// - `width`: Width of the window.
    /// - `height`: Height of the window.
    pub fn consoleSetWindow(console: *mut PrintConsole, x: c_int, y: c_int, width: c_int, height: c_int);

    /// Gets a pointer to the console with the default values.
    /// This should only be used when using a single console or without changing the console that is returned, otherwise use consoleInit().
    ///
    /// **Returns:** A pointer to the console with the default values.
    pub fn consoleGetDefault() -> *mut PrintConsole;

    /// Make the specified console the render target.
    ///
    /// # Arguments
    /// - `console`: A pointer to the console struct (must have been initialized with consoleInit(PrintConsole* console)).
    ///
    /// **Returns:** A pointer to the previous console.
    pub fn consoleSelect(console: *mut PrintConsole) -> *mut PrintConsole;

    /// Initialise the console.
    ///
    /// # Arguments
    /// - `console`: A pointer to the console data to initialize (if it's NULL, the default console will be used).
    ///
    /// **Returns:** A pointer to the current console.
    pub fn consoleInit(console: *mut PrintConsole) -> *mut PrintConsole;

    /// Deinitialise the console.
    ///
    /// # Arguments
    /// - `console`: A pointer to the console data to initialize (if it's NULL, the default console will be used).
    pub fn consoleExit(console: *mut PrintConsole);

    /// Updates the console, submitting a new frame to the display.
    ///
    /// # Arguments
    /// - `console`: A pointer to the console data to initialize (if it's NULL, the default console will be used).
    ///
    /// **Remark:** This function should be called periodically. Failure to call this function will result in lack of screen updating.
    pub fn consoleUpdate(console: *mut PrintConsole);

    /// Initializes debug console output on stderr to the specified device.
    ///
    /// # Arguments
    /// - `device`: The debug device (or devices) to output debug print statements to.
    pub fn consoleDebugInit(device: debugDevice);

    /// Clears the screan by using printf("\x1b[2J");
    pub fn consoleClear();
}
