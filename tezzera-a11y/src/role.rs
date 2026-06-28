/// Semantic role of a UI element, following WAI-ARIA conventions.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Role {
    Button,
    TextInput,
    Checkbox,
    Switch,
    Slider,
    ProgressBar,
    Image,
    Heading { level: u8 }, // 1–6
    Label,
    Link,
    List,
    ListItem,
    Dialog,
    Alert,
    Navigation,
    Main,
    None,
}

impl Role {
    /// ARIA role string for web output.
    pub fn aria_role(&self) -> &'static str {
        match self {
            Role::Button => "button",
            Role::TextInput => "textbox",
            Role::Checkbox => "checkbox",
            Role::Switch => "switch",
            Role::Slider => "slider",
            Role::ProgressBar => "progressbar",
            Role::Image => "img",
            Role::Heading { .. } => "heading",
            Role::Label => "label",
            Role::Link => "link",
            Role::List => "list",
            Role::ListItem => "listitem",
            Role::Dialog => "dialog",
            Role::Alert => "alert",
            Role::Navigation => "navigation",
            Role::Main => "main",
            Role::None => "none",
        }
    }

    /// ARIA level attribute (only relevant for Heading).
    pub fn aria_level(&self) -> Option<u8> {
        if let Role::Heading { level } = self {
            Some(*level)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn role_button_aria_role() {
        assert_eq!(Role::Button.aria_role(), "button");
    }

    #[test]
    fn role_heading_aria_role() {
        assert_eq!(Role::Heading { level: 2 }.aria_role(), "heading");
    }

    #[test]
    fn role_heading_aria_level() {
        assert_eq!(Role::Heading { level: 3 }.aria_level(), Some(3));
    }

    #[test]
    fn role_slider_aria_role() {
        assert_eq!(Role::Slider.aria_role(), "slider");
    }

    #[test]
    fn role_none_has_no_level() {
        assert_eq!(Role::None.aria_level(), Option::<u8>::None);
    }

    #[test]
    fn role_eq() {
        assert_eq!(Role::Button, Role::Button);
        assert_ne!(Role::Button, Role::Link);
        assert_eq!(Role::Heading { level: 1 }, Role::Heading { level: 1 });
        assert_ne!(Role::Heading { level: 1 }, Role::Heading { level: 2 });
    }

    #[test]
    fn role_clone() {
        let r = Role::Heading { level: 4 };
        let r2 = r.clone();
        assert_eq!(r, r2);
    }
}
