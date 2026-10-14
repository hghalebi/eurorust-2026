use std::{fmt, time::Duration};

#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum InputError {
    #[error(
        "tool name must begin with an ASCII letter and contain only letters, digits, hyphens or underscores"
    )]
    InvalidToolName,
    #[error("this tool-request contract requires a positive timeout")]
    ZeroTimeout,
}

pub struct ToolName(String);
impl TryFrom<String> for ToolName {
    type Error = InputError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        if !value
            .as_bytes()
            .first()
            .is_some_and(u8::is_ascii_alphabetic)
            || !value
                .bytes()
                .all(|c| c.is_ascii_alphanumeric() || c == b'-' || c == b'_')
        {
            return Err(InputError::InvalidToolName);
        }
        Ok(Self(value))
    }
}
impl fmt::Display for ToolName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

pub struct ToolTimeout(Duration);
impl TryFrom<u64> for ToolTimeout {
    type Error = InputError;
    fn try_from(milliseconds: u64) -> Result<Self, Self::Error> {
        if milliseconds == 0 {
            return Err(InputError::ZeroTimeout);
        }
        Ok(Self(Duration::from_millis(milliseconds)))
    }
}
impl fmt::Display for ToolTimeout {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}ms", self.0.as_millis())
    }
}

/// Narrow CLI/transport DTO: validation consumes it before domain use.
pub struct RawRequest {
    pub tool: String,
    pub timeout_ms: u64,
}
impl RawRequest {
    pub fn validate(self) -> Result<CheckedRequest, InputError> {
        Ok(CheckedRequest::new(
            self.tool.try_into()?,
            self.timeout_ms.try_into()?,
        ))
    }
}

/// Domain roles cannot be swapped, and representation fields cannot be fabricated.
/// ```
/// use agent_contract_lab::input::RawRequest;
/// let request=RawRequest{tool:"index-code".into(),timeout_ms:500}.validate().unwrap();
/// assert_eq!(request.to_string(), "tool=index-code timeout=500ms");
/// ```
/// ```compile_fail
/// use agent_contract_lab::input::{ToolName,ToolTimeout,CheckedRequest};
/// let tool=ToolName::try_from("index-code".to_owned()).unwrap();
/// let timeout=ToolTimeout::try_from(500).unwrap();
/// let request=CheckedRequest::new(timeout,tool);
/// ```
pub struct CheckedRequest {
    tool: ToolName,
    timeout: ToolTimeout,
}
impl CheckedRequest {
    pub fn new(tool: ToolName, timeout: ToolTimeout) -> Self {
        Self { tool, timeout }
    }
}
impl fmt::Display for CheckedRequest {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "tool={} timeout={}", self.tool, self.timeout)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn input_contract_preserves_exact_names_and_rejects_invalid_values() {
        assert_eq!(
            RawRequest {
                tool: " index".into(),
                timeout_ms: 1
            }
            .validate()
            .err(),
            Some(InputError::InvalidToolName)
        );
        assert_eq!(
            RawRequest {
                tool: "index".into(),
                timeout_ms: 0
            }
            .validate()
            .err(),
            Some(InputError::ZeroTimeout)
        );
        assert_eq!(
            RawRequest {
                tool: "Index_2".into(),
                timeout_ms: 1
            }
            .validate()
            .unwrap()
            .to_string(),
            "tool=Index_2 timeout=1ms"
        );
    }
}
