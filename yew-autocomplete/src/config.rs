#[derive(PartialEq, Clone)]
pub struct Config {
    pub show_selected: bool,
    pub multi_select: bool,
}

#[allow(clippy::derivable_impls)]
impl Default for Config {
    fn default() -> Self {
        Self {
            show_selected: false,
            multi_select: false,
        }
    }
}
