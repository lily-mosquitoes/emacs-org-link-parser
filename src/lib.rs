#[derive(Debug, PartialEq, Eq)]
pub struct Link {
    pub link: Option<String>,
    pub description: Option<String>,
}

pub fn parse_line(line: &str) -> Vec<Link> {
    let mut link_begin: Option<usize> = None;
    let mut link_end: Option<usize> = None;

    let mut line_links: Vec<Link> = Vec::new();

    for (index, character) in line.chars().enumerate() {
        if character == '[' && line.chars().nth(index-1) == Some('[') {
            link_begin = Some(index+1);
        } else if character == ']' && line.chars().nth(index-1) == Some(']') {
            link_end = Some(index-1);
        }

        if let Some(begin) = link_begin {
            if let Some(end) = link_end {
                let text: Vec<String> = line[begin..end].to_string()
                    .split("][")
                    .map(|x| x.to_string())
                    .collect();
                let mut text = text.into_iter();

                line_links.push(Link {
                    link: text.next(),
                    description: text.next(),
                });
                link_begin = None;
                link_end = None;
            }
        }
    }

    line_links
}

#[cfg(test)]
mod tests {
    use super::{Link, parse_line};

    #[test]
    fn correctly_parsed() {
        let line_to_parse = "*** [[#examp_3le][p3]] dumplings, [[www.best-sauce.com][p34]] [[#anothr][pretty54]] [[alonelink]]";

        let expected_output: Vec<Link> = vec![Link { link: Some("#examp_3le".to_string()), description: Some("p3".to_string()) }, Link { link: Some("www.best-sauce.com".to_string()), description: Some("p34".to_string()) }, Link { link: Some("#anothr".to_string()), description: Some("pretty54".to_string()) }, Link { link: Some("alonelink".to_string()), description: None }];

        assert_eq!(parse_line(line_to_parse), expected_output);
    }
}
