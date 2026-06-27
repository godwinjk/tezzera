use crate::types::ComponentId;
use tezzera_state::{use_atom as create_atom, Atom};

/// Per-component build context passed to every `TezzeraComponent::build` call.
///
/// The context is the component's handle to the framework during a build: it
/// carries the component's identity and accumulates cleanup callbacks that run
/// when the component unmounts.
pub struct Context {
    /// Identity of the component that owns this context.
    pub(crate) component_id: ComponentId,
    /// Callbacks registered via `on_cleanup` / `on_unmount` / `on_mount`.
    pub(crate) unmount_callbacks: Vec<Box<dyn FnOnce() + Send>>,
}

impl Context {
    /// Creates a new `Context` for the component with the given `id`.
    pub fn new(id: ComponentId) -> Self {
        Context {
            component_id: id,
            unmount_callbacks: Vec::new(),
        }
    }

    /// Returns the `ComponentId` of the component that owns this context.
    pub fn component_id(&self) -> ComponentId {
        self.component_id
    }

    /// Registers a cleanup function to run when the component unmounts.
    pub fn on_cleanup(&mut self, f: impl FnOnce() + Send + 'static) {
        self.unmount_callbacks.push(Box::new(f));
    }

    /// Creates a local atom scoped to this component's lifetime.
    /// The atom is initialized with `default` on first call.
    pub fn state<T: Clone + Send + Sync + 'static>(&mut self, default: T) -> Atom<T> {
        create_atom(default)
    }
}

impl Drop for Context {
    fn drop(&mut self) {
        // Drain and invoke every registered cleanup in registration order.
        let callbacks: Vec<Box<dyn FnOnce() + Send>> =
            std::mem::take(&mut self.unmount_callbacks);
        for cb in callbacks {
            cb();
        }
    }
}
