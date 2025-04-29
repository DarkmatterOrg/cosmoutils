use notify_rust::Notification;

/// Send an error notification
///
/// ## Example
/// ```rust
/// error_notification("Hello, world", "foo", "firefox", 1200);
/// ```
///
pub fn error_notification(title: &str, content: &str, appname: &str, time: i32) {
    Notification::new().summary(title).body(content).appname(appname).icon("dialog-error").timeout(time).show().unwrap();
}

/// Send a warn notification
///
/// ## Example
/// ```rust
/// warn_notification("Hello, world", "foo", "firefox", 1200);
/// ```
///
pub fn warn_notification(title: &str, content: &str, appname: &str, time: i32) {
    Notification::new().summary(title).body(content).appname(appname).icon("dialog-warning").timeout(time).show().unwrap();
}

/// Send a info notification
///
/// ## Example
/// ```rust
/// info_notification("Hello, world", "foo", "firefox", 1200);
/// ```
///
pub fn info_notification(title: &str, content: &str, appname: &str, time: i32) {
    Notification::new().summary(title).body(content).appname(appname).icon("dialog-information").timeout(time).show().unwrap();
}

/// Send a success notification
///
/// ## Example
/// ```rust
/// success_notification("Hello, world", "foo", "firefox", 1200);
/// ```
///
pub fn success_notification(title: &str, content: &str, appname: &str, time: i32) {
    Notification::new().summary(title).body(content).appname(appname).icon("data-success").timeout(time).show().unwrap();
}