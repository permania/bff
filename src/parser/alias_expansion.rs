use crate::config::schema::TreeConfig;

pub trait ExpandAlias {
    fn expand(&self, conf: &TreeConfig) -> Self;
}

impl ExpandAlias for Vec<String> {
    /// Expands each string in the vector using the alias mappings from `TreeConfig`.
    ///
    /// If a string matches a key in `.bff.toml (TreeConfig.alias)`, it is replaced by the mapped value.
    /// Otherwise, it remains unchanged.
    fn expand(&self, conf: &TreeConfig) -> Self {
        let aliases = &conf.alias;

        let expanded: Vec<String> = self
            .iter()
            .flat_map(|s| {
                let word = aliases.get(s).unwrap_or(s).to_string();
                word.split_whitespace()
                    .map(|w| w.to_string())
                    .collect::<Vec<String>>()
            })
            .collect();

        expanded
    }
}
