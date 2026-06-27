use crate::context::Context;

/// Runs `f` immediately as the component mounts and registers the returned
/// cleanup function to run when the component unmounts.
///
/// This mirrors the React `useEffect` with an empty dependency array: the
/// setup runs once on mount, the teardown runs once on unmount.
pub fn on_mount<F, Cleanup>(ctx: &mut Context, f: F)
where
    F: FnOnce() -> Cleanup + Send + 'static,
    Cleanup: FnOnce() + Send + 'static,
{
    let cleanup = f();
    ctx.on_cleanup(cleanup);
}

/// Registers `f` to run when the component unmounts.
///
/// Equivalent to calling `ctx.on_cleanup(f)` directly; exposed as a named
/// function so call sites read as intent rather than mechanism.
pub fn on_unmount(ctx: &mut Context, f: impl FnOnce() + Send + 'static) {
    ctx.on_cleanup(f);
}
