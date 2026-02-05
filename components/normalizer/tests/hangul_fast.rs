// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use icu_normalizer::ComposingNormalizer;

#[test]
fn test_hangul_fast_path_lvt() {
    let normalizer = ComposingNormalizer::new_nfc();
    
    // Test L + V + T composition (should hit fast path)
    // L=0x1100, V=0x1161, T=0x11A8 (T index 1) -> 0xAC01
    // Uncomposed: U+1100 U+1161 U+11A8
    // Composed: U+AC01
    let input = "\u{1100}\u{1161}\u{11A8}";
    let expected = "\u{AC01}";
    assert_eq!(normalizer.normalize(input), expected);
}

#[test]
fn test_hangul_fast_path_lv_ascii() {
    let normalizer = ComposingNormalizer::new_nfc();
    
    // Test L + V + ASCII (should hit fast path for ASCII)
    // L=0x1100, V=0x1161, ASCII='x'
    // Composed: U+AC00 'x'
    let input = "\u{1100}\u{1161}x";
    let expected = "\u{AC00}x";
    assert_eq!(normalizer.normalize(input), expected);
}



#[test]
fn test_hangul_non_starter_t() {
    let normalizer = ComposingNormalizer::new_nfc();
    
    // L+V + Non-T (e.g. 0x1100 again) -> LV + L (Fast path should exit)
    let input = "\u{1100}\u{1161}\u{1100}";
    let expected = "\u{AC00}\u{1100}";
    assert_eq!(normalizer.normalize(input), expected);
}
