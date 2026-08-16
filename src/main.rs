use roxmltree as xml;

mod emitter;
mod parser;
mod semantic_analyzer;

fn main() {
    let source_code = std::fs::read_to_string("./main.crx").unwrap();
    let doc = match xml::Document::parse(&source_code) {
        Ok(i) => i,
        Err(err) => todo!("{:?}", err),
    };

    let root = doc.root_element();
    if !root.has_tag_name("cortex") {
        todo!(
            "expected root tag: 'cortex', found: {}",
            root.tag_name().name()
        );
    }

    let ast = parser::gen_ast(&doc, root);
    println!("{:#?}", ast);

    // semantic_analyzer::analyze(ast);
    emitter::write_basic(ast);
}
