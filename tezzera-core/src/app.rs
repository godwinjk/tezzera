use crate::element::Element;

/// Top-level application builder.
///
/// Holds the root element and (in future phases) the platform event-loop
/// handle, window configuration, and service registrations.
pub struct TezzeraApp {
    root: Option<Element>,
}

impl TezzeraApp {
    /// Creates a new `TezzeraApp` with no root element.
    pub fn new() -> Self {
        TezzeraApp { root: None }
    }

    /// Sets the root element of the application.
    pub fn child(mut self, element: impl Into<Element>) -> Self {
        self.root = Some(element.into());
        self
    }

    /// Starts the application event loop.
    ///
    /// # Phase 1 placeholder
    ///
    /// The real event loop is wired in `tezzera-render` (GPU path) and
    /// `tezzera-cli` (terminal path). This stub exists so application entry
    /// points compile against `tezzera-core` without pulling in renderer crates.
    pub fn run(self) {
        todo!("wire up in tezzera-render/tezzera-cli")
    }
}

impl Default for TezzeraApp {
    fn default() -> Self {
        TezzeraApp::new()
    }
}
