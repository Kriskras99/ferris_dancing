/// Defines an import (`use` statement).
#[derive(Debug, Clone)]
pub struct Import {
    line: String,

    /// Function visibility
    pub vis: Option<String>,
}

impl Import {
    /// Return a new import.
    pub fn new(path: impl ToString, ty: impl ToString) -> Self {
        Import {
            line: format!("{}::{}", path.to_string(), ty.to_string()),
            vis: None,
        }
    }

    /// Set the import visibility.
    pub fn vis(&mut self, vis: impl ToString) -> &mut Self {
        self.vis = Some(vis.to_string());
        self
    }
}
