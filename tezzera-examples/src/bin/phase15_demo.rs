use tezzera::prelude::*;
use tezzera_state::Atom;

// ── Phase 15 Demo — wgpu GPU Compositor ──────────────────────────────────────
//
// Identical to a Phase 13 counter app, but the platform now blits the tiny-skia
// pixel buffer via a wgpu fullscreen-quad shader instead of softbuffer memcpy
// (D072–D075). There is no visual difference — the GPU path is a transparent
// display backend upgrade. The adapter name is logged to stderr on startup.

struct Phase15Demo;

impl Component for Phase15Demo {
    fn build(&self, ctx: &mut Context) -> Element {
        let count: Atom<i32> = ctx.state(0_i32);
        let cnt = count.get();
        let inc = count.clone();
        let dec = count.clone();
        let inc_count = count.clone();

        Column::new().spacing(20.0).padding(EdgeInsets::all(32.0))
            .child(
                Text::display("Phase 15 — wgpu GPU Compositor")
                    .weight(FontWeight::Bold)
            )
            .child(Text::caption(
                "The CPU pixel buffer is now uploaded to a GPU texture each frame \
                 and blitted via a WGSL fullscreen-quad shader (D075). \
                 Softbuffer is used as a fallback if no GPU adapter is found."
            ))
            .child(Text::caption(
                "Check stderr on startup for: \"wgpu: Metal backend, adapter = …\""
            ))
            .child(
                Row::new().spacing(12.0)
                    .child(Button::new("−").on_press(move || dec.set(count.get() - 1)))
                    .child(Text::display(&cnt.to_string()).weight(FontWeight::Bold))
                    .child(Button::new("+").on_press(move || inc.set(inc_count.get() + 1)))
            )
            .child(Text::caption(
                "Counter renders identically to softbuffer — the GPU path is \
                 pixel-perfect because tiny-skia still draws at physical resolution."
            ))
            .into_element()
    }
}

fn main() {
    // Initialise env_logger so wgpu adapter info reaches stderr.
    let _ = env_logger::try_init();

    App::new()
        .title("Phase 15 — wgpu GPU Compositor")
        .size(720, 480)
        .launch(Phase15Demo);
}
