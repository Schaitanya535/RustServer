pub fn parser() -> tree_sitter::Parser {
    let mut parser = tree_sitter::Parser::new();
    let language = tree_sitter_sscript::LANGUAGE;
    parser
        .set_language(&language.into())
        .expect("Error loading Sscript parser");
    return parser;
}
