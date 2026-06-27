use std::fs;
use std::path::Path;

pub struct NewOptions {
    pub name: String,
}

impl NewOptions {
    pub fn from_args(args: &[String]) -> Result<Self, String> {
        let name = args.first()
            .ok_or_else(|| "usage: tzr new <name>".to_string())?
            .clone();
        // Validate: only alphanumeric + underscores + hyphens
        if !name.chars().all(|c| c.is_alphanumeric() || c == '_' || c == '-') {
            return Err(format!("invalid project name '{}': use letters, numbers, - or _", name));
        }
        Ok(Self { name })
    }
}

pub fn run(opts: NewOptions) -> Result<(), String> {
    let name = &opts.name;
    let dir = Path::new(name);

    if dir.exists() {
        return Err(format!("directory '{}' already exists", name));
    }

    println!("Creating TEZZERA project '{}'...", name);

    fs::create_dir_all(dir.join("src"))
        .map_err(|e| format!("failed to create directories: {}", e))?;

    write_file(dir.join("Cargo.toml"), &cargo_toml(name))?;
    write_file(dir.join("src").join("main.rs"), &main_rs(name))?;
    write_file(dir.join(".gitignore"), "/target\n")?;

    println!();
    println!("  Created '{}'", name);
    println!();
    println!("  Next steps:");
    println!("    cd {}", name);
    println!("    tzr dev");
    println!();
    Ok(())
}

fn write_file(path: impl AsRef<Path>, content: &str) -> Result<(), String> {
    fs::write(&path, content)
        .map_err(|e| format!("failed to write {}: {}", path.as_ref().display(), e))
}

fn cargo_toml(name: &str) -> String {
    format!(r#"[package]
name = "{name}"
version = "0.1.0"
edition = "2021"

[[bin]]
name = "{name}"
path = "src/main.rs"

[dependencies]
tezzera-core     = {{ git = "https://github.com/tezzera-ui/tezzera" }}
tezzera-layout   = {{ git = "https://github.com/tezzera-ui/tezzera" }}
tezzera-render   = {{ git = "https://github.com/tezzera-ui/tezzera" }}
tezzera-state    = {{ git = "https://github.com/tezzera-ui/tezzera" }}
tezzera-platform = {{ git = "https://github.com/tezzera-ui/tezzera" }}
tezzera-theme    = {{ git = "https://github.com/tezzera-ui/tezzera" }}
tezzera-widgets  = {{ git = "https://github.com/tezzera-ui/tezzera" }}
tezzera-animate  = {{ git = "https://github.com/tezzera-ui/tezzera" }}
"#)
}

fn main_rs(name: &str) -> String {
    let title = name.to_uppercase();
    let title_len = title.len();
    format!(r#"use tezzera_core::types::{{Point, Rect, Size}};
use tezzera_platform::{{InputEvent, MouseButton, TezzeraApp}};
use tezzera_render::{{Color, FontCache, SkiaCanvas}};
use tezzera_state::use_atom;

const W: u32 = 480;
const H: u32 = 320;

const BG:     Color = Color::rgb(18, 18, 28);
const ACCENT: Color = Color::rgb(103, 80, 164);
const TEXT:   Color = Color::rgb(230, 225, 229);

fn main() {{
    let font = FontCache::system_mono().expect("no system font found");
    let count = use_atom(0_i32);
    let mut mx = 0.0_f32;
    let mut my = 0.0_f32;

    TezzeraApp::new()
        .title("{title} — TEZZERA")
        .size(W, H)
        .run(move |canvas: &mut SkiaCanvas, events: &[InputEvent]| {{
            for ev in events {{
                match ev {{
                    InputEvent::MouseDown {{ x, y, button: MouseButton::Left }} => {{
                        let bx = W as f32 / 2.0 - 60.0;
                        let by = H as f32 / 2.0 + 20.0;
                        if *x >= bx && *x <= bx + 120.0 && *y >= by && *y <= by + 40.0 {{
                            count.update(|n| n + 1);
                        }}
                    }}
                    InputEvent::MouseMove {{ x, y }} => {{ mx = *x; my = *y; }}
                    _ => {{}}
                }}
            }}

            canvas.clear(BG);

            canvas.draw_text(
                "{title}",
                Point {{ x: W as f32 / 2.0 - {title_len}.0 * 9.0 / 2.0, y: 40.0 }},
                TEXT, &font, 18.0,
            );

            let label = format!("{{}}", count.get());
            canvas.draw_text(
                &label,
                Point {{ x: W as f32 / 2.0 - label.len() as f32 * 12.0 / 2.0, y: H as f32 / 2.0 - 20.0 }},
                ACCENT, &font, 36.0,
            );

            let bx = W as f32 / 2.0 - 60.0;
            let by = H as f32 / 2.0 + 20.0;
            let hovered = mx >= bx && mx <= bx + 120.0 && my >= by && my <= by + 40.0;
            let btn_color = if hovered {{ Color::rgb(130, 100, 200) }} else {{ ACCENT }};
            canvas.fill_rect(
                Rect {{ origin: Point {{ x: bx, y: by }}, size: Size {{ width: 120.0, height: 40.0 }} }},
                btn_color,
            );
            canvas.draw_text("Click me", Point {{ x: bx + 18.0, y: by + 12.0 }}, Color::WHITE, &font, 14.0);
        }});
}}
"#, title = title, title_len = title_len)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn new_opts_parses_name() {
        let args = vec!["my_app".to_string()];
        let opts = NewOptions::from_args(&args).unwrap();
        assert_eq!(opts.name, "my_app");
    }

    #[test]
    fn new_opts_errors_on_missing_name() {
        assert!(NewOptions::from_args(&[]).is_err());
    }

    #[test]
    fn new_opts_rejects_invalid_chars() {
        let args = vec!["my app!".to_string()];
        assert!(NewOptions::from_args(&args).is_err());
    }

    #[test]
    fn cargo_toml_contains_name() {
        let toml = cargo_toml("hello_world");
        assert!(toml.contains("hello_world"));
        assert!(toml.contains("tezzera-platform"));
    }

    #[test]
    fn main_rs_contains_title() {
        let src = main_rs("my_app");
        assert!(src.contains("MY_APP"));
        assert!(src.contains("use_atom"));
    }

    #[test]
    fn run_creates_directory_and_files() {
        let name = format!("_test_new_{}", std::process::id());
        let opts = NewOptions { name: name.clone() };
        run(opts).unwrap();
        assert!(std::path::Path::new(&name).join("Cargo.toml").exists());
        assert!(std::path::Path::new(&name).join("src/main.rs").exists());
        assert!(std::path::Path::new(&name).join(".gitignore").exists());
        fs::remove_dir_all(&name).unwrap();
    }

    #[test]
    fn run_errors_if_dir_exists() {
        let name = format!("_test_exists_{}", std::process::id());
        fs::create_dir(&name).unwrap();
        let result = run(NewOptions { name: name.clone() });
        assert!(result.is_err());
        fs::remove_dir_all(&name).unwrap();
    }
}
