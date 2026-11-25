#![allow(unused)] // TODO: delete this line once the lab is done
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
        todo!("lab-5-1-notifiers: implement me")
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
        todo!("lab-5-1-notifiers: implement me")
    }
}

impl Notifier for Email {
    fn name(&self) -> &str {
        "email"
    }
    fn send(&mut self, msg: &str) -> Result<(), NotifyError> {
        todo!("lab-5-1-notifiers: implement me")
    }
}

impl Notifier for Webhook {
    fn name(&self) -> &str {
        "webhook"
    }
    fn send(&mut self, msg: &str) -> Result<(), NotifyError> {
        todo!("lab-5-1-notifiers: implement me")
    }
}

/// Dynamic dispatch: a mixed collection of notifiers behind `Box<dyn Notifier>`.
/// Returns each notifier's name and its result, in order.
pub fn broadcast(
    notifiers: &mut [Box<dyn Notifier>],
    msg: &str,
) -> Vec<(String, Result<(), NotifyError>)> {
    todo!("lab-5-1-notifiers: implement me")
}

/// Static dispatch: every element is the SAME concrete type `N`.
/// Returns how many sends succeeded. (Try passing a mix — it won't compile.)
pub fn send_all<N: Notifier>(notifiers: &mut [N], msg: &str) -> usize {
    todo!("lab-5-1-notifiers: implement me")
}
