use gpui::*;

/// Typography system using SF Pro Display, SF Pro Text, and SF Mono
#[derive(Clone, Debug)]
pub struct Typography {
    /// Large display text (titles, headers)
    pub display_large: TextStyle,
    /// Medium display text
    pub display_medium: TextStyle,
    /// Small display text
    pub display_small: TextStyle,
    /// Body text large
    pub body_large: TextStyle,
    /// Body text medium (default)
    pub body_medium: TextStyle,
    /// Body text small
    pub body_small: TextStyle,
    /// Code blocks (SF Mono)
    pub code: TextStyle,
    /// Inline code (SF Mono)
    pub code_inline: TextStyle,
}

impl Typography {
    pub fn new() -> Self {
        Self {
            display_large: TextStyle {
                font_family: "SF Pro Display".into(),
                font_size: px(28.0).into(),
                font_weight: FontWeight::BOLD,
                line_height: px(34.0).into(),
                ..Default::default()
            },
            display_medium: TextStyle {
                font_family: "SF Pro Display".into(),
                font_size: px(22.0).into(),
                font_weight: FontWeight::SEMIBOLD,
                line_height: px(28.0).into(),
                ..Default::default()
            },
            display_small: TextStyle {
                font_family: "SF Pro Display".into(),
                font_size: px(18.0).into(),
                font_weight: FontWeight::SEMIBOLD,
                line_height: px(24.0).into(),
                ..Default::default()
            },
            body_large: TextStyle {
                font_family: "SF Pro Text".into(),
                font_size: px(16.0).into(),
                font_weight: FontWeight::NORMAL,
                line_height: px(24.0).into(),
                ..Default::default()
            },
            body_medium: TextStyle {
                font_family: "SF Pro Text".into(),
                font_size: px(14.0).into(),
                font_weight: FontWeight::NORMAL,
                line_height: px(20.0).into(),
                ..Default::default()
            },
            body_small: TextStyle {
                font_family: "SF Pro Text".into(),
                font_size: px(12.0).into(),
                font_weight: FontWeight::NORMAL,
                line_height: px(18.0).into(),
                ..Default::default()
            },
            code: TextStyle {
                font_family: "SF Mono".into(),
                font_size: px(14.0).into(),
                font_weight: FontWeight::NORMAL,
                line_height: px(20.0).into(),
                ..Default::default()
            },
            code_inline: TextStyle {
                font_family: "SF Mono".into(),
                font_size: px(13.0).into(),
                font_weight: FontWeight::NORMAL,
                line_height: px(18.0).into(),
                ..Default::default()
            },
        }
    }
}

impl Default for Typography {
    fn default() -> Self {
        Self::new()
    }
}

/// Spacing system using 8px base unit
#[derive(Clone, Copy, Debug)]
pub struct Spacing {
    pub xs: Pixels,      // 4px
    pub sm: Pixels,      // 8px
    pub md: Pixels,      // 16px
    pub lg: Pixels,      // 24px
    pub xl: Pixels,      // 32px
    pub xxl: Pixels,     // 48px
    pub xxxl: Pixels,    // 64px
}

impl Spacing {
    pub fn new() -> Self {
        Self {
            xs: px(4.0),
            sm: px(8.0),
            md: px(16.0),
            lg: px(24.0),
            xl: px(32.0),
            xxl: px(48.0),
            xxxl: px(64.0),
        }
    }
}

impl Default for Spacing {
    fn default() -> Self {
        Self::new()
    }
}

/// Color palette with semantic tokens
#[derive(Clone, Copy, Debug)]
pub struct ColorPalette {
    // Surface colors (backgrounds)
    pub surface_primary: Hsla,      // Main background
    pub surface_secondary: Hsla,    // Elevated surface
    pub surface_tertiary: Hsla,     // Higher elevation
    pub surface_active: Hsla,       // Active/selected state
    pub surface_hover: Hsla,        // Hover state

    // Text colors
    pub text_primary: Hsla,         // Primary text (WCAG AAA: 7:1)
    pub text_secondary: Hsla,       // Secondary text (WCAG AA: 4.5:1)
    pub text_tertiary: Hsla,        // Tertiary text (WCAG AA: 4.5:1)
    pub text_inverse: Hsla,         // Text on accent colors
    pub text_disabled: Hsla,        // Disabled state

    // Border colors
    pub border_primary: Hsla,       // Default borders
    pub border_secondary: Hsla,     // Subtle borders
    pub border_focus: Hsla,         // Focus state

    // Semantic colors
    pub accent: Hsla,               // Primary accent (blue)
    pub accent_hover: Hsla,         // Accent hover state
    pub success: Hsla,              // Success state (green)
    pub warning: Hsla,              // Warning state (yellow)
    pub error: Hsla,                // Error state (red)

    // Message bubble colors
    pub message_user: Hsla,         // User message background
    pub message_assistant: Hsla,    // Assistant message background
    pub message_system: Hsla,       // System message background
    pub message_code_bg: Hsla,      // Code block background

