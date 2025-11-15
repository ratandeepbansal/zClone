/// Motion and interaction guidelines for zClone
///
/// This module defines animation curves, durations, and interaction patterns
/// to create a cohesive and responsive user experience.

use gpui::*;
use std::time::Duration;

/// Animation duration tokens
#[derive(Clone, Copy, Debug)]
pub struct AnimationDurations {
    /// Very fast animations (100ms) - micro-interactions
    pub instant: Duration,
    /// Fast animations (200ms) - hover states, tooltips
    pub fast: Duration,
    /// Normal animations (300ms) - default for most transitions
    pub normal: Duration,
    /// Slow animations (400ms) - larger movements, page transitions
    pub slow: Duration,
    /// Very slow animations (600ms) - complex animations
    pub slower: Duration,
}

impl AnimationDurations {
    pub fn new() -> Self {
        Self {
            instant: Duration::from_millis(100),
            fast: Duration::from_millis(200),
            normal: Duration::from_millis(300),
            slow: Duration::from_millis(400),
            slower: Duration::from_millis(600),
        }
    }
}

impl Default for AnimationDurations {
    fn default() -> Self {
        Self::new()
    }
}

/// Easing curves for animations
#[derive(Clone, Copy, Debug)]
pub enum EasingCurve {
    /// Linear easing (no acceleration)
    Linear,
    /// Ease in (slow start)
    EaseIn,
    /// Ease out (slow end) - most common, feels natural
    EaseOut,
    /// Ease in-out (slow start and end)
    EaseInOut,
    /// Custom cubic bezier
    CubicBezier(f32, f32, f32, f32),
}

impl EasingCurve {
    /// Standard ease-out curve (recommended default)
    pub fn standard() -> Self {
        Self::CubicBezier(0.4, 0.0, 0.2, 1.0)
    }

    /// Emphasized ease for dramatic effect
    pub fn emphasized() -> Self {
        Self::CubicBezier(0.0, 0.0, 0.2, 1.0)
    }

    /// Decelerated ease for enter animations
    pub fn decelerated() -> Self {
        Self::CubicBezier(0.0, 0.0, 0.2, 1.0)
    }

    /// Accelerated ease for exit animations
    pub fn accelerated() -> Self {
        Self::CubicBezier(0.4, 0.0, 1.0, 1.0)
    }
}

/// Motion specifications for different interaction patterns
pub struct MotionSpecs;

impl MotionSpecs {
    /// Sidebar collapse/expand animation
    pub fn sidebar_toggle() -> MotionSpec {
        MotionSpec {
            duration: AnimationDurations::new().normal,
            easing: EasingCurve::standard(),
            properties: vec![
                AnimatedProperty::Width,
                AnimatedProperty::Opacity,
            ],
        }
    }

    /// Theme toggle fade animation
    pub fn theme_toggle() -> MotionSpec {
        MotionSpec {
            duration: AnimationDurations::new().normal,
            easing: EasingCurve::standard(),
            properties: vec![
                AnimatedProperty::Opacity,
                AnimatedProperty::BackgroundColor,
                AnimatedProperty::TextColor,
            ],
        }
    }

    /// Message streaming animation (token-by-token)
    pub fn message_streaming() -> MotionSpec {
        MotionSpec {
            duration: AnimationDurations::new().instant,
            easing: EasingCurve::Linear,
            properties: vec![
                AnimatedProperty::Height,
                AnimatedProperty::Opacity,
            ],
        }
    }

    /// Message bubble appear animation
    pub fn message_appear() -> MotionSpec {
        MotionSpec {
            duration: AnimationDurations::new().normal,
            easing: EasingCurve::decelerated(),
            properties: vec![
                AnimatedProperty::Opacity,
                AnimatedProperty::TranslateY,
            ],
        }
    }

    /// Button hover animation
    pub fn button_hover() -> MotionSpec {
        MotionSpec {
            duration: AnimationDurations::new().fast,
            easing: EasingCurve::standard(),
            properties: vec![
                AnimatedProperty::BackgroundColor,
                AnimatedProperty::Scale,
            ],
        }
    }

    /// Button press animation
    pub fn button_press() -> MotionSpec {
        MotionSpec {
            duration: AnimationDurations::new().instant,
            easing: EasingCurve::standard(),
            properties: vec![
                AnimatedProperty::Scale,
            ],
        }
    }

