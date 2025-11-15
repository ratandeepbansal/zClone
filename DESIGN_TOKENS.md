# zClone Design Tokens Reference

Quick reference for all design tokens in the zClone design system.

## Typography

### Font Families
- **Display**: SF Pro Display
- **Text**: SF Pro Text
- **Mono**: SF Mono

### Type Scale

| Token | Size | Line Height | Weight | Usage |
|-------|------|-------------|--------|-------|
| `display_large` | 28px | 34px | Bold | Page titles |
| `display_medium` | 22px | 28px | SemiBold | Section headers |
| `display_small` | 18px | 24px | SemiBold | Subsection headers |
| `body_large` | 16px | 24px | Regular | Emphasized body text |
| `body_medium` | 14px | 20px | Regular | Default body text |
| `body_small` | 12px | 18px | Regular | Captions, metadata |
| `code` | 14px | 20px | Regular | Code blocks |
| `code_inline` | 13px | 18px | Regular | Inline code |

## Colors

### Dark Mode

#### Surfaces
| Token | Value | Hex | Usage |
|-------|-------|-----|-------|
| `surface_primary` | hsla(0, 0%, 11%, 1) | #1C1C1C | Main background |
| `surface_secondary` | hsla(0, 0%, 14%, 1) | #242424 | Cards, panels |
| `surface_tertiary` | hsla(0, 0%, 18%, 1) | #2E2E2E | Elevated surfaces |
| `surface_active` | hsla(0, 0%, 22%, 1) | #383838 | Selected state |
| `surface_hover` | hsla(0, 0%, 16%, 0.8) | #292929 | Hover overlay |

#### Text (WCAG AAA: 7:1 contrast)
| Token | Value | Hex | Usage |
|-------|-------|-----|-------|
| `text_primary` | hsla(0, 0%, 96%, 1) | #F5F5F5 | Primary text |
| `text_secondary` | hsla(0, 0%, 72%, 1) | #B8B8B8 | Secondary text |
| `text_tertiary` | hsla(0, 0%, 56%, 1) | #8F8F8F | Tertiary text |
| `text_inverse` | hsla(0, 0%, 8%, 1) | #141414 | Text on accent |
| `text_disabled` | hsla(0, 0%, 40%, 1) | #666666 | Disabled state |

#### Borders
| Token | Value | Hex | Usage |
|-------|-------|-----|-------|
| `border_primary` | hsla(0, 0%, 24%, 1) | #3D3D3D | Default borders |
| `border_secondary` | hsla(0, 0%, 18%, 1) | #2E2E2E | Subtle borders |
| `border_focus` | hsla(211, 100%, 55%, 1) | #0A84FF | Focus rings |

#### Semantic Colors
| Token | Value | Hex | Usage |
|-------|-------|-----|-------|
| `accent` | hsla(211, 100%, 55%, 1) | #0A84FF | Primary actions |
| `accent_hover` | hsla(211, 100%, 65%, 1) | - | Accent hover |
| `success` | hsla(142, 70%, 45%, 1) | #34C759 | Success states |
| `warning` | hsla(48, 100%, 55%, 1) | #FFD60A | Warnings |
| `error` | hsla(4, 90%, 58%, 1) | #FF453A | Errors |

#### Message Bubbles
| Token | Value | Usage |
|-------|-------|-------|
| `message_user` | hsla(211, 20%, 20%, 1) | User messages |
| `message_assistant` | hsla(0, 0%, 14%, 1) | Assistant messages |
| `message_system` | hsla(48, 15%, 18%, 1) | System messages |
| `message_code_bg` | hsla(0, 0%, 10%, 1) | Code blocks |

#### Sidebar
| Token | Value | Usage |
|-------|-------|-------|
| `sidebar_bg` | hsla(0, 0%, 9%, 1) | Sidebar background |
| `sidebar_item_active` | hsla(211, 25%, 18%, 1) | Active session |
| `sidebar_item_hover` | hsla(0, 0%, 14%, 0.6) | Hover state |

### Light Mode

#### Surfaces
| Token | Value | Hex | Usage |
|-------|-------|-----|-------|
| `surface_primary` | hsla(0, 0%, 100%, 1) | #FFFFFF | Main background |
| `surface_secondary` | hsla(0, 0%, 97%, 1) | #F7F7F7 | Cards, panels |
| `surface_tertiary` | hsla(0, 0%, 94%, 1) | #F0F0F0 | Elevated surfaces |
| `surface_active` | hsla(0, 0%, 91%, 1) | #E8E8E8 | Selected state |
| `surface_hover` | hsla(0, 0%, 95%, 0.8) | #F2F2F2 | Hover overlay |

#### Text (WCAG AAA: 7:1 contrast)
| Token | Value | Hex | Usage |
|-------|-------|-----|-------|
| `text_primary` | hsla(0, 0%, 12%, 1) | #1E1E1E | Primary text |
| `text_secondary` | hsla(0, 0%, 38%, 1) | #616161 | Secondary text |
| `text_tertiary` | hsla(0, 0%, 52%, 1) | #858585 | Tertiary text |
| `text_inverse` | hsla(0, 0%, 98%, 1) | #FAFAFA | Text on accent |
| `text_disabled` | hsla(0, 0%, 68%, 1) | #ADADAD | Disabled state |

