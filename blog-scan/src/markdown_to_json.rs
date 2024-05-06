use markdown::*;

pub fn parse_markdown_front_matter(file: String) -> Result<serde_yaml::Value, serde_yaml::Error> {
    let mut parse_option = ParseOptions::mdx();
    parse_option.constructs.frontmatter = true;
    let yaml_front_matter = 
        to_mdast(&file, &parse_option).expect(&file).children().unwrap().into_iter()
        .filter(|x| matches!(x, mdast::Node::Yaml(_)))
        .fold(String::new(), |x, y| x + &"\n" + &y.to_string());
    serde_yaml::from_str(&yaml_front_matter)
}
