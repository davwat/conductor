//pub mod esp32c6; // TODO: only import if esp32c6 etc (or similar)... conditional compilation??
                 // or perhaps do something with extern if thats even possible idk lol
                 // pub use??

mod esp32c6;
//#[cfg(target_platform = "esp32c6")]
pub use self::esp32c6::*;
