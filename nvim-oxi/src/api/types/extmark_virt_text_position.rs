use nvim_types::String as NvimString;

#[non_exhaustive]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
/// Controls the positioning of the virtual text associated to an extmark.
pub enum ExtmarkVirtTextPosition {
    /// Right after the EOL character (default).
    Eol,

    /// Display over the specified column, without shifting the underlying
    /// text.
    Overlay,

    /// Display right aligned in the window.
    RightAlign,
}

impl From<ExtmarkVirtTextPosition> for NvimString {
    fn from(pos: ExtmarkVirtTextPosition) -> Self {
        use ExtmarkVirtTextPosition::*;

        Self::from(match pos {
            Eol => "eol",
            Overlay => "overlay",
            RightAlign => "right_align",
        })
    }
}