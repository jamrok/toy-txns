pub mod libs;
use libs::*;

use std::env;
use std::error::Error;

///=============================================================================
// ArgParse
///=============================================================================
pub struct Params {
    filename: String,
}

impl Params {
    pub fn parse(mut args: env::Args) -> Result<Params, Box<dyn Error>> {
        args.next(); // Skip arg[0]: program name

        // Get the first param
        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("No filename given".into()),
        };

        Ok(Params { filename })
    }

    pub fn filename(&self) -> &String {
        &self.filename
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn precision_64() {
        assert_eq!(crate::precision_64(8.23456, 4), 8.2346);
        assert_eq!(crate::precision_64(8.23454, 4), 8.2345);
        assert_eq!(crate::precision_64(8.23454, 3), 8.235);
        assert_eq!(crate::precision_64(8.23454, 2), 8.23);
        assert_eq!(crate::precision_64(8.23454, 1), 8.2);
    }
}
/// Returns an f64 rounded "half up" to the specified decimal place
pub fn precision_64(num: f64, dp: i32) -> f64 {
    (num * 10.0_f64.powi(dp)).round() / 10.0_f64.powi(dp)
}
