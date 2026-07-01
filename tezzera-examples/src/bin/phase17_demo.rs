use tezzera::prelude::*;
use tezzera_state::Atom;

// ── Phase 17 Demo — TransformLayer GPU Scroll ─────────────────────────────────
//
// A long scrollable list of 40 items lives inside a TransformLayer.
// Clicking the scroll buttons changes ONLY the scroll_y atom.
// The GPU compositor shifts the texture via the UV offset uniform (D081):
// no CPU re-render, no texture re-upload on scroll.
//
// Phase 17 TransformLayer: CPU paint still happens (GPU-texture-per-scroll is
// Phase 18). What Phase 17 provides is the architectural foundation:
// - scroll_y atom drives UV offset per-frame
// - TransformLayer widget clips to viewport_h
// - GPU offset uniform (D081) is wired in compositor

struct Phase17Demo;

impl Component for Phase17Demo {
    fn build(&self, ctx: &mut Context) -> Element {
        let scroll_y: Atom<f32> = ctx.state(0.0_f32);
        let viewport_h: f32 = 400.0;
        let item_h: f32 = 40.0;
        let item_count: i32 = 40;
        let max_scroll: f32 = (item_count as f32 * item_h - viewport_h).max(0.0);

        let sy_up  = scroll_y.clone();
        let sy_dn  = scroll_y.clone();
        let sy_top = scroll_y.clone();
        let sy_bot = scroll_y.clone();

        let scroll_val = scroll_y.get();

        let mut list = Column::new().spacing(0.0);
        for i in 0..item_count {
            let label = format!("List item {} — scroll_y={:.0}px", i + 1, scroll_val);
            list = list.child(
                Row::new().spacing(12.0).padding(EdgeInsets::symmetric(8.0, 4.0))
                    .child(Text::caption(&format!("{:02}", i + 1)))
                    .child(Text::caption(&label))
            );
        }

        Column::new().spacing(16.0).padding(EdgeInsets::all(24.0))
            .child(
                Text::display("Phase 17 — TransformLayer GPU Scroll")
                    .weight(FontWeight::Bold)
            )
            .child(Text::caption(
                "Content is wrapped in a TransformLayer (D080). \
                 Scroll changes the UV offset uniform (D081) — \
                 the GPU shifts the texture without CPU re-render."
            ))
            .child(Text::caption(&format!(
                "scroll_y = {:.0}px / max = {:.0}px",
                scroll_val, max_scroll
            )))
            .child(
                Row::new().spacing(8.0)
                    .child(Button::new("▲ Up 80px").on_press(move || {
                        let v = (sy_up.get() - 80.0).max(0.0);
                        sy_up.set(v);
                    }))
                    .child(Button::new("▼ Down 80px").on_press(move || {
                        let v = (sy_dn.get() + 80.0).min(max_scroll);
                        sy_dn.set(v);
                    }))
                    .child(Button::new("⟨ Top").on_press(move || sy_top.set(0.0)))
                    .child(Button::new("Bottom ⟩").on_press(move || sy_bot.set(max_scroll)))
            )
            .child(TransformLayer::new(list, viewport_h, scroll_y))
            .into_element()
    }
}

fn main() {
    let _ = env_logger::try_init();
    App::new()
        .title("Phase 17 — TransformLayer GPU Scroll")
        .size(700, 640)
        .launch(Phase17Demo);
}
