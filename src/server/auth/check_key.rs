use bcrypt::verify;
use std::env;

/// Checks if the provided password matches the hashed key in the environment
pub fn verify_admin_key(input_password: &str) -> bool {
    // 1. Fetch the hash from the environment variable
    // We use .ok() to handle the case where the variable isn't set at all
    let stored_hash = match env::var("ADMIN_KEY") {
        Ok(hash) => hash,
        Err(_) => {
            eprintln!("Error: ADMIN_KEY environment variable is not set!");
            return false;
        }
    };
    
    // 2. Compare the raw input password with the stored hash
    // bcrypt::verify extracts the salt from the hash and does the math for you
    match verify(input_password, &stored_hash) {
        Ok(is_valid) => is_valid,
        Err(_) => {
            eprintln!("Error: Bcrypt verification failed (possibly a malformed hash)");
            false
        }
    }
}
