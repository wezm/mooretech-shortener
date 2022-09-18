pub fn parse(txt: &str) -> Option<Vec<(&str, &str)>> {
    let mut mappings = Vec::new();
    for line in txt.lines() {
        if line.starts_with('#') {
            continue;
        }

        let (key, mut value) = line.split_once(' ')?;
        if let Some(idx) = value.find('#') {
            value = value[..idx].trim();
        }
        mappings.push((key, value));
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
        assert_eq!(parse("test /here").unwrap(), vec![("test", "/here")]);
    }

    #[test]
    fn test_mapping_with_comment() {
        assert_eq!(
            parse("test /here # comment").unwrap(),
            vec![("test", "/here")]
        );
    }
}
