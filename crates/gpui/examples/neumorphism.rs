use gpui::{
    div, point, px, rgb, size, App, AppContext, Bounds, BoxShadow, Div, IntoElement,
    ParentElement as _, Render, Styled as _, ViewContext, VisualContext as _, WindowBounds,
    WindowOptions,
};
use smallvec::{smallvec, SmallVec};

/// RGB surface colors.
///
/// Using hex for human comprehension of colours.
/// Derived colors are computed by uniformly darkening BG per channel.
mod colors {
    /// Subtract `amount` from each RGB channel of a hex color, clamping to 0.
    const fn darken(color: u32, amount: u32) -> u32 {
        let r = ((color >> 16) & 0xFF).saturating_sub(amount);
        let g = ((color >> 8) & 0xFF).saturating_sub(amount);
        let b = (color & 0xFF).saturating_sub(amount);
        (r << 16) | (g << 8) | b
    }

    /// Light gray background.
    ///
    /// Neumorphism requires a muted, non-white background
    /// so that both the light highlight and dark shadow are visible against it.
    pub const BG: u32 = 0xe0e5ec;

    /// Dark gray for readable text contrast against BG.
    pub const TEXT: u32 = 0x4a5568;

    /// Slightly darker than BG, used to fake a pressed/concave surface.
    pub const BG_PRESSED: u32 = darken(BG, 15);

    /// Background for the inset placeholder card.
    pub const BG_INSET: u32 = darken(BG, 8);

    /// Border color for the inset placeholder card.
    pub const BORDER_INSET: u32 = darken(BG, 24);
}

/// HSLA shadow colors with alpha for transparency.
///
/// Using HSLA for human comprehension of grayscale with varying alpha.
#[rustfmt::skip]
mod shadows {
    use gpui::Hsla;

    /// White highlight cast from the top-left light source.
    pub const HIGHLIGHT: Hsla = Hsla { h: 0., s: 0., l: 1., a: 0.7 };

    /// Dark shadow cast from the bottom-right, opposite the light source.
    pub const SHADOW: Hsla = Hsla { h: 0., s: 0., l: 0., a: 0.15 };

    /// Softer dark shadow for the fake pressed state.
    pub const SHADOW_PRESSED: Hsla = Hsla { h: 0., s: 0., l: 0., a: 0.1 };

    /// Softer highlight for the fake pressed state.
    pub const HIGHLIGHT_PRESSED: Hsla = Hsla { h: 0., s: 0., l: 1., a: 0.5 };
}

/// Layout dimensions in pixels.
mod layout {
    pub const CARD_SIZE: f32 = 160.;
    pub const CARD_RADIUS: f32 = 24.;
    pub const CIRCLE_SIZE: f32 = 80.;
    pub const WINDOW_WIDTH: f32 = 700.;
    pub const WINDOW_HEIGHT: f32 = 550.;
}

/// Build the pair of box shadows that create a raised neumorphic surface.
///
/// `distance` controls how far the shadows are offset from the element.
/// `blur` controls the softness of the shadow edges.
fn neumorphic_shadow(distance: f32, blur: f32) -> SmallVec<[BoxShadow; 2]> {
    smallvec![
        BoxShadow {
            color: shadows::HIGHLIGHT,
            offset: point(px(-distance), px(-distance)),
            blur_radius: px(blur),
            spread_radius: px(0.),
        },
        BoxShadow {
            color: shadows::SHADOW,
            offset: point(px(distance), px(distance)),
            blur_radius: px(blur),
            spread_radius: px(0.),
        },
    ]
}

/// A basic card element with the shared neumorphic background and sizing.
fn card(label: &str) -> Div {
    div()
        .flex()
        .flex_col()
        .items_center()
        .justify_center()
        .gap_2()
        .w(px(layout::CARD_SIZE))
        .h(px(layout::CARD_SIZE))
        .rounded(px(layout::CARD_RADIUS))
        .bg(rgb(colors::BG))
        .text_color(rgb(colors::TEXT))
        .child(label.to_string())
}

struct Neumorphism;

impl Render for Neumorphism {
    fn render(&mut self, _cx: &mut ViewContext<Self>) -> impl IntoElement {
        // NOTE: Ignoring gpui patterns for
        div()
            .flex()
            .flex_col()
            .size_full()
            .bg(rgb(colors::BG))
            .items_center()
            .justify_center()
            .gap_8()
            .child(
                div()
                    .text_xl()
                    .text_color(rgb(colors::TEXT))
                    .child("Neumorphic Shadows in GPUI"),
            )
            // Row of cards
            .child(
                div()
                    .flex()
                    .flex_row()
                    .gap_8()
                    // Raised (standard neumorphic)
                    .child(card("Raised").shadow(neumorphic_shadow(6., 12.)))
                    // Raised with more depth
                    .child(card("Deep").shadow(neumorphic_shadow(10., 20.)))
                    // Subtle
                    .child(card("Subtle").shadow(neumorphic_shadow(3., 6.))),
            )
            // Second row -- showing the limitation
            .child(
                div()
                    .flex()
                    .flex_row()
                    .gap_8()
                    // Faked "pressed" via darker bg + inverted shadow
                    // This is a workaround -- real inset shadows would be better
                    .child(
                        card("Pressed (fake)")
                            .bg(rgb(colors::BG_PRESSED))
                            .shadow(smallvec![
                                BoxShadow {
                                    color: shadows::SHADOW_PRESSED,
                                    offset: point(px(-2.), px(-2.)),
                                    blur_radius: px(4.),
                                    spread_radius: px(0.),
                                },
                                BoxShadow {
                                    color: shadows::HIGHLIGHT_PRESSED,
                                    offset: point(px(2.), px(2.)),
                                    blur_radius: px(4.),
                                    spread_radius: px(0.),
                                },
                            ]),
                    )
                    // Circular raised element
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .justify_center()
                            .w(px(layout::CIRCLE_SIZE))
                            .h(px(layout::CIRCLE_SIZE))
                            .rounded(px(layout::CIRCLE_SIZE / 2.))
                            .bg(rgb(colors::BG))
                            .text_color(rgb(colors::TEXT))
                            .shadow(neumorphic_shadow(6., 12.))
                            .child("Icon"),
                    )
                    // Placeholder for the inset shadow card (TODO)
                    .child(
                        card("Inset (TODO)")
                            .border_1()
                            .border_color(rgb(colors::BORDER_INSET))
                            .bg(rgb(colors::BG_INSET)),
                    ),
            )
    }
}

fn main() {
    App::new().run(|cx: &mut AppContext| {
        let bounds = Bounds::centered(
            None,
            size(px(layout::WINDOW_WIDTH), px(layout::WINDOW_HEIGHT)),
            cx,
        );
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                ..Default::default()
            },
            |cx| cx.new_view(|_cx| Neumorphism),
        );
    });
}
