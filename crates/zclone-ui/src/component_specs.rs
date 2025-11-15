/// Detailed component specifications for Phase 3+
///
/// This module contains comprehensive specifications for all UI components,
/// including layout, styling, behavior, and interaction patterns.

use crate::theme::*;
use gpui::*;

/// Message bubble specifications
pub mod message_bubble {
    use super::*;

    // Message bubble dimensions and spacing
    pub struct Specs {
        /// Horizontal padding inside bubble
        pub padding_x: Pixels,
        /// Vertical padding inside bubble
        pub padding_y: Pixels,
        /// Maximum width of bubble (for readability)
        pub max_width: Pixels,
        /// Gap between avatar and content
        pub avatar_gap: Pixels,
        /// Gap between content and metadata
        pub metadata_gap: Pixels,
    }

    impl Default for Specs {
        fn default() -> Self {
            Self {
                padding_x: px(16.0),
                padding_y: px(12.0),
                max_width: px(720.0),
                avatar_gap: px(12.0),
                metadata_gap: px(8.0),
            }
        }
    }

    // Layout: [Avatar] [Content + Metadata]
    // Content includes: Role label, Message text, Action buttons (retry/edit)
    // Metadata includes: Timestamp, Token count (if applicable)
}

/// Composer specifications
pub mod composer {
    use super::*;

    pub struct Specs {
        /// Minimum height of input area
        pub min_height: Pixels,
        /// Maximum height before scrolling
        pub max_height: Pixels,
        /// Padding around input
        pub padding: Pixels,
        /// Gap between input and send button
        pub button_gap: Pixels,
        /// Send button size
        pub button_size: Pixels,
    }

    impl Default for Specs {
        fn default() -> Self {
            Self {
                min_height: px(48.0),
                max_height: px(200.0),
                padding: px(12.0),
                button_gap: px(8.0),
                button_size: px(36.0),
            }
        }
    }

    // Layout: [Input Area (auto-growing)] [Send Button]
    // Features:
    // - Auto-growing textarea (min 48px, max 200px)
    // - Keyboard shortcut: Cmd+Enter to send
    // - Shift+Enter for new line
    // - Token counter in bottom-right
    // - Attachment button (Phase 4+)
}

/// Sidebar specifications
pub mod sidebar {
    use super::*;

    pub struct Specs {
        /// Sidebar width when expanded
        pub width_expanded: Pixels,
        /// Sidebar width when collapsed
        pub width_collapsed: Pixels,
        /// Header height (with new chat button)
        pub header_height: Pixels,
        /// Search bar height
        pub search_height: Pixels,
        /// Session item height
        pub item_height: Pixels,
        /// Gap between session items
        pub item_gap: Pixels,
        /// Scroll padding at edges
        pub scroll_padding: Pixels,
    }

    impl Default for Specs {
        fn default() -> Self {
            Self {
                width_expanded: px(280.0),
                width_collapsed: px(0.0),
                header_height: px(56.0),
                search_height: px(40.0),
                item_height: px(72.0),
                item_gap: px(4.0),
                scroll_padding: px(8.0),
            }
        }
    }

    // Layout (Vertical):
    // 1. Header: [New Chat Button] [Collapse Button]
    // 2. Search: [Search Icon] [Input] [Clear Button]
    // 3. Session List (Scrollable):
    //    - Each item: [Title] [Preview] [Timestamp] [Context Menu]
    //    - Active item highlighted
    //    - Hover state
    //
    // Context Menu Options:
    // - Rename
    // - Duplicate
    // - Archive
    // - Delete
}

/// Session list item specifications
pub mod session_item {
    use super::*;

    pub struct Specs {
        /// Height of item
        pub height: Pixels,
        /// Horizontal padding
        pub padding_x: Pixels,
        /// Vertical padding
        pub padding_y: Pixels,
        /// Gap between title and preview
        pub title_preview_gap: Pixels,
        /// Maximum preview length (characters)
        pub preview_max_chars: usize,
    }

