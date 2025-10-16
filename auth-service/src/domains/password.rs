#[derive(Clone, PartialEq)]
pub struct Password(String);

impl Password {
    pub fn parse(s: String) -> Result<Self, String> {
        if s.trim().is_empty() {
            return Err("Password cannot be empty".into());
        }

        if s.len() < 8 {
            return Err("Password should be more then 8 chars".into());
        }

        Ok(Self(s))
    }
}

impl AsRef<str> for Password {
    fn as_ref(&self) -> &str {
        self.0.as_str()
    }
}
