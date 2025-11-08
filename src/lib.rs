pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    let query = query.to_uppercase();

    for line in contents.lines() {
        if line.to_uppercase().contains(&query) {
            results.push(line);
        }
    }

    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "the";
        let contents = "\
The quick brown fox
jumped over
the lazy dog";

        assert_eq!(vec!["the lazy dog"], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "the";
        let contents = "\
The quick brown fox
jumped over
the lazy dog";

        assert_eq!(
            vec!["The quick brown fox", "the lazy dog"],
            search_case_insensitive(query, contents)
        );
    }
}
