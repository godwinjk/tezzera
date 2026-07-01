use tezzera::prelude::*;
use tezzera_state::Atom;

// ── Phase 16 Demo — Multi-Layer GPU Compositing ───────────────────────────────
//
// The base widget tree renders into the main SkiaCanvas.
// A dialog overlay renders into a SEPARATE overlay SkiaCanvas (D076).
// GpuPresenter.present_layers() uploads both as GPU textures and composites
// them with Porter-Duff "over" using wgpu ALPHA_BLENDING pipeline (D079).
//
// When the dialog is closed, the overlay canvas is cleared to transparent (D078);
// the base layer is NOT re-uploaded unless its own atoms changed.

struct Phase16Demo;

impl Component for Phase16Demo {
    fn build(&self, ctx: &mut Context) -> Element {
        let show_dialog: Atom<bool> = ctx.state(false);
        let click_count: Atom<i32>  = ctx.state(0_i32);

        let sd_open  = show_dialog.clone();
        let sd_close = show_dialog.clone();
        let cc       = click_count.clone();
        let cnt      = click_count.get();

        if show_dialog.get() {
            push_overlay(
                OverlayEntry::new(
                    LayerPosition::Centered,
                    Card::new(
                        Column::new().spacing(16.0).padding(EdgeInsets::all(24.0))
                            .child(Text::heading("Dialog — Overlay Layer"))
                            .child(Text::caption(
                                "This dialog renders in the OVERLAY SkiaCanvas (D076, D078)."
                            ))
                            .child(Text::caption(
                                "It is uploaded as a separate GPU texture and blended on top \
                                 via the wgpu ALPHA_BLENDING pipeline (D079)."
                            ))
                            .child(Text::caption(
                                "The base layer texture is NOT re-uploaded when only the \
                                 overlay changes — zero base CPU re-render."
                            ))
                            .child(
                                Button::new("Close Dialog")
                                    .on_press(move || sd_close.set(false))
                            )
                    )
                )
                .scrim(ScrimConfig {
                    color: Color::rgba(0, 0, 0, 160),
                    on_tap: Some(std::sync::Arc::new(move || sd_open.set(false))),
                })
                .input(InputBehavior::Block)
            );
        }

        Column::new().spacing(20.0).padding(EdgeInsets::all(32.0))
            .child(
                Text::display("Phase 16 — Multi-Layer GPU Compositing")
                    .weight(FontWeight::Bold)
            )
            .child(Text::caption(
                "Base layer renders here in the main SkiaCanvas. \
                 Overlay renders in a SEPARATE SkiaCanvas uploaded as its own GPU texture."
            ))
            .child(Text::caption(
                "GPU compositor: pass 1 blits base with REPLACE blend; \
                 pass 2 blits overlay with ALPHA_BLENDING (Porter-Duff over)."
            ))
            .child(Text::caption(
                "Softbuffer fallback: overlay is CPU-composited using integer \
                 Porter-Duff arithmetic — same visual result."
            ))
            .child(
                Row::new().spacing(12.0)
                    .child(
                        Button::new("Open Dialog Overlay")
                            .on_press(move || show_dialog.set(true))
                    )
                    .child(
                        Button::new("Increment Counter")
                            .on_press(move || cc.set(click_count.get() + 1))
                    )
                    .child(Text::display(&format!("Count: {cnt}")).weight(FontWeight::Bold))
            )
            .into_element()
    }
}

fn main() {
    let _ = env_logger::try_init();
    App::new()
        .title("Phase 16 — Multi-Layer GPU Compositing")
        .size(800, 560)
        .launch(Phase16Demo);
}
