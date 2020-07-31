use seed_style::*;
// Theme Definition
// -----------------
//
// A Theme Object is where all css related values and presets can be stored
// and then accessed at any point in the view.
//
// The Theme Object is broadly consistent with the Theme specification that is used in
// several css librarys: https://theme-ui.com/theme-spec/
//
// A Theme object is made up of named css values called aliases
// as well as scales for css values.
//
// Having a scale is useful for things like sizes and spacing
// because you can have consistent layout throughout your app.  For instance pixel gaps
// at 4px increments.
//
// Having named aliases for things like colors is useful because it means
// swapping out colors, or having a dark/light theme can be defined in a central location.
//
// In order to use cssvalue aliases we use an enum.
//
// // Main Color Theme Keys
#[derive(Hash, PartialEq, Eq, Clone)]
pub enum Color {
    Background,
    MainText,
    Primary,
    MutedPrimary,
    DarkPrimary,
    MutedSecondary,
    Secondary,
    DarkSecondary,
    Highlight,
}
impl ColorTheme for Color {} // Allows you to use a `Color` variant as a CssColor alias in the theme.

#[derive(Hash, PartialEq, Eq, Clone)]
pub enum MyStyles {
    HeaderText,
    BodyText,
}

impl StyleTheme for MyStyles {} // Allows you to use a `MyStyles` variant as a Style alias in the theme.

pub fn user_theme() -> Theme {
    Theme::new("basic_theme")
        // Various theme colors
        .set_color(Color::Background, CssColor::Hex(0xFFFFFF))
        .set_color(Color::MainText, CssColor::Hex(0x000000))
        .set_color(Color::Primary, CssColor::Hsl(200.0, 60.0, 40.0))
        .set_color(Color::MutedPrimary, CssColor::Hsl(200.0, 30.0, 80.0))
        .set_color(Color::Secondary, hsl(270, 80, 60))
        .set_color(Color::MutedSecondary, hsl(270, 70, 80)) // or use the hsl shortcut
        .set_color(Color::Highlight, hsl(310, 70, 85))
        .set_color(Color::DarkPrimary, hsl(200, 70, 35))
        .set_color(Color::DarkSecondary, hsl(300, 60, 20))
        .set_style(
            MyStyles::HeaderText,
            s().font_style_italic()
                .text_decoration_underline()
                .font_weight_v900()
                .text_align_center(),
        )
        .set_style(
            MyStyles::BodyText,
            s().line_height(CssLineHeight::Number(1.3))
                .letter_spacing(rem(0.012))
                .font_weight_v300(),
        )
}

// Not there are many other themeable properties including:

//  BorderTheme,
//  BorderWidthTheme,
//  BorderStyleTheme,
//  SpaceTheme,
//  LineHeightTheme,
//  LetterSpacingTheme,
//  BorderRadiusTheme,
//  FontTheme,
//  FontSizeTheme,
//  SizeTheme,
//  TransitionTheme,
//  ZIndexTheme,
//  DisplayTheme,
//  ColorTheme,
//  ShadowTheme,
//  StyleTheme,
//  BreakpointTheme,
