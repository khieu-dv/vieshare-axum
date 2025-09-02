use regex::Regex;
use validator::ValidationError;

pub fn validate_full_name(name: &str) -> Result<(), ValidationError> {
    // Remove extra spaces
    let space_regex = Regex::new(r"\s+").unwrap();
    let cleaned_name = space_regex.replace_all(name, " ");
    
    // Remove trailing spaces
    let trimmed_name = cleaned_name.trim();
    
    // To support all possible languages - exclude special characters and numbers
    let name_regex = Regex::new(r"^[^±!@£$%^&*_+§¡€#¢§¶•ªº«\\/<>?:;'|=.,0123456789]{3,20}$").unwrap();
    
    if name_regex.is_match(trimmed_name) {
        Ok(())
    } else {
        Err(ValidationError::new("invalid_full_name"))
    }
}