//! Module for the [`Key`] enum, a common interface for storing keycodes regardless of backend.

/// A [`Key`] variant represents one of the physical or logical keys on a standard keyboard.
///
/// This is a cross-backend interface. All input backends must convert their native keycodes or scan codes
/// into a [`Key`] variant so input can be handled consistently across different platforms and engines.
///
/// All explanations of key combinations are for a US keyboard.
#[expect(clippy::min_ident_chars, reason = "The variant 'A' for example is easily descriptive enough. It means the letter key 'A'.")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Key {

    /// The `Space` bar key, typically used to insert a whitespace character or execute a primary action (e.g. jumping).
    Space,

    /// The single quote / apostrophe key (`'`), which also produces a quotation mark (`"`) when combined with Shift.
    Apostrophe,

    /// The comma key (`,`), which also produces the less-than symbol (`<`) when combined with Shift.
    Comma,

    /// The minus / hyphen key (`-`), which also produces the underscore (`_`) when combined with Shift.
    Minus,

    /// The period / full stop key (`.`), which also produces the greater-than symbol (`>`) when combined with Shift.
    Period,

    /// The forward slash key (`/`), which also produces the question mark (`?`) when combined with Shift.
    Slash,

    /// The number `0` key located in the top alphanumeric row above the letter keys (not on the numeric keypad).
    Num0,

    /// The number `1` key located in the top alphanumeric row above the letter keys (not on the numeric keypad).
    Num1,

    /// The number `2` key located in the top alphanumeric row above the letter keys (not on the numeric keypad).
    Num2,

    /// The number `3` key located in the top alphanumeric row above the letter keys (not on the numeric keypad).
    Num3,

    /// The number `4` key located in the top alphanumeric row above the letter keys (not on the numeric keypad).
    Num4,

    /// The number `5` key located in the top alphanumeric row above the letter keys (not on the numeric keypad).
    Num5,

    /// The number `6` key located in the top alphanumeric row above the letter keys (not on the numeric keypad).
    Num6,

    /// The number `7` key located in the top alphanumeric row above the letter keys (not on the numeric keypad).
    Num7,

    /// The number `8` key located in the top alphanumeric row above the letter keys (not on the numeric keypad).
    Num8,

    /// The number `9` key located in the top alphanumeric row above the letter keys (not on the numeric keypad).
    Num9,

    /// The semicolon key (`;`), which also produces the colon (`:`) when combined with Shift.
    Semicolon,

    /// The equals key (`=`), which also produces the plus symbol (`+`) when combined with Shift.
    Equal,

    /// The letter key `A`.
    A,

    /// The letter key `B`.
    B,

    /// The letter key `C`.
    C,

    /// The letter key `D`.
    D,

    /// The letter key `E`.
    E,

    /// The letter key `F`.
    F,

    /// The letter key `G`.
    G,

    /// The letter key `H`.
    H,

    /// The letter key `I`.
    I,

    /// The letter key `J`.
    J,

    /// The letter key `K`.
    K,

    /// The letter key `L`.
    L,

    /// The letter key `M`.
    M,

    /// The letter key `N`.
    N,

    /// The letter key `O`.
    O,

    /// The letter key `P`.
    P,

    /// The letter key `Q`.
    Q,

    /// The letter key `R`.
    R,

    /// The letter key `S`.
    S,

    /// The letter key `T`.
    T,

    /// The letter key `U`.
    U,

    /// The letter key `V`.
    V,

    /// The letter key `W`.
    W,

    /// The letter key `X`.
    X,

    /// The letter key `Y`.
    Y,

    /// The letter key `Z`.
    Z,

    /// The left opening square bracket key (`[`), which also produces the opening curly brace (`{`) when combined with Shift.
    LeftBracket,

    /// The backslash key (`\`), which also produces the vertical pipe (`|`) when combined with Shift.
    Backslash,

    /// The right closing square bracket key (`]`), which also produces the closing curly brace (`}`) when combined with Shift.
    RightBracket,

    /// The grave accent / backtick key (`` ` ``), located below Escape on standard ANSI layouts, which also produces the tilde (`~`) when combined with Shift.
    GraveAccent,

    /// Non-standard international key 1, typically found on ISO or non-US keyboards (such as the key next to Left Shift or backslash on European layouts).
    World1,

    /// Non-standard international key 2, typically found on extended ISO or non-US regional keyboard layouts.
    World2,

    /// The `Escape` key (`Esc`), usually located in the top-left corner, commonly used to cancel operations or open pause menus.
    Escape,

    /// The main `Enter` / `Return` key on the primary keyboard section, used to commit text or execute commands.
    Enter,

    /// The `Tab` key, used for text indentation, navigating between UI focus targets, or opening menus/maps.
    Tab,

    /// The `Backspace` key, located above Enter, used to delete characters to the left of the cursor.
    Backspace,

    /// The `Insert` key (`Ins`), used to toggle between text insertion and overwrite modes.
    Insert,

    /// The `Delete` key (`Del`), used to delete characters to the right of the cursor or remove selected objects.
    Delete,

    /// The `Right Arrow` navigation key, used to move selection or camera to the right.
    Right,

    /// The `Left Arrow` navigation key, used to move selection or camera to the left.
    Left,

    /// The `Down Arrow` navigation key, used to move selection or camera downward.
    Down,

    /// The `Up Arrow` navigation key, used to move selection or camera upward.
    Up,

    /// The `Page Up` key (`PgUp`), used to scroll content or viewports upwards by a full screen increment.
    PageUp,

    /// The `Page Down` key (`PgDn`), used to scroll content or viewports downwards by a full screen increment.
    PageDown,

    /// The `Home` key, used to move the cursor or viewport to the beginning of a line, document, or container.
    Home,

    /// The `End` key, used to move the cursor or viewport to the end of a line, document, or container.
    End,

    /// The `Caps Lock` toggle key, used to lock character input into uppercase mode.
    CapsLock,

    /// The `Scroll Lock` key, an older control key originally used to modify arrow key behavior to scroll text rather than move the cursor.
    ScrollLock,

    /// The `Num Lock` key, used to toggle the dedicated numeric keypad between numeric entry and navigation modes.
    NumLock,

    /// The `Print Screen` key (`PrtScn` / `SysRq`), used to capture screenshots or send system diagnostic requests.
    PrintScreen,

    /// The `Pause` / `Break` key, traditionally used to temporarily halt terminal output or pause execution.
    Pause,

    /// Function key `F1`, commonly bound to in-application Help or user manuals.
    F1,

    /// Function key `F2`, commonly used for renaming files or toggling tools.
    F2,

    /// Function key `F3`, commonly used for search/find functions or debug overlays.
    F3,

    /// Function key `F4`, often used in keyboard shortcuts to close windows or elements.
    F4,

    /// Function key `F5`, commonly used to refresh content or save rapid game progress.
    F5,

    /// Function key `F6`.
    F6,

    /// Function key `F7`.
    F7,

    /// Function key `F8`.
    F8,

    /// Function key `F9`.
    F9,

    /// Function key `F10`, often used to activate the application menu bar.
    F10,

    /// Function key `F11`, commonly used to toggle fullscreen display mode.
    F11,

    /// Function key `F12`, commonly used to open developer tools or capture screenshots.
    F12,

    /// Extended function key `F13`, available on expanded keyboards or programmable macropads.
    F13,

    /// Extended function key `F14`, available on expanded keyboards or programmable macropads.
    F14,

    /// Extended function key `F15`, available on expanded keyboards or programmable macropads.
    F15,

    /// Extended function key `F16`, available on expanded keyboards or programmable macropads.
    F16,

    /// Extended function key `F17`, available on expanded keyboards or programmable macropads.
    F17,

    /// Extended function key `F18`, available on expanded keyboards or programmable macropads.
    F18,

    /// Extended function key `F19`, available on expanded keyboards or programmable macropads.
    F19,

    /// Extended function key `F20`, available on expanded keyboards or programmable macropads.
    F20,

    /// Extended function key `F21`, available on expanded keyboards or programmable macropads.
    F21,

    /// Extended function key `F22`, available on expanded keyboards or programmable macropads.
    F22,

    /// Extended function key `F23`, available on expanded keyboards or programmable macropads.
    F23,

    /// Extended function key `F24`, available on expanded keyboards or programmable macropads.
    F24,

    /// Extended function key `F25`, available on specialized industrial or macro keyboards.
    F25,

    /// The `0` key located exclusively on the dedicated numeric keypad (Numpad).
    Keypad0,

    /// The `1` key located exclusively on the dedicated numeric keypad (Numpad).
    Keypad1,

    /// The `2` key located exclusively on the dedicated numeric keypad (Numpad).
    Keypad2,

    /// The `3` key located exclusively on the dedicated numeric keypad (Numpad).
    Keypad3,

    /// The `4` key located exclusively on the dedicated numeric keypad (Numpad).
    Keypad4,

    /// The `5` key located exclusively on the dedicated numeric keypad (Numpad).
    Keypad5,

    /// The `6` key located exclusively on the dedicated numeric keypad (Numpad).
    Keypad6,

    /// The `7` key located exclusively on the dedicated numeric keypad (Numpad).
    Keypad7,

    /// The `8` key located exclusively on the dedicated numeric keypad (Numpad).
    Keypad8,

    /// The `9` key located exclusively on the dedicated numeric keypad (Numpad).
    Keypad9,

    /// The decimal point / full stop key (`.`) located exclusively on the dedicated numeric keypad.
    KeypadDecimal,

    /// The division / forward slash key (`/`) located exclusively on the dedicated numeric keypad.
    KeypadDivide,

    /// The multiplication / asterisk key (`*`) located exclusively on the dedicated numeric keypad.
    KeypadMultiply,

    /// The subtraction / minus key (`-`) located exclusively on the dedicated numeric keypad.
    KeypadSubtract,

    /// The addition / plus key (`+`) located exclusively on the dedicated numeric keypad.
    KeypadAdd,

    /// The dedicated `Enter` key located on the bottom-right of the numeric keypad.
    KeypadEnter,

    /// The equals key (`=`) located on specialized or extended numeric keypads (common on Apple keypads).
    KeypadEqual,

    /// The modifier key `Shift` on the left side of the keyboard.
    LeftShift,

    /// The modifier key `Control` (`Ctrl`) on the left side of the keyboard.
    LeftControl,

    /// The modifier key `Alt` / `Option` on the left side of the keyboard.
    LeftAlt,

    /// The system modifier key on the left side (Windows key on PC, Command key `⌘` on macOS, Super key on Linux).
    LeftSuper,

    /// The modifier key `Shift` on the right side of the keyboard.
    RightShift,

    /// The modifier key `Control` (`Ctrl`) on the right side of the keyboard.
    RightControl,

    /// The modifier key `Alt` / `AltGr` / `Option` on the right side of the keyboard.
    RightAlt,

    /// The system modifier key on the right side (Windows key on PC, Command key `⌘` on macOS, Super key on Linux).
    RightSuper,

    /// The `Menu` / `Application` key, usually located near the right Control key, used to open context menus.
    Menu,

}