#### Semantic Colors
| Token | Value | Hex | Usage |
|-------|-------|-----|-------|
| `accent` | hsla(211, 100%, 50%, 1) | #007AFF | Primary actions |
| `accent_hover` | hsla(211, 100%, 45%, 1) | - | Accent hover |
| `success` | hsla(142, 75%, 42%, 1) | #28CD41 | Success states |
| `warning` | hsla(43, 100%, 50%, 1) | #FF9500 | Warnings |
| `error` | hsla(4, 100%, 55%, 1) | #FF3B30 | Errors |

## Spacing

8px-based scale:

| Token | Value | Usage |
|-------|-------|-------|
| `xs` | 4px | Tight spacing |
| `sm` | 8px | Compact spacing |
| `md` | 16px | Default spacing |
| `lg` | 24px | Comfortable spacing |
| `xl` | 32px | Loose spacing |
| `xxl` | 48px | Section spacing |
| `xxxl` | 64px | Page spacing |

## Border Radius

| Token | Value | Usage |
|-------|-------|-------|
| `xs` | 2px | Tight corners |
| `sm` | 4px | Small elements |
| `md` | 8px | Default |
| `lg` | 12px | Cards, panels |
| `xl` | 16px | Large surfaces |
| `full` | 9999px | Pills, fully rounded |

## Shadows

### Dark Mode
| Token | Offset | Blur | Spread | Color | Usage |
|-------|--------|------|--------|-------|-------|
| `sm` | 0, 1px | 2px | 0 | rgba(0,0,0,0.3) | Subtle |
| `md` | 0, 4px | 8px | 0 | rgba(0,0,0,0.4) | Default |
| `lg` | 0, 8px | 16px | 0 | rgba(0,0,0,0.5) | High |
| `xl` | 0, 16px | 32px | 0 | rgba(0,0,0,0.6) | Modal |

### Light Mode
| Token | Offset | Blur | Spread | Color | Usage |
|-------|--------|------|--------|-------|-------|
| `sm` | 0, 1px | 2px | 0 | rgba(0,0,0,0.1) | Subtle |
| `md` | 0, 4px | 8px | 0 | rgba(0,0,0,0.15) | Default |
| `lg` | 0, 8px | 16px | 0 | rgba(0,0,0,0.2) | High |
| `xl` | 0, 16px | 32px | 0 | rgba(0,0,0,0.25) | Modal |

## Animation Durations

| Token | Value | Usage |
|-------|-------|-------|
| `instant` | 100ms | Micro-interactions |
| `fast` | 200ms | Hover, tooltips |
| `normal` | 300ms | Default transitions |
| `slow` | 400ms | Larger movements |
| `slower` | 600ms | Complex animations |

## Easing Curves

| Curve | Bezier | Usage |
|-------|--------|-------|
| `standard` | (0.4, 0.0, 0.2, 1.0) | Default |
| `emphasized` | (0.0, 0.0, 0.2, 1.0) | Dramatic |
| `decelerated` | (0.0, 0.0, 0.2, 1.0) | Enter |
| `accelerated` | (0.4, 0.0, 1.0, 1.0) | Exit |

## Component Dimensions

### Sidebar
- Width (expanded): 280px
- Width (collapsed): 0px
- Header height: 56px
- Search height: 40px
- Item height: 72px

### Message Bubble
- Padding: 16px × 12px
- Max width: 720px
- Avatar gap: 12px

### Composer
- Min height: 48px
- Max height: 200px
- Padding: 12px
- Button size: 36px

### Buttons
- Small: 28px
- Medium: 36px
- Large: 44px

### Settings Panel
- Width: 400px
- Padding: 24px
- Section gap: 24px

## Keyboard Shortcuts

| Key | Action | Context |
|-----|--------|---------|
| `Cmd+Enter` | Send message | Composer |
| `Cmd+N` | New chat | Global |
| `Cmd+,` | Settings | Global |
| `Cmd+B` | Toggle sidebar | Global |
| `Cmd+K` | Search sessions | Global |
| `Cmd+W` | Close session | Chat |
| `Cmd+Shift+D` | Toggle theme | Global |
| `Escape` | Close modal | Modal |
| `↑/↓` | Navigate | Sidebar |
| `Enter` | Select | Sidebar |

## Accessibility

### Touch Targets
- Minimum: 44px × 44px

### Contrast Ratios
- Normal text (AA): 4.5:1
- Normal text (AAA): 7:1
- Large text (AA): 3:0
- Large text (AAA): 4.5:1
- Focus indicator: 3:1

### Focus Indicators
- Ring width: 2px
- Ring offset: 2px
- Ring blur: 4px

## Code Usage

### Rust
```rust
use zclone_ui::*;

// Get theme
let theme = ZCloneTheme::dark();

// Typography
let style = theme.typography.body_medium;

// Colors
let bg = theme.colors.surface_primary;
let text = theme.colors.text_primary;

// Spacing
let padding = theme.spacing.md;

// Radius
let corners = theme.radius.md;
```

### Component Example
```rust
div()
  .bg(theme.colors.surface_secondary)
  .p(theme.spacing.md)
  .rounded(theme.radius.lg)
  .text_color(theme.colors.text_primary)
  .child("Content")
```

---

**Note**: All color values use HSLA format for better programmatic manipulation. Hex values provided for reference only.
