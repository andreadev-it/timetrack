use colored::{Colorize, ColoredString};

pub enum Styles {
    Error,
    Title,
    Message,
    Primary,
    Secondary
}

pub fn style_string(label: &str, style: Styles) -> ColoredString {
    match style {
        Styles::Error => label.red().bold(),
        Styles::Title => label.bold(),
        Styles::Message => label.bold(),
        Styles::Primary => label.green().bold(),
        Styles::Secondary => label.blue(),
    }
}
