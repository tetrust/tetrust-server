use sha256::digest;

pub fn hash_password(password: String, salt: &String) -> String {
    digest(password + salt.as_str())
}
