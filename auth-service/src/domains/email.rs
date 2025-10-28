#[derive(Clone, PartialEq, Hash, Eq, Debug)]
pub struct Email(String);

impl Email {
    pub fn parse(s: String) -> Result<Self, String> {
        if s.trim().is_empty() {
            return Err("Email cannot be empty".into());
        }

        if !s.contains("@") {
            return Err("Email must contain '@' symbol".into());
        }

        Ok(Self(s))
    }
}

impl AsRef<str> for Email {
    fn as_ref(&self) -> &str {
        self.0.as_str()
    }
}