    // Sidebar colors
    pub sidebar_bg: Hsla,           // Sidebar background
    pub sidebar_item_active: Hsla,  // Active session item
    pub sidebar_item_hover: Hsla,   // Hover state
}

impl ColorPalette {
    /// Dark mode palette with high contrast ratios
    pub fn dark() -> Self {
        Self {
            // Surface colors - Dark theme
            surface_primary: hsla(0.0, 0.0, 0.11, 1.0),      // #1C1C1C
            surface_secondary: hsla(0.0, 0.0, 0.14, 1.0),    // #242424
            surface_tertiary: hsla(0.0, 0.0, 0.18, 1.0),     // #2E2E2E
            surface_active: hsla(0.0, 0.0, 0.22, 1.0),       // #383838
            surface_hover: hsla(0.0, 0.0, 0.16, 0.8),        // #292929 with opacity

            // Text colors - optimized for dark bg (contrast > 7:1 for AAA)
            text_primary: hsla(0.0, 0.0, 0.96, 1.0),         // #F5F5F5
            text_secondary: hsla(0.0, 0.0, 0.72, 1.0),       // #B8B8B8
            text_tertiary: hsla(0.0, 0.0, 0.56, 1.0),        // #8F8F8F
            text_inverse: hsla(0.0, 0.0, 0.08, 1.0),         // #141414
            text_disabled: hsla(0.0, 0.0, 0.40, 1.0),        // #666666

            // Border colors
            border_primary: hsla(0.0, 0.0, 0.24, 1.0),       // #3D3D3D
            border_secondary: hsla(0.0, 0.0, 0.18, 1.0),     // #2E2E2E
            border_focus: hsla(211.0, 1.0, 0.55, 1.0),       // #0A84FF (blue)

            // Semantic colors
            accent: hsla(211.0, 1.0, 0.55, 1.0),             // #0A84FF (Apple blue)
            accent_hover: hsla(211.0, 1.0, 0.65, 1.0),       // Lighter blue
            success: hsla(142.0, 0.70, 0.45, 1.0),           // #34C759 (Apple green)
            warning: hsla(48.0, 1.0, 0.55, 1.0),             // #FFD60A (Apple yellow)
            error: hsla(4.0, 0.90, 0.58, 1.0),               // #FF453A (Apple red)

            // Message bubbles
            message_user: hsla(211.0, 0.20, 0.20, 1.0),      // Subtle blue tint
            message_assistant: hsla(0.0, 0.0, 0.14, 1.0),    // Surface secondary
            message_system: hsla(48.0, 0.15, 0.18, 1.0),     // Subtle yellow tint
            message_code_bg: hsla(0.0, 0.0, 0.10, 1.0),      // Darker for code

            // Sidebar
            sidebar_bg: hsla(0.0, 0.0, 0.09, 1.0),           // #171717
            sidebar_item_active: hsla(211.0, 0.25, 0.18, 1.0), // Blue tint
            sidebar_item_hover: hsla(0.0, 0.0, 0.14, 0.6),   // Subtle hover
        }
    }

    /// Light mode palette with high contrast ratios
    pub fn light() -> Self {
        Self {
            // Surface colors - Light theme
            surface_primary: hsla(0.0, 0.0, 1.0, 1.0),       // #FFFFFF
            surface_secondary: hsla(0.0, 0.0, 0.97, 1.0),    // #F7F7F7
            surface_tertiary: hsla(0.0, 0.0, 0.94, 1.0),     // #F0F0F0
            surface_active: hsla(0.0, 0.0, 0.91, 1.0),       // #E8E8E8
            surface_hover: hsla(0.0, 0.0, 0.95, 0.8),        // #F2F2F2 with opacity

            // Text colors - optimized for light bg (contrast > 7:1 for AAA)
            text_primary: hsla(0.0, 0.0, 0.12, 1.0),         // #1E1E1E
            text_secondary: hsla(0.0, 0.0, 0.38, 1.0),       // #616161
            text_tertiary: hsla(0.0, 0.0, 0.52, 1.0),        // #858585
            text_inverse: hsla(0.0, 0.0, 0.98, 1.0),         // #FAFAFA
            text_disabled: hsla(0.0, 0.0, 0.68, 1.0),        // #ADADAD

            // Border colors
            border_primary: hsla(0.0, 0.0, 0.85, 1.0),       // #D9D9D9
            border_secondary: hsla(0.0, 0.0, 0.91, 1.0),     // #E8E8E8
            border_focus: hsla(211.0, 1.0, 0.50, 1.0),       // #007AFF (blue)

            // Semantic colors
            accent: hsla(211.0, 1.0, 0.50, 1.0),             // #007AFF (Apple blue)
            accent_hover: hsla(211.0, 1.0, 0.45, 1.0),       // Darker blue
            success: hsla(142.0, 0.75, 0.42, 1.0),           // #28CD41 (Apple green)
            warning: hsla(43.0, 1.0, 0.50, 1.0),             // #FF9500 (Apple orange)
            error: hsla(4.0, 1.0, 0.55, 1.0),                // #FF3B30 (Apple red)

            // Message bubbles
            message_user: hsla(211.0, 0.15, 0.96, 1.0),      // Very subtle blue tint
            message_assistant: hsla(0.0, 0.0, 0.97, 1.0),    // Surface secondary
            message_system: hsla(48.0, 0.12, 0.95, 1.0),     // Subtle yellow tint
            message_code_bg: hsla(0.0, 0.0, 0.96, 1.0),      // Slightly darker

            // Sidebar
            sidebar_bg: hsla(0.0, 0.0, 0.98, 1.0),           // #FAFAFA
            sidebar_item_active: hsla(211.0, 0.20, 0.94, 1.0), // Blue tint
            sidebar_item_hover: hsla(0.0, 0.0, 0.96, 0.6),   // Subtle hover
        }
    }
}