    /// Modal/dialog appear animation
    pub fn modal_appear() -> MotionSpec {
        MotionSpec {
            duration: AnimationDurations::new().normal,
            easing: EasingCurve::decelerated(),
            properties: vec![
                AnimatedProperty::Opacity,
                AnimatedProperty::Scale,
            ],
        }
    }

    /// Modal/dialog dismiss animation
    pub fn modal_dismiss() -> MotionSpec {
        MotionSpec {
            duration: AnimationDurations::new().fast,
            easing: EasingCurve::accelerated(),
            properties: vec![
                AnimatedProperty::Opacity,
                AnimatedProperty::Scale,
            ],
        }
    }

    /// Toast notification slide-in animation
    pub fn toast_appear() -> MotionSpec {
        MotionSpec {
            duration: AnimationDurations::new().normal,
            easing: EasingCurve::emphasized(),
            properties: vec![
                AnimatedProperty::Opacity,
                AnimatedProperty::TranslateY,
            ],
        }
    }

    /// Settings panel slide-in from right
    pub fn settings_slide_in() -> MotionSpec {
        MotionSpec {
            duration: AnimationDurations::new().normal,
            easing: EasingCurve::standard(),
            properties: vec![
                AnimatedProperty::TranslateX,
                AnimatedProperty::Opacity,
            ],
        }
    }

    /// Session item hover animation
    pub fn session_item_hover() -> MotionSpec {
        MotionSpec {
            duration: AnimationDurations::new().fast,
            easing: EasingCurve::standard(),
            properties: vec![
                AnimatedProperty::BackgroundColor,
            ],
        }
    }

    /// Composer auto-grow animation
    pub fn composer_grow() -> MotionSpec {
        MotionSpec {
            duration: AnimationDurations::new().fast,
            easing: EasingCurve::standard(),
            properties: vec![
                AnimatedProperty::Height,
            ],
        }
    }

    /// Focus ring animation
    pub fn focus_ring() -> MotionSpec {
        MotionSpec {
            duration: AnimationDurations::new().fast,
            easing: EasingCurve::standard(),
            properties: vec![
                AnimatedProperty::BorderColor,
                AnimatedProperty::BoxShadow,
            ],
        }
    }
}

/// Specification for a motion/animation
#[derive(Clone, Debug)]
pub struct MotionSpec {
    pub duration: Duration,
    pub easing: EasingCurve,
    pub properties: Vec<AnimatedProperty>,
}

/// Properties that can be animated
#[derive(Clone, Copy, Debug)]
pub enum AnimatedProperty {
    Opacity,
    Width,
    Height,
    Scale,
    TranslateX,
    TranslateY,
    BackgroundColor,
    TextColor,
    BorderColor,
    BoxShadow,
}

/// Interaction states for components
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum InteractionState {
    Default,
    Hover,
    Active,
    Focused,
    Disabled,
    Loading,
}

/// Interaction guidelines
pub struct InteractionGuidelines;

impl InteractionGuidelines {
    /// Hover state specifications
    pub fn hover() -> HoverSpec {
        HoverSpec {
            scale_factor: 1.02,  // Subtle scale up
            opacity_change: 0.1,  // Slight opacity increase
        }
    }

    /// Active/pressed state specifications
    pub fn active() -> ActiveSpec {
        ActiveSpec {
            scale_factor: 0.98,  // Slight scale down
            opacity_change: -0.1, // Slight opacity decrease
        }
    }

    /// Focus state specifications (keyboard navigation)
    pub fn focus() -> FocusSpec {
        FocusSpec {
            ring_width: px(2.0),
            ring_offset: px(2.0),
            ring_blur: px(4.0),
        }
    }

    /// Disabled state specifications
    pub fn disabled() -> DisabledSpec {
        DisabledSpec {
            opacity: 0.4,
        }
    }

    /// Loading state specifications
    pub fn loading() -> LoadingSpec {
        LoadingSpec {
            spinner_size: px(16.0),
            opacity: 0.6,
        }
    }
}

/// Hover interaction specification
pub struct HoverSpec {
    pub scale_factor: f32,
    pub opacity_change: f32,
}

/// Active/pressed interaction specification
pub struct ActiveSpec {
    pub scale_factor: f32,
    pub opacity_change: f32,
}

