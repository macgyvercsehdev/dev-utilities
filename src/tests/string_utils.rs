use crate::utils::string_utils::remove_mask;


#[test]
fn test_remove_mask_basic() {
    // Basic test case for removing mask
    assert_eq!(remove_mask("123-456-789", "-"), "123456789");

    // Test case with a different mask
    assert_eq!(remove_mask("(123) 456-7890", "()- "), "1234567890");
}

#[test]
fn test_remove_mask_empty_input() {
    // Test case with empty input string
    assert_eq!(remove_mask("", "-"), "");

    // Test case with empty mask
    assert_eq!(remove_mask("1234567890", ""), "1234567890");
}

#[test]
fn test_remove_mask_no_mask_chars() {
    // Test case with no mask characters in input
    assert_eq!(remove_mask("1234567890", "-"), "1234567890");
}

#[test]
fn test_remove_mask_special_characters() {
    // Test case with special characters in input and mask
    assert_eq!(remove_mask("12#34%56@78", "#%@"), "12345678");
}

#[test]
fn test_remove_mask_repeated_mask_chars() {
    // Test case with repeated mask characters
    assert_eq!(remove_mask("1-2-3-4-5", "-"), "12345");
}

#[test]
fn test_remove_mask_unicode_chars() {
    // Test case with unicode characters in input and mask
    assert_eq!(remove_mask("こんにちは, world!", "にち, "), "こんはworld!");
}
