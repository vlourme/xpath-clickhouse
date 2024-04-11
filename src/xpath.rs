use sxd_document::parser;
use sxd_xpath::evaluate_xpath;
use sxd_xpath::Value;

/// Parse input data into two parts (xml, xpath).
fn parse_input(data: &str) -> Option<(&str, &str)> {
    let parts: Vec<&str> = data.splitn(2, '\t').collect();
    if parts.len() != 2 {
        return None;
    }

    Some((parts[0], parts[1]))
}

/// Evaluate XPATH expression and return the first result.
pub fn xpath(data: &str) -> Option<String> {
    let (xml, path) = parse_input(data)?;

    let Ok(package) = parser::parse(xml) else {
        return None;
    };
    let document = package.as_document();

    let context = evaluate_xpath(&document, path).ok()?;

    Some(context.string())
}

/// Evaluate XPATH expression and return all results as an array.
pub fn xpath_to_array(data: &str) -> Option<Vec<String>> {
    let (xml, path) = parse_input(data)?;

    let Ok(package) = parser::parse(xml) else {
        return None;
    };
    let document = package.as_document();

    let context = evaluate_xpath(&document, path).ok()?;
    let nodeset = match context {
        Value::Nodeset(nodeset) => nodeset,
        _ => return None,
    };

    let mut result = Vec::new();
    for node in nodeset.document_order() {
        result.push(node.string_value());
    }

    Some(result)
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_xpath() {
        let data = r#"
            <root>
                <element>value</element>
            </root>
        "#;
        let path = "/root/element";
        let result = xpath(&format!("{}\t{}", data, path));

        assert_eq!(result, Some("value".to_string()));
    }

    #[test]
    fn test_xpath_multiple() {
        let data = r#"
            <root>
                <element>value1</element>
                <element>value2</element>
                <element>value3</element>
            </root>
        "#;
        let path = "/root/element";
        let result = xpath(&format!("{}\t{}", data, path));

        assert_eq!(result, Some("value1".to_string()));
    }

    #[test]
    fn test_xpath_multiple_array() {
        let data = r#"
            <root>
                <element>value1</element>
                <element>value2</element>
                <element>value3</element>
            </root>
        "#;
        let path = "/root/element";
        let result = xpath_to_array(&format!("{}\t{}", data, path));

        assert_eq!(
            result,
            Some(vec![
                "value1".to_string(),
                "value2".to_string(),
                "value3".to_string()
            ])
        );
    }
}
