use crate::context::Context;
use crate::element::Element;
use crate::types::ComponentId;

/// The core trait that every TEZZERA component must implement.
///
/// A component is a pure function from props (via `&self`) and context to an
/// `Element` tree. Implement this trait on any struct that carries component
/// configuration (props, slots, etc.).
pub trait TezzeraComponent: 'static {
    /// Produces the `Element` tree for this component.
    ///
    /// Called by the framework during the build phase. Never call directly from
    /// application code; use `mount` so lifecycle tracing is included.
    fn build(&self, ctx: &mut Context) -> Element;

    /// Returns the fully-qualified type name used in diagnostics and tracing.
    fn type_name(&self) -> &'static str {
        std::any::type_name::<Self>()
    }

    /// Framework-level entry point: emits `ComponentMount` then delegates to
    /// `build`. Prefer this over calling `build` directly.
    fn mount(&self, ctx: &mut Context) -> Element {
        #[cfg(debug_assertions)]
        {
            use tezzera_trace::{event::TezzeraTrace, location, trace};
            trace!(TezzeraTrace::ComponentMount {
                id: ctx.component_id(),
                name: self.type_name(),
                location: location!(),
            });
        }
        self.build(ctx)
    }

    /// Framework-level unmount hook: emits `ComponentUnmount`.
    ///
    /// The caller is responsible for dropping the `Context` (which runs
    /// cleanup callbacks) before or after calling this method.
    fn unmount(&self, id: ComponentId) {
        #[cfg(debug_assertions)]
        {
            use tezzera_trace::{event::TezzeraTrace, trace};
            trace!(TezzeraTrace::ComponentUnmount {
                id,
                name: self.type_name(),
            });
        }
        // Suppress unused-variable warning in release builds.
        let _ = id;
    }
}