    impl Default for Specs {
        fn default() -> Self {
            Self {
                height: px(72.0),
                padding_x: px(12.0),
                padding_y: px(10.0),
                title_preview_gap: px(4.0),
                preview_max_chars: 60,
            }
        }
    }

    // Layout (Vertical):
    // [Title - Bold, 14px]
    // [Preview - Regular, 12px, Gray, Truncated]
    // [Timestamp - 11px, Right-aligned]
}

/// Settings panel specifications
pub mod settings_panel {
    use super::*;

    pub struct Specs {
        /// Panel width
        pub width: Pixels,
        /// Content padding
        pub padding: Pixels,
        /// Gap between sections
        pub section_gap: Pixels,
        /// Input field height
        pub input_height: Pixels,
        /// Label spacing
        pub label_spacing: Pixels,
    }

    impl Default for Specs {
        fn default() -> Self {
            Self {
                width: px(400.0),
                padding: px(24.0),
                section_gap: px(24.0),
                input_height: px(40.0),
                label_spacing: px(8.0),
            }
        }
    }

    // Layout (Vertical):
    // 1. Header: [Title] [Close Button]
    // 2. API Key Section:
    //    - Label
    //    - Input (masked) with Show/Hide toggle
    //    - Test Connection button
    //    - Status indicator
    // 3. Model Settings:
    //    - Model selector dropdown
    //    - Temperature slider (0-2)
    //    - System prompt textarea
    // 4. Appearance:
    //    - Theme toggle (Light/Dark/Auto)
    // 5. Actions:
    //    - Save button
    //    - Cancel button
}

/// Chat surface specifications
pub mod chat_surface {
    use super::*;

    pub struct Specs {
        /// Message list padding
        pub message_padding: Pixels,
        /// Gap between messages
        pub message_gap: Pixels,
        /// Composer container height
        pub composer_container_height: Pixels,
        /// Divider height between messages and composer
        pub divider_height: Pixels,
    }

    impl Default for Specs {
        fn default() -> Self {
            Self {
                message_padding: px(16.0),
                message_gap: px(16.0),
                composer_container_height: px(120.0),
                divider_height: px(1.0),
            }
        }
    }

    // Layout (Vertical):
    // 1. Message List (Flex-1, Scrollable):
    //    - Virtualized for performance
    //    - Auto-scroll to bottom on new message
    //    - Scroll to maintain position on resize
    // 2. Divider
    // 3. Composer (Fixed height)
}

/// Toolbar specifications
pub mod toolbar {
    use super::*;

    pub struct Specs {
        /// Toolbar height
        pub height: Pixels,
        /// Horizontal padding
        pub padding_x: Pixels,
        /// Gap between toolbar items
        pub item_gap: Pixels,
        /// Icon button size
        pub button_size: Pixels,
    }

    impl Default for Specs {
        fn default() -> Self {
            Self {
                height: px(48.0),
                padding_x: px(16.0),
                item_gap: px(8.0),
                button_size: px(32.0),
            }
        }
    }

    // Layout (Horizontal):
    // Left: [Sidebar Toggle] [Session Title]
    // Right: [Settings] [Theme Toggle]
}

/// Toast notification specifications
pub mod toast {
    use super::*;

    pub struct Specs {
        /// Toast width
        pub width: Pixels,
        /// Toast height
        pub height: Pixels,
        /// Padding
        pub padding: Pixels,
        /// Gap between icon and text
        pub icon_gap: Pixels,
        /// Duration (milliseconds)
        pub duration_ms: u64,
    }

    impl Default for Specs {
        fn default() -> Self {
            Self {
                width: px(320.0),
                height: px(56.0),
                padding: px(16.0),
                icon_gap: px(12.0),
                duration_ms: 3000,
            }
        }
    }

    // Layout: [Icon] [Message] [Close]
    // Types: Success, Error, Warning, Info
    // Position: Bottom-center, stacked vertically
    // Animation: Slide up + fade in, fade out after duration
}