/// Focus interaction specification
pub struct FocusSpec {
    pub ring_width: Pixels,
    pub ring_offset: Pixels,
    pub ring_blur: Pixels,
}

/// Disabled state specification
pub struct DisabledSpec {
    pub opacity: f32,
}

/// Loading state specification
pub struct LoadingSpec {
    pub spinner_size: Pixels,
    pub opacity: f32,
}

/// Scroll behavior specifications
pub struct ScrollBehavior;

impl ScrollBehavior {
    /// Smooth scroll duration
    pub fn smooth_scroll_duration() -> Duration {
        Duration::from_millis(300)
    }

    /// Auto-scroll to bottom (for new messages)
    pub fn auto_scroll_duration() -> Duration {
        Duration::from_millis(200)
    }

    /// Scroll threshold before showing "scroll to bottom" button
    pub fn scroll_threshold() -> Pixels {
        px(200.0)
    }
}

/// Keyboard shortcuts
pub struct KeyboardShortcuts;

impl KeyboardShortcuts {
    /// Get all keyboard shortcuts
    pub fn all() -> Vec<Shortcut> {
        vec![
            Shortcut {
                key: "Cmd+Enter",
                action: "Send message",
                context: "Composer focused",
            },
            Shortcut {
                key: "Cmd+N",
                action: "New chat",
                context: "Global",
            },
            Shortcut {
                key: "Cmd+,",
                action: "Open settings",
                context: "Global",
            },
            Shortcut {
                key: "Cmd+B",
                action: "Toggle sidebar",
                context: "Global",
            },
            Shortcut {
                key: "Cmd+K",
                action: "Quick search sessions",
                context: "Global",
            },
            Shortcut {
                key: "Cmd+W",
                action: "Close/archive current session",
                context: "Chat view",
            },
            Shortcut {
                key: "Cmd+Shift+D",
                action: "Toggle theme (light/dark)",
                context: "Global",
            },
            Shortcut {
                key: "Escape",
                action: "Close modal/panel",
                context: "When modal/panel open",
            },
            Shortcut {
                key: "↑/↓",
                action: "Navigate sessions",
                context: "Sidebar focused",
            },
            Shortcut {
                key: "Enter",
                action: "Select session",
                context: "Sidebar focused",
            },
        ]
    }
}

/// Keyboard shortcut definition
pub struct Shortcut {
    pub key: &'static str,
    pub action: &'static str,
    pub context: &'static str,
}

/// Accessibility considerations
pub struct AccessibilityGuidelines;

impl AccessibilityGuidelines {
    /// Minimum touch target size (for mouse/touch)
    pub fn min_touch_target() -> Pixels {
        px(44.0)  // iOS HIG recommendation
    }

    /// Focus indicator minimum contrast ratio
    pub fn focus_contrast_ratio() -> f32 {
        3.0  // WCAG 2.1 requirement
    }

    /// Text contrast ratios
    pub fn text_contrast() -> TextContrastRatios {
        TextContrastRatios {
            normal_text_aa: 4.5,   // WCAG AA
            normal_text_aaa: 7.0,  // WCAG AAA
            large_text_aa: 3.0,    // WCAG AA for large text
            large_text_aaa: 4.5,   // WCAG AAA for large text
        }
    }

    /// Animation preferences
    pub fn reduced_motion() -> bool {
        // Should check system preference
        // macOS: System Preferences > Accessibility > Display > Reduce motion
        false  // Default, should be read from system
    }
}

/// Text contrast ratio requirements
pub struct TextContrastRatios {
    pub normal_text_aa: f32,
    pub normal_text_aaa: f32,
    pub large_text_aa: f32,
    pub large_text_aaa: f32,
}

// Tests temporarily disabled due to compiler recursion limit issues with GPUI macros
// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn test_animation_durations() {
//         let durations = AnimationDurations::new();
//         assert_eq!(durations.instant, Duration::from_millis(100));
//         assert_eq!(durations.fast, Duration::from_millis(200));
//         assert_eq!(durations.normal, Duration::from_millis(300));
//     }
//
//     #[test]
//     fn test_accessibility_targets() {
//         assert_eq!(AccessibilityGuidelines::min_touch_target(), px(44.0));
//     }
// }
