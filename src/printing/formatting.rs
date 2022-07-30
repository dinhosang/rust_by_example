use std::fmt::{self, Display, Formatter, Octal, UpperHex};

struct City {
    name: &'static str,
    // Latitude
    lat: f32,
    // Longitude
    lon: f32,
}

impl Display for City {
    // `f` is a buffer, and this method must write the formatted string into it
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let lat_c = if self.lat >= 0.0 { 'N' } else { 'S' };
        let lon_c = if self.lon >= 0.0 { 'E' } else { 'W' };

        // `write!` is like `format!`, but it will write the formatted string
        // into a buffer (the first argument)
        write!(
            f,
            "{}: {:.3}°{} {:.3}°{}",
            self.name,
            self.lat.abs(),
            lat_c,
            self.lon.abs(),
            lon_c
        )
    }
}

#[derive(Debug)]
struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

impl Octal for Color {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "Color<Octal> {{ Red: 0o{:0>2o}, Green: 0o{:0>2o}, Blue: 0o{:0>2o} }}",
            self.red, self.green, self.blue
        )
    }
}

impl UpperHex for Color {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "Color<UpperHex> {{ Red: 0x{:0>2X}, Green: 0x{:0>2X}, Blue: 0x{:0>2X} }}",
            self.red, self.green, self.blue
        )
    }
}

pub fn some_formatting() {
    for city in [
        City {
            name: "Dublin",
            lat: 53.347_78,
            lon: -6.259722,
        },
        City {
            name: "Oslo",
            lat: 59.95,
            lon: 10.75,
        },
        City {
            name: "Vancouver",
            lat: 49.25,
            lon: -123.1,
        },
    ]
    .iter()
    {
        println!("{}", *city);
    }
    for color in [
        Color {
            red: 128,
            green: 255,
            blue: 90,
        },
        Color {
            red: 0,
            green: 3,
            blue: 254,
        },
        Color {
            red: 0,
            green: 0,
            blue: 0,
        },
    ]
    .iter()
    {
        // Switch this to use {} once you've added an implementation
        // for fmt::Display.
        println!("{:?}", *color);
        println!("{:o}", *color);
        println!("{:X}", *color);
    }
}
