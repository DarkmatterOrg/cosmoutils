use notify_rust::Notification;

/// Send an error notification
///
/// ## Example
/// ```rust
/// use cosmoutils::modules::notifications::error_notification;
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
/// use cosmoutils::modules::notifications::warn_notification;
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
/// use cosmoutils::modules::notifications::info_notification;
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
/// use cosmoutils::modules::notifications::success_notification;
/// success_notification("Hello, world", "foo", "firefox", 1200);
/// ```
///
pub fn success_notification(title: &str, content: &str, appname: &str, time: i32) {
    Notification::new().summary(title).body(content).appname(appname).icon("data-success").timeout(time).show().unwrap();
}