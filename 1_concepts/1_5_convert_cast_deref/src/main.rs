use std::ops::Deref;
use rand::Rng;



#[derive(Debug)]
struct EmailString(String);

impl EmailString{
    pub fn new(email: &str) -> Result<Self, &'static str>{
        if email.is_empty() {
            return Err("Email cannot be empty");
        }
        let at_count = email.matches('@').count();
        if at_count != 1 {
            return Err("Email must contain exactly one @");
        }
        let parts: Vec<&str> = email.split('@').collect();
        let local_part = parts[0];
        let domain_part = parts[1];

        if local_part.is_empty() {
            return Err("Local part cannot be empty");
        }
        
        if domain_part.is_empty() {
            return Err("Domain part cannot be empty");
        }

        if !domain_part.contains('.') {
            return Err("Domain must contain at least one dot");
        }

        if domain_part.starts_with('.') || domain_part.ends_with('.') {
            return Err("Domain cannot start or end with a dot");
        }

        let domain_parts: Vec<&str> = domain_part.split('.').collect();
        if domain_parts.last().unwrap().is_empty() {
            return Err("Top-level domain cannot be empty");
        }

        Ok(EmailString(email.to_string()))
        
    }
}



struct Random<T>{
    data: [T;3],
}

impl<T> Random<T>{
    pub fn new(v1: T,v2: T,v3: T) -> Self{
        Random {
            data: [v1,v2,v3],
        }
    }
}

impl<T> Deref for Random<T>{
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.data[rand::rng().random_range(0..=2)]
    }
}
fn main() {
    let r = Random::new("hi", "you", "there");
    for _ in 0..20 {
    println!("{}", *r);
}
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_valid_emails() {
        assert!(EmailString::new("user@domain.com").is_ok());
        assert!(EmailString::new("test.email@example.org").is_ok());
        assert!(EmailString::new("user+tag@domain.co.uk").is_ok());
    }
    
    #[test]
    fn test_invalid_emails() {
        assert!(EmailString::new("").is_err());
        assert!(EmailString::new("user").is_err());
        assert!(EmailString::new("@domain.com").is_err());
        assert!(EmailString::new("user@").is_err());
        assert!(EmailString::new("user@@domain.com").is_err());
        assert!(EmailString::new("user@domain").is_err());
        assert!(EmailString::new("user@.domain.com").is_err());
        assert!(EmailString::new("user@domain.").is_err());
    }
}