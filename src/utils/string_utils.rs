// Removes the specified mask characters from a string.
//
// This function takes an input string `input` and a mask string `mask`,
// and returns a new string with characters from the mask removed.
//
// # Examples
//
// ```rust
// use dev_utilities::utils::string_utils::remove_mask;
//
// let result = remove_mask("123-456-789", "-");
// assert_eq!(result, "123456789");
//
// let result = remove_mask("(123) 456-7890", "()- ");
// assert_eq!(result, "1234567890");
// ```
pub fn remove_mask(input: &str, mask: &str) -> String {
    let mut result = String::with_capacity(input.len());
    for ch in input.chars() {
        if !mask.contains(ch) {
            result.push(ch);
        }
    }
    result
}
