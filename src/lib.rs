extern crate proc_macro;
extern crate regex;

use proc_macro::TokenStream;
use regex::Regex;

static mut V: Vec<String> = Vec::new();

#[proc_macro_attribute]
pub fn register_handler(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let regexp = Regex::new(r"(?:pub\s+)?(?:async)?\s+fn\s+(\w+)\s*\(").unwrap();
    let mut s = regexp
        .captures_iter(&item.to_string())
        .map(|capture| capture.get(1).unwrap().as_str().to_string())
        .collect::<Vec<String>>();

    unsafe {
        V.append(&mut s);
    }

    item
}

#[proc_macro_derive(ServiceConfigInitializer)]
pub fn derive_reg(_ts: TokenStream) -> TokenStream {
    let s = _ts.to_string();

    unsafe {
        let _rep = V
            .iter()
            .map(|a| format!(".service({})", a))
            .collect::<String>();
        V.clear();

        let rep = format!(
            r#"impl ServiceConfigInitializer for $1 {{ 
            fn register_handlers(cfg: &mut ServiceConfig) {{ 
                cfg{0};  
            }} 
        }}"#,
            _rep
        );

        let result = Regex::new(r"(?:pub\s+)?struct\s+(\w+).+")
            .unwrap()
            .replace(s.as_str(), rep.as_str())
            .to_string();

        result.parse().unwrap()
    }
}
