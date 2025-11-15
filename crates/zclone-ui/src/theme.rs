use gpui::*;

#[derive(Clone, Copy)]
pub struct ZCloneTheme {
    pub background: Hsla,
    pub surface: Hsla,
    pub text_primary: Hsla,
    pub text_secondary: Hsla,
    pub accent: Hsla,
    pub border: Hsla,
}

impl ZCloneTheme {
    pub fn dark() -> Self {
        Self {
            background: rgb(0x1e1e1e),
            surface: rgb(0x252525),
            text_primary: rgb(0xffffff),
            text_secondary: rgb(0xb0b0b0),
            accent: rgb(0x007aff),
            border: rgb(0x3a3a3a),
        }
    }

    pub fn light() -> Self {
        Self {
            background: rgb(0xffffff),
            surface: rgb(0xf5f5f5),
            text_primary: rgb(0x000000),
            text_secondary: rgb(0x666666),
            accent: rgb(0x007aff),
            border: rgb(0xe0e0e0),
        }
    }
}
