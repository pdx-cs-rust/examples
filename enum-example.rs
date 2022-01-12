#[derive(Debug, Clone, Copy)]
enum Color {
    Red,
    Green,
    Blue,
    Custom(u8, u8, u8),
}
use Color::*;

fn main() {
    let color = Custom(127,127,0);
    let color_name = match color {
        Red => "#ff0000".to_string(),
        Green => "#00ff00".to_string(),
        Blue => "#0000ff".to_string(),
        Custom(r, g, b) => format!("#{:02x}{:02x}{:02x}", r, g, b),
    };
    println!("{}", color_name);
}
