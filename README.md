# Emacs Org Link Parser

This is a Rust library to parse links, formatted as [Emacs Org-mode hyperlinks](https://orgmode.org/guide/Hyperlinks.html), from a string.

## Example usage:

```
use emacs_org_link_parser as org;

fn main() {
    let line_to_parse = "*** [[#mycookbook][page 3]] dumplings, [[www.best-sauce.com][sauce here: number 4]] [[#pictures][how it looks]] [[forum.com]]";

    let parsed: Vec<org::Link> = org::parse_line(line_to_parse);

    println!("{:?}", parsed);
}
```
Expected output:
```
[Link { link: Some("#mycookbook"), description: Some("page 3") }, Link { link: Some("www.best-sauce.com"), description: Some("sauce here: number 4") }, Link { link: Some("#pictures"), description: Some("how it looks") }, Link { link: Some("forum.com"), description: None }]
```