/// Code block specifications
pub mod code_block {
    use super::*;

    pub struct Specs {
        /// Padding inside code block
        pub padding: Pixels,
        /// Border radius
        pub radius: Pixels,
        /// Max height before scroll
        pub max_height: Pixels,
        /// Header height (language + copy button)
        pub header_height: Pixels,
        /// Line number width
        pub line_number_width: Pixels,
    }

    impl Default for Specs {
        fn default() -> Self {
            Self {
                padding: px(16.0),
                radius: px(8.0),
                max_height: px(600.0),
                header_height: px(36.0),
                line_number_width: px(48.0),
            }
        }
    }

    // Layout:
    // 1. Header: [Language Label] [Copy Button]
    // 2. Code Content:
    //    - Line numbers (optional)
    //    - Syntax highlighted code
    //    - Horizontal scroll if needed
    //
    // Features:
    // - Copy to clipboard
    // - Syntax highlighting (Phase 5)
    // - Line numbers toggle
}

/// Modal/Dialog specifications
pub mod modal {
    use super::*;

    pub struct Specs {
        /// Backdrop opacity
        pub backdrop_opacity: f32,
        /// Modal max width
        pub max_width: Pixels,
        /// Modal padding
        pub padding: Pixels,
        /// Header height
        pub header_height: Pixels,
        /// Footer height
        pub footer_height: Pixels,
    }

    impl Default for Specs {
        fn default() -> Self {
            Self {
                backdrop_opacity: 0.6,
                max_width: px(600.0),
                padding: px(24.0),
                header_height: px(56.0),
                footer_height: px(64.0),
            }
        }
    }

    // Layout:
    // 1. Backdrop (Full screen, semi-transparent)
    // 2. Modal Container (Centered):
    //    - Header: [Title] [Close Button]
    //    - Content (Scrollable)
    //    - Footer: [Cancel] [Primary Action]
    //
    // Animation:
    // - Fade in backdrop
    // - Scale up + fade in modal
    // - ESC to close
}

/// Button specifications
pub mod button {
    use super::*;

    #[derive(Debug, Clone, Copy)]
    pub enum Size {
        Small,
        Medium,
        Large,
    }

    #[derive(Debug, Clone, Copy)]
    pub enum Variant {
        Primary,    // Filled with accent color
        Secondary,  // Outlined
        Ghost,      // No background/border
        Danger,     // Red/destructive
    }

    pub struct Specs {
        pub size: Size,
        pub variant: Variant,
    }

    impl Specs {
        pub fn height(&self) -> Pixels {
            match self.size {
                Size::Small => px(28.0),
                Size::Medium => px(36.0),
                Size::Large => px(44.0),
            }
        }

        pub fn padding_x(&self) -> Pixels {
            match self.size {
                Size::Small => px(12.0),
                Size::Medium => px(16.0),
                Size::Large => px(20.0),
            }
        }

        pub fn font_size(&self) -> Pixels {
            match self.size {
                Size::Small => px(12.0),
                Size::Medium => px(14.0),
                Size::Large => px(16.0),
            }
        }
    }

    // States: Default, Hover, Active, Disabled, Loading
    // Features:
    // - Icon support (left/right)
    // - Loading spinner
    // - Keyboard navigation (Tab, Enter)
}

/// Input field specifications
pub mod input {
    use super::*;

    pub struct Specs {
        /// Input height
        pub height: Pixels,
        /// Padding
        pub padding_x: Pixels,
        /// Border width
        pub border_width: Pixels,
        /// Icon size (if present)
        pub icon_size: Pixels,
    }

    impl Default for Specs {
        fn default() -> Self {
            Self {
                height: px(40.0),
                padding_x: px(12.0),
                border_width: px(1.0),
                icon_size: px(20.0),
            }
        }
    }

    // States: Default, Focused, Error, Disabled
    // Features:
    // - Placeholder text
    // - Clear button (when has value)
    // - Icon support (left/right)
    // - Validation feedback
}
