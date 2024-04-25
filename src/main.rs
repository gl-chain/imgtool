//! 用于根文档

use std::env;
use std::path::Path;

/// 用于注释
/// # Examples
/// ``` Rust
/// let a = 1;
/// let b = 1;
/// assert!(a==b);
/// ```
fn main() {
    let image_path = env::args().skip(1).next().unwrap();
    let path = Path::new(&image_path);
    let img = image::open(path).unwrap();
    let rotated = img.rotate90();
    rotated.save(path).unwrap();
}
