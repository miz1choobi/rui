use crate::*;

const SLIDER_WIDTH :f32 = 4.0;
const SLIDER_THUMB_RADIUS : f32 = 10.0;

/// Horizontal slider built from other Views.
pub fn slider(value: impl Binding<f32>) -> impl View {
    state(0.0, move |width| {
        let w = width.get();
        let x = value.get() * w;
        let value = value.clone();

        zstack((
            rectangle()
                .color(CLEAR_COLOR)
                .drag(move |off, _state| {
                    value.set( (value.get() + off.x / w).clamp(0.0,1.0));
                }),
            canvas(move |sz, vger| {
                let c = sz.center();
                let paint = vger.color_paint(BUTTON_BACKGROUND_COLOR);
                vger.fill_rect(
                    [0.0, c.y-SLIDER_WIDTH/2.0].into(),
                    [sz.width(), c.y+SLIDER_WIDTH/2.0].into(),
                    0.0,
                    paint
                );
                let paint = vger.color_paint(AZURE_HIGHLIGHT);
                vger.fill_circle(
                    [x, c.y],
                    SLIDER_THUMB_RADIUS,
                    paint
                );
            }),
        ))
        .geom(move |sz| if sz.width != w { width.set(sz.width) })
    })
}
