use crate::role::Role;

/// A single accessibility node in the tree.
#[derive(Debug, Clone)]
pub struct A11yNode {
    /// Unique ID within the tree (assigned by A11yTree::add).
    pub id: u64,
    pub role: Role,
    /// Short accessible label (e.g. button text, image alt text).
    pub label: String,
    /// Longer description (optional).
    pub description: Option<String>,
    /// Whether this node can receive keyboard focus.
    pub focusable: bool,
    /// Whether this node currently has focus.
    pub focused: bool,
    /// Whether this node is disabled (not interactive).
    pub disabled: bool,
    /// Current value for range inputs (Slider, ProgressBar).
    pub value: Option<f32>,
    /// Min/max for range inputs.
    pub value_min: Option<f32>,
    pub value_max: Option<f32>,
    /// Checked state for Checkbox/Switch (None = not applicable).
    pub checked: Option<bool>,
    /// Child node IDs.
    pub children: Vec<u64>,
}

impl A11yNode {
    pub fn new(role: Role, label: impl Into<String>) -> Self {
        Self {
            id: 0,
            role,
            label: label.into(),
            description: None,
            focusable: false,
            focused: false,
            disabled: false,
            value: None,
            value_min: None,
            value_max: None,
            checked: None,
            children: Vec::new(),
        }
    }

    pub fn focusable(mut self, f: bool) -> Self {
        self.focusable = f;
        self
    }
    pub fn disabled(mut self, d: bool) -> Self {
        self.disabled = d;
        self
    }
    pub fn description(mut self, d: impl Into<String>) -> Self {
        self.description = Some(d.into());
        self
    }
    pub fn value(mut self, v: f32, min: f32, max: f32) -> Self {
        self.value = Some(v);
        self.value_min = Some(min);
        self.value_max = Some(max);
        self
    }
    pub fn checked(mut self, c: bool) -> Self {
        self.checked = Some(c);
        self
    }
    pub fn child(mut self, id: u64) -> Self {
        self.children.push(id);
        self
    }

    /// Emit this node as a JSON object string (no serde dep — hand-written).
    pub fn to_json(&self) -> String {
        let desc = self
            .description
            .as_deref()
            .map(|d| format!(r#","description":"{}""#, escape_json(d)))
            .unwrap_or_default();
        let val = self
            .value
            .map(|v| {
                format!(
                    r#","aria-valuenow":{},"aria-valuemin":{},"aria-valuemax":{}"#,
                    v,
                    self.value_min.unwrap_or(0.0),
                    self.value_max.unwrap_or(1.0),
                )
            })
            .unwrap_or_default();
        let checked = self
            .checked
            .map(|c| format!(r#","aria-checked":{}"#, c))
            .unwrap_or_default();
        let level = self
            .role
            .aria_level()
            .map(|l| format!(r#","aria-level":{}"#, l))
            .unwrap_or_default();
        let disabled = if self.disabled {
            r#","aria-disabled":true"#
        } else {
            ""
        };
        format!(
            r#"{{"id":{},"role":"{}","aria-label":"{}","focusable":{}"#,
            self.id,
            self.role.aria_role(),
            escape_json(&self.label),
            self.focusable
        ) + &desc
            + &val
            + &checked
            + &level
            + disabled
            + "}"
    }
}

fn escape_json(s: &str) -> String {
    s.replace('\\', "\\\\")
        .replace('"', "\\\"")
        .replace('\n', "\\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn node_new_defaults() {
        let n = A11yNode::new(Role::Button, "Click me");
        assert_eq!(n.id, 0);
        assert_eq!(n.label, "Click me");
        assert!(!n.focusable);
        assert!(!n.focused);
        assert!(!n.disabled);
        assert!(n.description.is_none());
        assert!(n.value.is_none());
        assert!(n.checked.is_none());
        assert!(n.children.is_empty());
    }

    #[test]
    fn node_focusable_setter() {
        let n = A11yNode::new(Role::Button, "OK").focusable(true);
        assert!(n.focusable);
    }

    #[test]
    fn node_disabled_setter() {
        let n = A11yNode::new(Role::Button, "Disabled").disabled(true);
        assert!(n.disabled);
    }

    #[test]
    fn node_value_setter() {
        let n = A11yNode::new(Role::Slider, "Volume").value(0.5, 0.0, 1.0);
        assert_eq!(n.value, Some(0.5));
        assert_eq!(n.value_min, Some(0.0));
        assert_eq!(n.value_max, Some(1.0));
    }

    #[test]
    fn node_checked_setter() {
        let n = A11yNode::new(Role::Checkbox, "Accept").checked(true);
        assert_eq!(n.checked, Some(true));
    }

    #[test]
    fn node_to_json_contains_role() {
        let n = A11yNode::new(Role::Button, "Submit");
        let json = n.to_json();
        assert!(json.contains(r#""role":"button""#));
    }

    #[test]
    fn node_to_json_contains_label() {
        let n = A11yNode::new(Role::Button, "Submit");
        let json = n.to_json();
        assert!(json.contains(r#""aria-label":"Submit""#));
    }

    #[test]
    fn node_to_json_escape_quotes() {
        let n = A11yNode::new(Role::Button, r#"Say "hello""#);
        let json = n.to_json();
        assert!(json.contains(r#"Say \"hello\""#));
    }
}
