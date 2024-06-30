use html_parser;

// After retrieving the html file we convert it to json with the help of html_parser crate
pub fn parse_to_json(res: &str) -> Result<String, html_parser::Error> {
    let res_html = html_parser::Dom::parse(res)?;
    let res_json = res_html.to_json()?;
    Ok(res_json)
}
