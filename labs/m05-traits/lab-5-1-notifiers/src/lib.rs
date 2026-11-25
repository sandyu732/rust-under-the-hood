//! Lab 5.1 — Notifier plugins: dynamic vs static dispatch.

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NotifyError {
    InvalidTarget(String),
    Rejected(String),
}

pub trait Notifier {
    fn name(&self) -> &str;
    fn send(&mut self, msg: &str) -> Result<(), NotifyError>;

    /// Default method: send each message, stopping at the first error.
    /// Implementors get this for free.
    fn send_many(&mut self, msgs: &[&str]) -> Result<(), NotifyError> {
        // SOLUTION-START
        for m in msgs {
            self.send(m)?;
        }
        Ok(())
        // SOLUTION-END
    }
}

/// Prints and remembers every message.
#[derive(Default)]
pub struct Console {
    pub sent: Vec<String>,
}

/// Formats as "To: <to>\n<msg>". `to` must contain '@', else InvalidTarget.
pub struct Email {
    pub to: String,
    pub outbox: Vec<String>,
}

/// Only `https://` URLs are allowed (else InvalidTarget). Empty messages are
/// Rejected("empty message").
pub struct Webhook {
    pub url: String,
    pub delivered: Vec<String>,
}

impl Notifier for Console {
    fn name(&self) -> &str {
        "console"
    }
    fn send(&mut self, msg: &str) -> Result<(), NotifyError> {
        // SOLUTION-START
        println!("[console] {msg}");
        self.sent.push(msg.to_string());
        Ok(())
        // SOLUTION-END
    }
}

impl Notifier for Email {
    fn name(&self) -> &str {
        "email"
    }
    fn send(&mut self, msg: &str) -> Result<(), NotifyError> {
        // SOLUTION-START
        if !self.to.contains('@') {
            return Err(NotifyError::InvalidTarget(self.to.clone()));
        }
        self.outbox.push(format!("To: {}\n{msg}", self.to));
        Ok(())
        // SOLUTION-END
    }
}

impl Notifier for Webhook {
    fn name(&self) -> &str {
        "webhook"
    }
    fn send(&mut self, msg: &str) -> Result<(), NotifyError> {
        // SOLUTION-START
        if !self.url.starts_with("https://") {
            return Err(NotifyError::InvalidTarget(self.url.clone()));
        }
        if msg.is_empty() {
            return Err(NotifyError::Rejected("empty message".into()));
        }
        self.delivered.push(msg.to_string());
        Ok(())
        // SOLUTION-END
    }
}

/// Dynamic dispatch: a mixed collection of notifiers behind `Box<dyn Notifier>`.
/// Returns each notifier's name and its result, in order.
pub fn broadcast(
    notifiers: &mut [Box<dyn Notifier>],
    msg: &str,
) -> Vec<(String, Result<(), NotifyError>)> {
    // SOLUTION-START
    notifiers
        .iter_mut()
        .map(|n| (n.name().to_string(), n.send(msg)))
        .collect()
    // SOLUTION-END
}

/// Static dispatch: every element is the SAME concrete type `N`.
/// Returns how many sends succeeded. (Try passing a mix — it won't compile.)
pub fn send_all<N: Notifier>(notifiers: &mut [N], msg: &str) -> usize {
    // SOLUTION-START
    notifiers
        .iter_mut()
        .filter_map(|n| n.send(msg).ok())
        .count()
    // SOLUTION-END
}
