use notify_rust::Notification;

pub fn error_notification(title: &str, content: &str, appname: &str, time: i32) {
    Notification::new().summary(title).body(content).appname(appname).icon("dialog-error").timeout(time).show().unwrap();
}

pub fn warn_notification(title: &str, content: &str, appname: &str, time: i32) {
    Notification::new().summary(title).body(content).appname(appname).icon("dialog-warning").timeout(time).show().unwrap();
}

pub fn info_notification(title: &str, content: &str, appname: &str, time: i32) {
    Notification::new().summary(title).body(content).appname(appname).icon("dialog-information").timeout(time).show().unwrap();
}

pub fn success_notification(title: &str, content: &str, appname: &str, time: i32) {
    Notification::new().summary(title).body(content).appname(appname).icon("data-success").timeout(time).show().unwrap();
}