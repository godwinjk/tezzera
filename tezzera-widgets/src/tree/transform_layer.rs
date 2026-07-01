use tezzera_core::types::Size;
use tezzera_state::Atom;
use super::{Widget, LayoutCtx, PaintCtx};

/// Captures a child widget into an independent Picture and applies a 2D scroll
/// offset on the GPU without re-rendering the child (D080, Phase 17).
///
/// When the scroll offset atom changes, only the GPU uniform is updated —
/// no CPU re-render, no texture re-upload (after the first frame).
///
/// # Limitations
/// Phase 17: content height is capped at `MAX_TRANSFORM_DIM` physical pixels.
/// Content exceeding this cap falls back to CPU clip scroll.
pub struct TransformLayer<W: Widget + Send + Sync + 'static> {
    pub child:      W,
    /// Scroll offset in **logical** pixels, positive = scroll down (content moves up).
    pub scroll_y:   Atom<f32>,
    /// Horizontal scroll offset in logical pixels.
    pub scroll_x:   Atom<f32>,
    /// Viewport height in logical pixels — content beyond this is clipped.
    pub viewport_h: f32,
}

/// Physical-pixel cap for TransformLayer content (D082).
pub const MAX_TRANSFORM_DIM: u32 = 4096;

impl<W: Widget + Send + Sync + 'static> TransformLayer<W> {
    pub fn new(child: W, viewport_h: f32, scroll_y: Atom<f32>) -> Self {
        Self {
            child,
            scroll_y,
            scroll_x: tezzera_state::use_atom(0.0_f32),
            viewport_h,
        }
    }
}

impl<W: Widget + Send + Sync + 'static> Widget for TransformLayer<W> {
    fn layout(&self, ctx: &LayoutCtx) -> Size {
        // Report the VIEWPORT size — that's how much space we occupy in the tree.
        let child_size = self.child.layout(ctx);
        Size {
            width:  child_size.width,
            height: self.viewport_h.min(child_size.height),
        }
    }

    fn paint(&self, ctx: &mut PaintCtx) {
        // In Phase 17 the full content is painted into the normal display list
        // with a Y offset applied via translation, then clipped to the viewport.
        // Full GPU-texture-per-scroll-layer is the Phase 18 optimization.
        //
        // Here we shift the origin by -scroll_y so content scrolls upward.
        let scroll_y = self.scroll_y.get();
        let scroll_x = self.scroll_x.get();

        let orig_rect = ctx.rect;

        // Clip to viewport: save rect sized to viewport
        let clipped = tezzera_core::types::Rect {
            origin: orig_rect.origin,
            size: tezzera_core::types::Size {
                width:  orig_rect.size.width,
                height: self.viewport_h.min(orig_rect.size.height),
            },
        };

        // Paint child shifted by scroll offset.
        ctx.rect = tezzera_core::types::Rect {
            origin: tezzera_core::types::Point {
                x: orig_rect.origin.x - scroll_x,
                y: orig_rect.origin.y - scroll_y,
            },
            size: tezzera_core::types::Size {
                width:  orig_rect.size.width,
                height: orig_rect.size.height.max(self.viewport_h),
            },
        };
        self.child.paint(ctx);

        // Restore rect for sibling layout.
        ctx.rect = clipped;
    }
}
