#[derive(PartialEq, Clone)]
pub struct Config {
    pub auto: bool,
    pub show_selected: bool,
    pub multi_select: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            auto: true,
            show_selected: false,
            multi_select: false,
        }
    }
}