/// Border radius tokens
#[derive(Clone, Copy, Debug)]
pub struct BorderRadius {
    pub xs: Pixels,      // 2px - tight corners
    pub sm: Pixels,      // 4px - small elements
    pub md: Pixels,      // 8px - default
    pub lg: Pixels,      // 12px - cards, panels
    pub xl: Pixels,      // 16px - large surfaces
    pub full: Pixels,    // 9999px - fully rounded
}

impl BorderRadius {
    pub fn new() -> Self {
        Self {
            xs: px(2.0),
            sm: px(4.0),
            md: px(8.0),
            lg: px(12.0),
            xl: px(16.0),
            full: px(9999.0),
        }
    }
}

impl Default for BorderRadius {
    fn default() -> Self {
        Self::new()
    }
}

/// Shadow tokens for elevation
#[derive(Clone, Debug)]
pub struct Shadows {
    pub sm: BoxShadow,   // Subtle elevation
    pub md: BoxShadow,   // Default elevation
    pub lg: BoxShadow,   // High elevation
    pub xl: BoxShadow,   // Modal/dialog
}

impl Shadows {
    pub fn dark() -> Self {
        Self {
            sm: BoxShadow {
                offset: point(px(0.0), px(1.0)),
                blur_radius: px(2.0),
                spread_radius: px(0.0),
                color: hsla(0.0, 0.0, 0.0, 0.3),
            },
            md: BoxShadow {
                offset: point(px(0.0), px(4.0)),
                blur_radius: px(8.0),
                spread_radius: px(0.0),
                color: hsla(0.0, 0.0, 0.0, 0.4),
            },
            lg: BoxShadow {
                offset: point(px(0.0), px(8.0)),
                blur_radius: px(16.0),
                spread_radius: px(0.0),
                color: hsla(0.0, 0.0, 0.0, 0.5),
            },
            xl: BoxShadow {
                offset: point(px(0.0), px(16.0)),
                blur_radius: px(32.0),
                spread_radius: px(0.0),
                color: hsla(0.0, 0.0, 0.0, 0.6),
            },
        }
    }

    pub fn light() -> Self {
        Self {
            sm: BoxShadow {
                offset: point(px(0.0), px(1.0)),
                blur_radius: px(2.0),
                spread_radius: px(0.0),
                color: hsla(0.0, 0.0, 0.0, 0.1),
            },
            md: BoxShadow {
                offset: point(px(0.0), px(4.0)),
                blur_radius: px(8.0),
                spread_radius: px(0.0),
                color: hsla(0.0, 0.0, 0.0, 0.15),
            },
            lg: BoxShadow {
                offset: point(px(0.0), px(8.0)),
                blur_radius: px(16.0),
                spread_radius: px(0.0),
                color: hsla(0.0, 0.0, 0.0, 0.2),
            },
            xl: BoxShadow {
                offset: point(px(0.0), px(16.0)),
                blur_radius: px(32.0),
                spread_radius: px(0.0),
                color: hsla(0.0, 0.0, 0.0, 0.25),
            },
        }
    }
}

/// Complete theme combining all design tokens
#[derive(Clone, Debug)]
pub struct ZCloneTheme {
    pub typography: Typography,
    pub spacing: Spacing,
    pub colors: ColorPalette,
    pub radius: BorderRadius,
    pub shadows: Shadows,
}

impl ZCloneTheme {
    pub fn dark() -> Self {
        Self {
            typography: Typography::new(),
            spacing: Spacing::new(),
            colors: ColorPalette::dark(),
            radius: BorderRadius::new(),
            shadows: Shadows::dark(),
        }
    }

    pub fn light() -> Self {
        Self {
            typography: Typography::new(),
            spacing: Spacing::new(),
            colors: ColorPalette::light(),
            radius: BorderRadius::new(),
            shadows: Shadows::light(),
        }
    }
}

// Legacy compatibility - keeping old struct for backward compatibility
#[deprecated(note = "Use ZCloneTheme instead")]
#[derive(Clone, Copy)]
pub struct LegacyTheme {
    pub background: Hsla,
    pub surface: Hsla,
    pub text_primary: Hsla,
    pub text_secondary: Hsla,
    pub accent: Hsla,
    pub border: Hsla,
}
