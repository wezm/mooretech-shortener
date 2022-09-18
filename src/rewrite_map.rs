use url::Url;

// Used by build.rs
#[allow(unused)]
pub fn parse(txt: &str) -> Option<Vec<(&str, Url)>> {
    let mut mappings = Vec::new();
    for line in txt.lines() {
        if line.starts_with('#') {
            continue;
        }

        let (key, mut value) = line.split_once(' ')?;
        if let Some(idx) = value.find('#') {
            value = value[..idx].trim();
        }

        // Ensure destination URL is valid
        let url: Url = value.parse().expect("invalid URL");

        mappings.push((key, url));
    }
    Some(mappings)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_comment() {
        assert_eq!(parse("# Comment line").unwrap(), vec![]);
    }

    #[test]
    fn test_mapping() {
        assert_eq!(
            parse("test https://example.com/here").unwrap(),
            vec![("test", "https://example.com/here".parse().unwrap())]
        );
    }

    #[test]
    fn test_mapping_with_comment() {
        assert_eq!(
            parse("test https://example.com/here # comment").unwrap(),
            vec![("test", "https://example.com/here".parse().unwrap())]
        );
    }
}
