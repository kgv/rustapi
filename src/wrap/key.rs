use crate::um::winuser::{GetAsyncKeyState, GetKeyState};
use anyhow::Result;
use derive_more::From;
use winapi::um::winuser::{
    VK_ADD, VK_APPS, VK_BACK, VK_CAPITAL, VK_CONTROL, VK_DECIMAL, VK_DELETE, VK_DIVIDE, VK_DOWN,
    VK_END, VK_ESCAPE, VK_F1, VK_F10, VK_F11, VK_F12, VK_F13, VK_F14, VK_F15, VK_F16, VK_F17,
    VK_F18, VK_F19, VK_F2, VK_F20, VK_F21, VK_F22, VK_F23, VK_F24, VK_F3, VK_F4, VK_F5, VK_F6,
    VK_F7, VK_F8, VK_F9, VK_HOME, VK_INSERT, VK_LCONTROL, VK_LEFT, VK_LMENU, VK_LSHIFT, VK_LWIN,
    VK_MENU, VK_MULTIPLY, VK_NEXT, VK_NUMLOCK, VK_NUMPAD0, VK_NUMPAD1, VK_NUMPAD2, VK_NUMPAD3,
    VK_NUMPAD4, VK_NUMPAD5, VK_NUMPAD6, VK_NUMPAD7, VK_NUMPAD8, VK_NUMPAD9, VK_OEM_1, VK_OEM_102,
    VK_OEM_2, VK_OEM_3, VK_OEM_4, VK_OEM_5, VK_OEM_6, VK_OEM_7, VK_OEM_8, VK_OEM_CLEAR,
    VK_OEM_COMMA, VK_OEM_MINUS, VK_OEM_PERIOD, VK_OEM_PLUS, VK_PRIOR, VK_RCONTROL, VK_RETURN,
    VK_RIGHT, VK_RMENU, VK_RSHIFT, VK_RWIN, VK_SCROLL, VK_SEPARATOR, VK_SHIFT, VK_SNAPSHOT,
    VK_SPACE, VK_SUBTRACT, VK_TAB, VK_UP,
};

/// Key.
#[derive(Clone, Copy, From, Debug)]
pub enum Key {
    /// 0 key
    Zero,
    /// 1 key
    One,
    /// 2 key
    Two,
    /// 3 key
    Three,
    /// 4 key
    Four,
    /// 5 key
    Five,
    /// 6 key
    Six,
    /// 7 key
    Seven,
    /// 8 key
    Eight,
    /// 9 key
    Nine,

    /// A key
    A,
    /// B key
    B,
    /// C key
    C,
    /// D key
    D,
    /// E key
    E,
    /// F key
    F,
    /// G key
    G,
    /// H key
    H,
    /// I key
    I,
    /// J key
    J,
    /// K key
    K,
    /// L key
    L,
    /// M key
    M,
    /// N key
    N,
    /// O key
    O,
    /// P key
    P,
    /// Q key
    Q,
    /// R key
    R,
    /// S key
    S,
    /// T key
    T,
    /// U key
    U,
    /// V key
    V,
    /// W key
    W,
    /// X key
    X,
    /// Y key
    Y,
    /// Z key
    Z,

    /// ESC key
    Escape,

    /// BACKSPACE key
    Backspace,
    /// CAPS LOCK key
    CapsLock,
    /// ENTER key
    Enter,
    /// SPACEBAR
    Spacebar,
    /// TAB key
    Tab,

    /// ALT key
    Alt,
    /// CTRL key
    Ctrl,
    /// Left CONTROL key
    LeftCtrl,
    /// Right CONTROL key
    RightCtrl,
    /// SHIFT key
    Shift,
    /// Left SHIFT key
    LeftShift,
    /// Right SHIFT key
    RightShift,
    /// Left MENU key
    LeftAlt,
    /// Right MENU key
    RightAlt,

