pub enum ColorString {
    Reset,
    // Foreground colors
    FgBlue,
    FgRed,
    // Background colors
    BgBlack,
    BgWhite,
}

impl ColorString {
    pub fn as_str(&self) -> &'static str {
        match self {
            ColorString::Reset => "\x1b[0m",
            // Foreground colors
            ColorString::FgBlue => "\x1b[34m",
            ColorString::FgRed => "\x1b[31m",
            // Background colors
            ColorString::BgBlack => "\x1b[40m",
            ColorString::BgWhite => "\x1b[47m",
        }
    }
}
