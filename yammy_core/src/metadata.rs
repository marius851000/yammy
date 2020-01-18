/// Store metadata about a [super::Mod]
#[derive(Default, Clone)]
pub struct Metadata {
    /// The displayed name of the mod
    pub name: String,
    /// A list of author of the mod
    pub authors: Vec<String>,
    /// A description of the mod
    pub description: String,
    /// The license of the mod
    pub license: String,
}