    /// Applications key (Natural keyboard)
    Applications,
    /// Left Windows key (Natural keyboard)
    LeftWindows,
    /// Right Windows key (Natural keyboard)
    RightWindows,

    /// END key
    End,
    /// DEL key
    Delete,
    /// HOME key
    Home,
    /// INS key
    Insert,
    /// PAGE UP key
    PageUp,
    /// PAGE DOWN key
    PageDown,
    /// PRINT SCREEN key
    PrintScreen,

    /// NUM LOCK key
    NumLock,
    /// SCROLL LOCK key
    ScrollLock,

    /// Control-break processing
    ControlBreak,
    /// Computer Sleep key
    ComputerSleep,

    /// CLEAR key
    Clear,
    /// PAUSE key
    Pause,
    /// SELECT key
    Select,
    /// PRINT key
    Print,
    /// EXECUTE key
    Execute,
    /// HELP key
    Help,

    /// Used to pass Unicode characters as if they were keystrokes. The VK_PACKET key is the low word of a 32-bit Virtual Key value used for non-keyboard input methods. For more information, see Remark in KEYBDINPUT, SendInput, WM_KEYDOWN, and WM_KEYUP
    Packet,
    /// Attn key
    Attn,
    /// CrSel key
    CrSel,
    /// ExSel key
    ExSel,
    /// Erase EOF key
    EraseEof,
    /// Play key
    Play,
    /// Zoom key
    Zoom,
    /// Reserved
    Noname,
    /// PA1 key
    PA1,

    /// Browser
    Browser(Browser),
    /// Cursor control
    CursorControl(CursorControl),
    /// Function
    Function(Function),
    /// Media
    Media(Media),
    /// Mouse
    Mouse(Mouse),
    /// NumericKeypad
    NumPad(NumericKeypad),
    /// Volume
    Volume(Volume),
    /// OEM
    Oem(Oem),
    // /// IME
    // Ime(Ime),
}

impl Key {
    pub fn absolute(self) -> bool {
        self.async_state_lossy().0
    }

    pub fn async_state(self) -> Result<(bool, bool)> {
        GetAsyncKeyState::builder().key(self).build()()
    }

    pub fn async_state_lossy(self) -> (bool, bool) {
        self.async_state().unwrap_or_default()
    }

    pub fn relative(self) -> bool {
        self.async_state_lossy().1
    }

    pub fn state(self) -> (bool, bool) {
        GetKeyState::builder().virtual_key(self).build()()
    }
}

// impl From<Browser> for Key {
//     fn from(from: Browser) -> Self {
//         Key::Browser(from)
//     }
// }

// impl From<CursorControl> for Key {
//     fn from(from: CursorControl) -> Self {
//         Key::CursorControl(from)
//     }
// }

// impl From<Function> for Key {
//     fn from(from: Function) -> Self {
//         Key::Function(from)
//     }
// }

// impl From<Media> for Key {
//     fn from(from: Media) -> Self {
//         Key::Media(from)
//     }
// }

// impl From<Mouse> for Key {
//     fn from(from: Mouse) -> Self {
//         Key::Mouse(from)
//     }
// }

// impl From<NumericKeypad> for Key {
//     fn from(from: NumericKeypad) -> Self {
//         Key::NumPad(from)
//     }
// }

// impl From<Oem> for Key {
//     fn from(from: Oem) -> Self {
//         Key::Oem(from)
//     }
// }

// impl From<Volume> for Key {
//     fn from(from: Volume) -> Self {
//         Key::Volume(from)
//     }
// }

