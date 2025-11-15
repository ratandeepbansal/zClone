**Situation**
You are developing zClone, a ChatGPT clone built on the GPUI framework (from Zed Industries) to ensure a smooth, fast desktop experience. The application needs to support multiple chat windows, persistent chat history in a collapsible sidebar, and user-configurable OpenAI API integration.

**Task**
Design and specify the complete UI/UX architecture for zClone, including:
- Chat window layout with message display and input area
- Collapsible side panel for chat history management
- Settings section for OpenAI API key configuration
- Dark and light mode implementations
- Font and visual styling specifications

**Objective**
Create an inspiring, modern, and performant interface that feels native to macOS while maintaining the speed and smoothness advantages of the GPUI framework. The design should prioritize visual elegance and user experience across both color modes.

**Knowledge**
- The application is built on GPUI (Zed's UI framework), which prioritizes performance and smooth rendering
- Target platform: macOS (with consideration for modern system fonts like SF Pro Display)
- Core features: multiple chat windows, collapsible chat history sidebar, settings panel with API key input
- The user must provide their own OpenAI API key to use the application
- The interface should support seamless switching between dark and light modes
- Visual aesthetic should be sleek, modern, and inspiring

**Behavioral Rules**
1. The assistant should specify typography choices that are modern, legible, and optimized for macOS (prioritize system fonts like SF Pro Display, SF Mono)
2. The assistant should design a collapsible sidebar that preserves screen real estate while keeping chat history easily accessible
3. The assistant should ensure the settings section clearly indicates where and how users input their OpenAI API key, with appropriate security considerations (masked input)
4. The assistant should define color palettes and contrast ratios for both dark and light modes that maintain visual hierarchy and readability
5. The assistant should prioritize layout decisions that leverage GPUI's performance capabilities, avoiding unnecessary complexity that could impact smoothness