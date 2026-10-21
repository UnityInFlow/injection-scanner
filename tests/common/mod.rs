use std::sync::OnceLock;

use injection_scanner::patterns::load_embedded_patterns;
use injection_scanner::scanner::Scanner;

/// Each integration-test binary compiles its embedded patterns once.
pub fn scanner() -> &'static Scanner {
    static SCANNER: OnceLock<Scanner> = OnceLock::new();
    SCANNER.get_or_init(|| {
        Scanner::new(&load_embedded_patterns().expect("embedded patterns must load"))
            .expect("patterns must compile")
    })
}