impl From<Key> for i32 {
    fn from(from: Key) -> Self {
        match from {
            Key::Zero => 0x30,
            Key::One => 0x31,
            Key::Two => 0x32,
            Key::Three => 0x33,
            Key::Four => 0x34,
            Key::Five => 0x35,
            Key::Six => 0x36,
            Key::Seven => 0x37,
            Key::Eight => 0x38,
            Key::Nine => 0x39,
            Key::A => 0x41,
            Key::B => 0x42,
            Key::C => 0x43,
            Key::D => 0x44,
            Key::E => 0x45,
            Key::F => 0x46,
            Key::G => 0x47,
            Key::H => 0x48,
            Key::I => 0x49,
            Key::J => 0x4A,
            Key::K => 0x4B,
            Key::L => 0x4C,
            Key::M => 0x4D,
            Key::N => 0x4E,
            Key::O => 0x4F,
            Key::P => 0x50,
            Key::Q => 0x51,
            Key::R => 0x52,
            Key::S => 0x53,
            Key::T => 0x54,
            Key::U => 0x55,
            Key::V => 0x56,
            Key::W => 0x57,
            Key::X => 0x58,
            Key::Y => 0x59,
            Key::Z => 0x5A,

            Key::Alt => VK_MENU,
            Key::Applications => VK_APPS,
            Key::Backspace => VK_BACK,
            Key::CapsLock => VK_CAPITAL,
            Key::Ctrl => VK_CONTROL,
            Key::Delete => VK_DELETE,
            Key::End => VK_END,
            Key::Enter => VK_RETURN,
            Key::Escape => VK_ESCAPE,
            Key::Home => VK_HOME,
            Key::Insert => VK_INSERT,
            Key::LeftAlt => VK_LMENU,
            Key::LeftCtrl => VK_LCONTROL,
            Key::LeftShift => VK_LSHIFT,
            Key::LeftWindows => VK_LWIN,
            Key::NumLock => VK_NUMLOCK,
            Key::PageDown => VK_NEXT,
            Key::PageUp => VK_PRIOR,
            Key::PrintScreen => VK_SNAPSHOT,
            Key::RightAlt => VK_RMENU,
            Key::RightCtrl => VK_RCONTROL,
            Key::RightShift => VK_RSHIFT,
            Key::RightWindows => VK_RWIN,
            Key::ScrollLock => VK_SCROLL,
            Key::Shift => VK_SHIFT,
            Key::Spacebar => VK_SPACE,
            Key::Tab => VK_TAB,
            // Cursor control.
            Key::CursorControl(CursorControl::LeftArrow) => VK_LEFT,
            Key::CursorControl(CursorControl::UpArrow) => VK_UP,
            Key::CursorControl(CursorControl::RightArrow) => VK_RIGHT,
            Key::CursorControl(CursorControl::DownArrow) => VK_DOWN,
            // Function.
            Key::Function(Function::F1) => VK_F1,
            Key::Function(Function::F2) => VK_F2,
            Key::Function(Function::F3) => VK_F3,
            Key::Function(Function::F4) => VK_F4,
            Key::Function(Function::F5) => VK_F5,
            Key::Function(Function::F6) => VK_F6,
            Key::Function(Function::F7) => VK_F7,
            Key::Function(Function::F8) => VK_F8,
            Key::Function(Function::F9) => VK_F9,
            Key::Function(Function::F10) => VK_F10,
            Key::Function(Function::F11) => VK_F11,
            Key::Function(Function::F12) => VK_F12,
            Key::Function(Function::F13) => VK_F13,
            Key::Function(Function::F14) => VK_F14,
            Key::Function(Function::F15) => VK_F15,
            Key::Function(Function::F16) => VK_F16,
            Key::Function(Function::F17) => VK_F17,
            Key::Function(Function::F18) => VK_F18,
            Key::Function(Function::F19) => VK_F19,
            Key::Function(Function::F20) => VK_F20,
            Key::Function(Function::F21) => VK_F21,
            Key::Function(Function::F22) => VK_F22,
            Key::Function(Function::F23) => VK_F23,
            Key::Function(Function::F24) => VK_F24,
            // Numeric keypad.
            Key::NumPad(NumericKeypad::Add) => VK_ADD,
            Key::NumPad(NumericKeypad::Decimal) => VK_DECIMAL,
            Key::NumPad(NumericKeypad::Divide) => VK_DIVIDE,
            Key::NumPad(NumericKeypad::Multiply) => VK_MULTIPLY,
            Key::NumPad(NumericKeypad::Separator) => VK_SEPARATOR,
            Key::NumPad(NumericKeypad::Subtract) => VK_SUBTRACT,
            Key::NumPad(NumericKeypad::Zero) => VK_NUMPAD0,
            Key::NumPad(NumericKeypad::One) => VK_NUMPAD1,
            Key::NumPad(NumericKeypad::Two) => VK_NUMPAD2,
            Key::NumPad(NumericKeypad::Three) => VK_NUMPAD3,
            Key::NumPad(NumericKeypad::Four) => VK_NUMPAD4,
            Key::NumPad(NumericKeypad::Five) => VK_NUMPAD5,
            Key::NumPad(NumericKeypad::Six) => VK_NUMPAD6,
            Key::NumPad(NumericKeypad::Seven) => VK_NUMPAD7,
            Key::NumPad(NumericKeypad::Eight) => VK_NUMPAD8,
            Key::NumPad(NumericKeypad::Nine) => VK_NUMPAD9,
            // Oem.
            Key::Oem(Oem::_1) => VK_OEM_1,
            Key::Oem(Oem::_2) => VK_OEM_2,
            Key::Oem(Oem::_3) => VK_OEM_3,
            Key::Oem(Oem::_4) => VK_OEM_4,
            Key::Oem(Oem::_5) => VK_OEM_5,
            Key::Oem(Oem::_6) => VK_OEM_6,
            Key::Oem(Oem::_7) => VK_OEM_7,
            Key::Oem(Oem::_8) => VK_OEM_8,
            Key::Oem(Oem::_102) => VK_OEM_102,
            Key::Oem(Oem::Clear) => VK_OEM_CLEAR,
            Key::Oem(Oem::Comma) => VK_OEM_COMMA,
            Key::Oem(Oem::Minus) => VK_OEM_MINUS,
            Key::Oem(Oem::Period) => VK_OEM_PERIOD,
            Key::Oem(Oem::Plus) => VK_OEM_PLUS,
            _ => unimplemented!(),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Browser {
    /// Browser Back key.
    Back,
    /// Browser Forward key.
    Forward,
    /// Browser Refresh key.
    Refresh,
    /// Browser Stop key.
    Stop,
    /// Browser Search key.
    Search,
    /// Browser Favorites key.
    Favorites,
    /// Browser Start and Home key.
    Home,
}

#[derive(Clone, Copy, Debug)]
pub enum CursorControl {
    /// LEFT ARROW key.
    LeftArrow,
    /// UP ARROW key.
    UpArrow,
    /// RIGHT ARROW key.
    RightArrow,
    /// DOWN ARROW key.
    DownArrow,
}

#[derive(Clone, Copy, Debug)]
pub enum Function {
    /// F1 key.
    F1,
    /// F2 key.
    F2,
    /// F3 key.
    F3,
    /// F4 key.
    F4,
    /// F5 key.
    F5,
    /// F6 key.
    F6,
    /// F7 key.
    F7,
    /// F8 key.
    F8,
    /// F9 key.
    F9,
    /// F10 key.
    F10,
    /// F11 key.
    F11,
    /// F12 key.
    F12,
    /// F13 key.
    F13,
    /// F14 key.
    F14,
    /// F15 key.
    F15,
    /// F16 key.
    F16,
    /// F17 key.
    F17,
    /// F18 key.
    F18,
    /// F19 key.
    F19,
    /// F20 key.
    F20,
    /// F21 key.
    F21,
    /// F22 key.
    F22,
    /// F23 key.
    F23,
    /// F24 key.
    F24,
}

#[derive(Clone, Copy, Debug)]
pub enum Ime {
    /// IME convert.
    Convert,
    /// IME nonconvert.
    NonConvert,
    /// IME accept.
    Accept,
    /// IME mode change request.
    ModeChangeRequest,
    /// IME Kana mode.
    KanaMode,
    /// IME Hanguel mode (maintained for compatibility; use VK_HANGUL).
    HanguelMode,
    /// IME Hangul mode.
    HangulMode,
    /// IME Junja mode.
    JunjaMode,
    /// IME final mode.
    FinalMode,
    /// IME Hanja mode.
    HanjaMode,
    /// IME Kanji mode.
    KanjiMode,
    /// IME PROCESS key.
    Process,
}

#[derive(Clone, Copy, Debug)]
pub enum Launch {
    /// Start Mail key.
    Mail,
    /// Select Media key.
    MediaSelect,
    /// Start Application 1 key.
    Application1,
    /// Start Application 2 key.
    Application2,
}

#[derive(Clone, Copy, Debug)]
pub enum Media {
    /// Next Track key.
    NextTrack,
    /// Previous Track key.
    PreviousTrack,
    /// Stop Media key.
    Stop,
    /// Play/Pause Media key.
    PlayPause,
}

#[derive(Clone, Copy, Debug)]
pub enum Mouse {
    /// Left mouse button.
    LeftButton,
    /// Right mouse button.
    RightButton,
    /// Middle mouse button (three-button mouse).
    MiddleButton,
    /// X1 mouse button.
    X1Button,
    /// X2 mouse button.
    X2Button,
}

#[derive(Clone, Copy, Debug)]
pub enum NumericKeypad {
    /// Add key (+).
    Add,
    /// Decimal key (.).
    Decimal,
    /// Divide key (/).
    Divide,
    /// Multiply key (*).
    Multiply,
    /// Separator key (\).
    Separator,
    /// Subtract key (-).
    Subtract,

    /// Numeric keypad 0 key.
    Zero,
    /// Numeric keypad 1 key.
    One,
    /// Numeric keypad 2 key.
    Two,
    /// Numeric keypad 3 key.
    Three,
    /// Numeric keypad 4 key.
    Four,
    /// Numeric keypad 5 key.
    Five,
    /// Numeric keypad 6 key.
    Six,
    /// Numeric keypad 7 key.
    Seven,
    /// Numeric keypad 8 key.
    Eight,
    /// Numeric keypad 9 key.
    Nine,
}

#[derive(Clone, Copy, Debug)]
pub enum Oem {
    /// Used for miscellaneous characters; it can vary by keyboard. For the US standard keyboard, the ';:' key.
    _1,
    /// Used for miscellaneous characters; it can vary by keyboard. For the US standard keyboard, the '/?' key.
    _2,
    /// Used for miscellaneous characters; it can vary by keyboard. For the US standard keyboard, the '`~' key.
    _3,
    /// Used for miscellaneous characters; it can vary by keyboard. For the US standard keyboard, the '[{' key.
    _4,
    /// Used for miscellaneous characters; it can vary by keyboard. For the US standard keyboard, the '\|' key.
    _5,
    /// Used for miscellaneous characters; it can vary by keyboard. For the US standard keyboard, the ']}' key.
    _6,
    /// Used for miscellaneous characters; it can vary by keyboard. For the US standard keyboard, the 'single-quote/double-quote' key.
    _7,
    /// Used for miscellaneous characters; it can vary by keyboard..
    _8,
    /// Either the angle bracket key or the backslash key on the RT 102-key keyboard.
    _102,

    /// Clear key.
    Clear,
    /// For any country/region, the ',' key.
    Comma,
    /// For any country/region, the '-' key.
    Minus,
    /// For any country/region, the '.' key.
    Period,
    /// For any country/region, the '+' key.
    Plus,
}

#[derive(Clone, Copy, Debug)]
pub enum Volume {
    /// Volume Mute key.
    Mute,
    /// Volume Down key.
    Down,
    /// Volume Up key.
    Up,
}
