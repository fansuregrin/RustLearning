//! # Art
//!
//! A library for modeling artistic concepts.

pub use self::kinds::PrimaryColor;
pub use self::kinds::SecondaryColor;
pub use self::utils::mix;

pub mod kinds {
    /// The primary colors according to the RYB color model.
    #[derive(Debug)]
    pub enum PrimaryColor {
        Red,
        Yellow,
        Blue,
    }

    /// The secondary colors according to the RYB color model.
    #[derive(Debug)]
    pub enum SecondaryColor {
        Orange,
        Green,
        Purple,
    }
}

pub mod utils {
    use super::kinds::*;

    /// Combines two primary colors in equal amounts to create
    /// a secondary color.
    pub fn mix(
        c1: PrimaryColor,
        c2: PrimaryColor
    ) -> Result<SecondaryColor, &'static str> {
        match c1 {
            PrimaryColor::Red => {
                match c2 {
                    PrimaryColor::Red => Err("c2 must be different from c1"),
                    PrimaryColor::Yellow => Ok(SecondaryColor::Orange),
                    PrimaryColor::Blue => Ok(SecondaryColor::Purple),
                } 
            },
            PrimaryColor::Yellow => {
                match c2 {
                    PrimaryColor::Red => Ok(SecondaryColor::Orange),
                    PrimaryColor::Yellow => Err("c2 must be different from c1"),
                    PrimaryColor::Blue => Ok(SecondaryColor::Green),
                } 
            }
            PrimaryColor::Blue => {
                match c2 {
                    PrimaryColor::Red => Ok(SecondaryColor::Purple),
                    PrimaryColor::Yellow => Ok(SecondaryColor::Green),
                    PrimaryColor::Blue => Err("c2 must be different from c1"),
                } 
            },
        }
    }
}