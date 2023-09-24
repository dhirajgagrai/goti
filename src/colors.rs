use palette::named;
use palette::Srgb;

#[derive(Debug, Clone, Copy)]
pub enum Color {
    Honeydew,
    SteelBlue,
}

impl ToString for Color {
    fn to_string(&self) -> String {
        format!("{:?}", self).to_lowercase()
    }
}

type Rgb = Srgb<u8>;

impl From<Color> for Rgb {
    fn from(c: Color) -> Self {
        named::from_str(&c.to_string()).unwrap()
    }
}
