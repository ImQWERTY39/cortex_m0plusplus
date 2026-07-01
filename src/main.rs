use roxmltree as xml;

mod emitter;
mod parser;

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

    let ast = parser::gen_ast(root);
    println!("{:#?}", ast);

    emitter::write_basic(ast);
}
