type Version = [u32; 3];

pub fn is_forced(local: &str, manifest: &str) -> bool {
    match (parse_version(local), min_version(manifest)) {
        (Some(local), Some(min_version)) => local < min_version,
        _ => false,
    }
}

fn min_version(manifest: &str) -> Option<Version> {
    let manifest: toml::Table = manifest.parse().ok()?;
    let min_version = manifest
        .get("package")?
        .get("metadata")?
        .get("weathr")?
        .get("min_version")?
        .as_str()?;
    parse_version(min_version)
}

fn parse_version(version: &str) -> Option<Version> {
    let mut parts = version.trim().split('.').map(|part| part.parse().ok());
    let version = [parts.next()??, parts.next()??, parts.next()??];
    parts.next().is_none().then_some(version)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn manifest(metadata: &str) -> String {
        format!("[package]\nname = \"weathr\"\nversion = \"1.8.0\"\n\n{metadata}")
    }

    #[test]
    fn test_min_version_is_read_from_metadata() {
        let manifest = manifest("[package.metadata.weathr]\nmin_version = \"1.9.2\"\n");
        assert_eq!(min_version(&manifest), Some([1, 9, 2]));
    }

    #[test]
    fn test_min_version_absent_section_is_none() {
        assert_eq!(min_version(&manifest("")), None);
    }

    #[test]
    fn test_min_version_absent_key_is_none() {
        let manifest = manifest("[package.metadata.weathr]\nother = \"1.9.0\"\n");
        assert_eq!(min_version(&manifest), None);
    }

    #[test]
    fn test_min_version_garbage_value_is_none() {
        let manifest = manifest("[package.metadata.weathr]\nmin_version = \"abc\"\n");
        assert_eq!(min_version(&manifest), None);
    }

    #[test]
    fn test_min_version_non_string_value_is_none() {
        let manifest = manifest("[package.metadata.weathr]\nmin_version = 2\n");
        assert_eq!(min_version(&manifest), None);
    }

    #[test]
    fn test_min_version_invalid_toml_is_none() {
        assert_eq!(min_version("[package\nmin_version = "), None);
    }

    fn forced_with(local: &str, min_version: &str) -> bool {
        let manifest = manifest(&format!(
            "[package.metadata.weathr]\nmin_version = \"{min_version}\"\n"
        ));
        is_forced(local, &manifest)
    }

    #[test]
    fn test_older_local_version_is_forced() {
        assert!(forced_with("1.8.0", "1.9.0"));
        assert!(forced_with("1.9.9", "1.10.0"));
        assert!(forced_with("0.12.3", "1.0.0"));
    }

    #[test]
    fn test_equal_local_version_is_not_forced() {
        assert!(!forced_with("1.9.0", "1.9.0"));
    }

    #[test]
    fn test_newer_local_version_is_not_forced() {
        assert!(!forced_with("1.10.0", "1.9.5"));
        assert!(!forced_with("2.0.0", "1.99.99"));
    }

    #[test]
    fn test_other_lengths_are_not_forced() {
        assert!(!forced_with("1.8.0", "1.9"));
        assert!(!forced_with("1.8.0", "1.9.0.1"));
        assert!(!forced_with("1.8", "1.9.0"));
    }

    #[test]
    fn test_garbage_versions_are_not_forced() {
        assert!(!forced_with("1.8.0", "1.x.0"));
        assert!(!forced_with("1.8.0-beta", "1.9.0"));
        assert!(!forced_with("1.8.0", ""));
    }

    #[test]
    fn test_missing_min_version_is_not_forced() {
        assert!(!is_forced("1.8.0", &manifest("")));
    }
}
