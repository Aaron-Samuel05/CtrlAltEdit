//! OpenArt's source repository and issue tracker.

/// OpenArt's source repository.
pub const GITHUB: &str = "https://github.com/Aaron-Samuel05/OpenArt";
/// New issue on OpenArt's repository.
pub const ISSUES: &str = "https://github.com/Aaron-Samuel05/OpenArt/issues";

/// (command id, label, url) for every link, in menu order.
pub const ALL: [(&str, &str, &str); 2] = [
    ("help.github", "OpenArt on GitHub", GITHUB),
    ("help.reportIssue", "Report an Issue…", ISSUES),
];

/// The URL a `help.*` link command opens.
pub fn url_for(command: &str) -> Option<&'static str> {
    ALL.iter().find(|(id, _, _)| *id == command).map(|(_, _, u)| *u)
}

/// Open `url` in the system browser (a new tab on the web).
pub fn open(ctx: &egui::Context, url: &str) {
    ctx.open_url(egui::OpenUrl::new_tab(url));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn urls_point_to_the_openart_repository() {
        assert_eq!(GITHUB, "https://github.com/Aaron-Samuel05/OpenArt");
        assert_eq!(ISSUES, format!("{GITHUB}/issues"));
        assert!(ALL.iter().all(|(id, _, u)| id.starts_with("help.") && u.starts_with("https://")));
        assert_eq!(url_for("help.github"), Some(GITHUB));
        assert_eq!(url_for("help.nope"), None);
    }
}
