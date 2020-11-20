
#[derive(Debug)]
pub struct Colour {
    // red, green, blue, opacity
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: f64,
}

impl Colour {
    pub fn new(r: u8, g: u8, b: u8, a: f64) -> Colour {
        assert!(0.0 <= a && a <= 1.0);
        Colour { r, g, b, a }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn make_colour() {
        let white = Colour::new(0, 0, 0, 1.0);
        let black = Colour::new(255, 255, 255, 1.0);
        assert_eq!(white.a, black.a);
        assert_ne!(white.r, black.r);
        assert_eq!(black.g, 255);
        let red = Colour::new(255, 0, 0, 1.0);
        let green = Colour::new(0, 255, 0, 1.0);
        let blue = Colour::new(0, 0, 255, 1.0);
        println!("{:?}", red);
        println!("{:?}", green);
        println!("{:?}", blue);
    }

}