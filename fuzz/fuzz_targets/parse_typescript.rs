use honggfuzz::fuzz;
use tree_sitter::Parser;

fn main() {
    loop {
        fuzz!(|data: &[u8]| {
            let code: &str = std::str::from_utf8(data).unwrap();
            let mut parser = Parser::new();
            parser.set_language(tree_sitter_typescript::language_typescript()).expect("Error loading typescript grammar");
            let _ = parser.parse(code, None);
        });
    }
}