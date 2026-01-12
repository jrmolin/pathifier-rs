use std::collections::HashSet;

pub mod util;

/// Returns the platform-specific path delimiter
pub fn get_delimiter() -> &'static str {
    if cfg!(unix) {
        ":"
    } else {
        ";"
    }
}

/// Deduplicates a PATH-like string, preserving order and keeping first occurrences.
pub fn deduplicate_path(input: &str, delimiter: &str) -> String {
    let mut seen: HashSet<&str> = HashSet::new();
    let mut result: Vec<&str> = Vec::new();

    for part in input.split(delimiter) {
        if seen.insert(part) {
            result.push(part);
        }
    }

    result.join(delimiter)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_input() {
        assert_eq!(deduplicate_path("", ":"), "");
    }

    #[test]
    fn test_single_path() {
        assert_eq!(deduplicate_path("/usr/bin", ":"), "/usr/bin");
    }

    #[test]
    fn test_no_duplicates() {
        assert_eq!(
            deduplicate_path("/usr/bin:/bin:/usr/local/bin", ":"),
            "/usr/bin:/bin:/usr/local/bin"
        );
    }

    #[test]
    fn test_with_duplicates() {
        assert_eq!(
            deduplicate_path("/usr/bin:/bin:/usr/bin:/usr/local/bin:/bin", ":"),
            "/usr/bin:/bin:/usr/local/bin"
        );
    }

    #[test]
    fn test_all_duplicates() {
        assert_eq!(
            deduplicate_path("/usr/bin:/usr/bin:/usr/bin", ":"),
            "/usr/bin"
        );
    }

    #[test]
    fn test_preserves_order() {
        // First occurrence should be kept
        assert_eq!(
            deduplicate_path("/a:/b:/c:/b:/a:/d", ":"),
            "/a:/b:/c:/d"
        );
    }

    #[test]
    fn test_empty_segments() {
        // Empty segments between delimiters
        assert_eq!(
            deduplicate_path("/usr/bin::/bin::", ":"),
            "/usr/bin::/bin"
        );
    }

    #[test]
    fn test_windows_delimiter() {
        assert_eq!(
            deduplicate_path("C:\\Windows;C:\\Program Files;C:\\Windows", ";"),
            "C:\\Windows;C:\\Program Files"
        );
    }

    #[test]
    fn test_paths_with_spaces() {
        assert_eq!(
            deduplicate_path("/path with spaces:/another path:/path with spaces", ":"),
            "/path with spaces:/another path"
        );
    }

    #[test]
    fn test_unicode_paths() {
        assert_eq!(
            deduplicate_path("/home/用户:/usr/bin:/home/用户", ":"),
            "/home/用户:/usr/bin"
        );
    }

    #[test]
    fn test_get_delimiter() {
        let delim = get_delimiter();
        #[cfg(unix)]
        assert_eq!(delim, ":");
        #[cfg(windows)]
        assert_eq!(delim, ";");
    }
}
