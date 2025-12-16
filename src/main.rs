mod http;

use http::parser::Parser;
fn main() {
    let p = Parser {
        tokens: vec!["foo".to_string(), "bar".to_string()],
    };
    
    
   println!("Hello {:?}, world!", p);
}
