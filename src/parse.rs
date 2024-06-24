use html_parser;

pub fn parse_to_json(res: &str) -> Result<String, html_parser::Error> {
    let res_html = html_parser::Dom::parse(res)?;
    let res_json = res_html.to_json()?;
    Ok(res_json)
}
