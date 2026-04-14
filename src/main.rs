use std::collections::HashMap;
use std::env;
use std::fs;
use std::path::Path;
use std::process::Command;

/// Compiler Trait
pub trait Compiler {
    fn compile(&mut self, source: &str);
    fn next_token(&mut self) -> String;
    fn parse(&mut self);
    fn current_token(&self) -> String;
    fn set_current_token(&mut self, tok: String);
}

/// Lexical Analysis Trait
pub trait LexicalAnalyzer {
    fn get_char(&mut self) -> char;
    fn add_char(&mut self, c: char);
    fn lookup(&self, s: &str) -> bool;
}

/// Syntax Analysis Trait (Option 1)
pub trait SyntaxAnalyzer {
    fn parse_lolcode(&mut self);        
    fn parse_head(&mut self);           
    fn parse_title(&mut self);         
    fn parse_comment(&mut self);        
    fn parse_body(&mut self);           
    fn parse_paragraph(&mut self);      
    fn parse_inner_paragraph(&mut self);
    fn parse_inner_text(&mut self);     
    fn parse_variable_define(&mut self);
    fn parse_variable_use(&mut self);   
    fn parse_bold(&mut self);           
    fn parse_italics(&mut self);        
    fn parse_list(&mut self);           
    fn parse_list_items(&mut self);     
    fn parse_inner_list(&mut self);     
    fn parse_link(&mut self);          
    fn parse_newline(&mut self);        
    fn parse_text(&mut self);           
}


/// Character by character Lexical Analyzer
/// Lexical Analyzer structure and implementaiton
pub struct SimpleLexicalAnalyzer {
    input: Vec<char>,
    pos: usize,
    current_build: String,
    pub tokens: Vec<String>,
}

impl SimpleLexicalAnalyzer {
    pub fn new(source: &str) -> Self {
        Self {
            input: source.chars().collect(),
            pos: 0,
            current_build: String::new(),
            tokens: Vec::new(),
        }
    }

    /// Converts characters into tokens then adds to abstract parse tree
    pub fn tokenize(&mut self) {
        loop {
            let c = self.get_char();
            if c == '\0' {
                break;
            }

            if c.is_whitespace() {
                if !self.current_build.is_empty() {
                    self.tokens.push(self.current_build.clone());
                    self.current_build.clear();
                }
            } else {
                self.add_char(c);
            }
        }

        if !self.current_build.is_empty() {
            self.tokens.push(self.current_build.clone());
        }

        self.tokens.reverse();
    }
}

/// Implementation & get_char method (returns next character)
impl LexicalAnalyzer for SimpleLexicalAnalyzer {
    fn get_char(&mut self) -> char {
        if self.pos < self.input.len() {
            let c = self.input[self.pos];
            self.pos += 1;
            c
        } else {
            '\0'
        }
    }

    /// Adds the character to the current token
    fn add_char(&mut self, c: char) {
        self.current_build.push(c);
    }

    /// Checks if token matches a keyword
    fn lookup(&self, s: &str) -> bool {
        let upper = s.to_uppercase();

        matches!(
            upper.as_str(),
            "#HAI" | "#KBYE" | "#OBTW" | "#TLDR" | "#MAEK" | "#GIMMEH" | "#MKAY"
                | "#IHAZ" | "#ITIZ" | "#LEMMESEE" | "#OIC"
                | "HEAD" | "TITLE" | "PARAGRAF" | "BOLD" | "ITALICS"
                | "LIST" | "ITEM" | "NEWLINE" | "LINX"
        )
    }
}

/// Recursive Decent Parser/Syntax Analyzer
pub struct LolcodeCompiler {
    lexer: SimpleLexicalAnalyzer,
    current_tok: String,
    html: String,
    scopes: Vec<HashMap<String, String>>,
}

/// Implementation
impl LolcodeCompiler {
    pub fn new() -> Self {
        Self {
            lexer: SimpleLexicalAnalyzer::new(""),
            current_tok: String::new(),
            html: String::new(),
            scopes: vec![HashMap::new()],
        }
    }

    /// Loads First Token
    fn start(&mut self) {
        let tok = self.lexer.tokens.pop().unwrap_or_default();
        self.current_tok = tok;
    }

    /// Moves to the next token
    fn advance(&mut self) {
        self.current_tok = self.lexer.tokens.pop().unwrap_or_default();
    }

    /// Ensures the token matches the string input
    fn expect(&mut self, expected: &str) {
        if self.current_tok.to_uppercase() == expected.to_uppercase() {
            self.advance();
        } else {
            eprintln!("Syntax error: expected {}, found {}", expected, self.current_tok);
            std::process::exit(1);
        }
    }


    /// Static Scope 
    /// Creates new scope
    fn push_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }

    /// Removes most recent scope 
    fn pop_scope(&mut self) {
        self.scopes.pop();
    }

    /// Stores a variable in the current scope
    fn define_var(&mut self, name: String, value: String) {
        if let Some(scope) = self.scopes.last_mut() {
            scope.insert(name, value);
        }
    }

    /// Retrieves varaible from the mos recent scope
    fn lookup_var(&self, name: &str) -> String {
        for scope in self.scopes.iter().rev() {
            if let Some(v) = scope.get(name) {
                return v.clone();
            }
        }
        eprintln!("Static semantic error: variable '{}' not defined", name);
        std::process::exit(1);
    }

    /// Appends HTML keywords and texts to the output
    fn append(&mut self, s: &str) {
        self.html.push_str(s);
    }

    /// Parser Start & Semantic Analyzer 
    /// #HAI comments? head body #KBYE
    fn parse_lolcode(&mut self) {
        self.append("<html>\n");

        self.expect("#HAI");

        self.parse_comment();
        self.parse_head();
        self.parse_body();

        self.expect("#KBYE");

        self.append("</html>\n");    
    }

    /// #MAEK HEAD title #OIC
    fn parse_head(&mut self) {
        if self.current_tok.to_uppercase() == "#MAEK" {
            self.advance();
            if self.current_tok.to_uppercase() != "HEAD" {
                eprintln!("Syntax error: expected HEAD");
                std::process::exit(1);
            }
            self.advance();

            self.append("<head>\n");
            self.parse_title();
            self.expect("#MKAY");
            self.append("</head>\n");
        }
    }

    /// #GIMMEH TITLE text #OIC
    fn parse_title(&mut self) {
        self.expect("#GIMMEH");
        if self.current_tok.to_uppercase() != "TITLE" {
            eprintln!("Syntax error: expected TITLE");
            std::process::exit(1);
        }
        self.advance();

        let mut text = String::new();

        while self.current_tok.to_uppercase() != "#OIC" {
            text.push_str(&self.current_tok);
            text.push(' ');
            self.advance();

            if self.current_tok.is_empty() {
                eprintln!("Syntax error: TITLE missing #OIC terminator");
                std::process::exit(1);
            }
        }

        self.expect("#OIC");

        self.append(&format!("<title>{}</title>\n", text));
    }

    /// #OBTW text #TLDR
   fn parse_comment(&mut self) {
        while self.current_tok.to_uppercase() == "#OBTW" {
            self.advance();

            let mut text = String::new();

            while self.current_tok.to_uppercase() != "#TLDR" {
                text.push_str(&self.current_tok);
                text.push(' ');
                self.advance();

                if self.current_tok.is_empty() {
                    eprintln!("Syntax error: comment missing #TLDR terminator");
                    std::process::exit(1);
                }
            }

            self.expect("#TLDR");
            self.append(&format!("<!-- {} -->\n", text));
        }
   }

   /// (paragraph | comment | bold | italics | list | link | 
   /// newline | variable define | variable use | text)*
    fn parse_body(&mut self) {
        while !self.current_tok.is_empty()
            && self.current_tok.to_uppercase() != "#KBYE"
        {
            match self.current_tok.to_uppercase().as_str() {
                "#OBTW" => self.parse_comment(),
                "#MAEK" => {
                    let next = self.lexer.tokens.last().unwrap_or(&"".into()).to_uppercase();
                    if next == "PARAGRAF" {
                        self.parse_paragraph();
                    } else if next == "LIST" {
                        self.parse_list();
                    } else {
                        eprintln!("Syntax error: unexpected token '{}' after #MAEK", next);
                        std::process::exit(1);
                    }
                }
                "#GIMMEH" => {
                    let next = self.lexer.tokens.last().unwrap_or(&"".into()).to_uppercase();

                    if next == "BOLD" {
                        self.parse_bold();
                    } else if next == "ITALICS" {
                        self.parse_italics();
                    } else if next == "LINX" {
                        self.parse_link();
                    } else if next == "NEWLINE" {
                        self.parse_newline();
                    } else {
                        eprintln!("Syntax error: unexpected token '{}' after #GIMMEH", next);
                        std::process::exit(1);
                    }

                }
                "#IHAZ" => self.parse_variable_define(),
                "#LEMMESEE" => self.parse_variable_use(),
                _ => self.parse_text(),
            }

            
        }

    }

    /// #MAEK PARAGRAF variable define? inner paragraph #MKAY
    fn parse_paragraph(&mut self) {
        self.expect("#MAEK");
        self.expect("PARAGRAF");

        self.append("<p>");
        self.push_scope();

        if self.current_tok.to_uppercase() == "#IHAZ" {
            self.parse_variable_define();
        }

        self.parse_inner_paragraph();

        self.expect("#MKAY");
        self.pop_scope();
        self.append("</p>\n");
    }

    fn parse_inner_paragraph(&mut self) {
        while !self.current_tok.is_empty()
            && self.current_tok.to_uppercase() != "#MKAY"
        {
            self.parse_inner_text();
        }
    }

    /// (variable use | bold | italics | list | link | newline | text)*
    fn parse_inner_text(&mut self) {
        match self.current_tok.to_uppercase().as_str() {
            "#GIMMEH" => {
                let next = self.lexer.tokens.last().unwrap_or(&"".into()).to_uppercase();
                if next == "BOLD" {
                    self.parse_bold();
                } else if next == "ITALICS" {
                    self.parse_italics();
                } else if next == "LINX" {
                    self.parse_link();
                } else if next == "NEWLINE" {
                    self.parse_newline();
                } else {
                    eprintln!("Syntax error: unexpected token '{}' after #GIMMEH", next);
                    std::process::exit(1);
                }
           }
            "#MAEK" => {
                    let next = self.lexer.tokens.last().unwrap_or(&"".into()).to_uppercase();
                    if next == "LIST" {
                        self.parse_list();
                    } else {
                        eprintln!("Syntax error: unexpected token '{}' after #MAEK", next);
                        std::process::exit(1);
                    }
                }
            "#LEMMESEE" => self.parse_variable_use(),
            "#IHAZ" => self.parse_variable_define(),
            _ => self.parse_text(),
        }
    }

    /// #IHAZ varname #ITIZ value #MKAY
    fn parse_variable_define(&mut self) {
        self.expect("#IHAZ");

        let name = self.current_tok.clone();
        self.advance();

        self.expect("#ITIZ");

        let value = self.current_tok.clone();
        self.advance();

        self.expect("#MKAY");

        self.define_var(name, value);
    }

    /// #LEMMESEE varname #OIC
    fn parse_variable_use(&mut self) {
        self.expect("#LEMMESEE");

        let name = self.current_tok.clone();
        self.advance();

        self.expect("#OIC");

        let value = self.lookup_var(&name);
        self.append(&(value + " "));
    }

    /// #GIMMEH BOLD text #OIC
    fn parse_bold(&mut self) {
        self.expect("#GIMMEH");
        self.expect("BOLD");

        let mut text = String::new();

        while self.current_tok.to_uppercase() != "#OIC" {
            text.push_str(&self.current_tok);
            text.push(' ');
            self.advance();

            if self.current_tok.is_empty() {
                eprintln!("Syntax error: TITLE missing #OIC terminator");
                std::process::exit(1);
            }
        }

        self.expect("#OIC");

        self.append(&format!("<b>{}</b>", text));
    }

    /// #GIMMEH ITALICS text #OIC
    fn parse_italics(&mut self) {
        self.expect("#GIMMEH");
        self.expect("ITALICS");

        let mut text = String::new();

        while self.current_tok.to_uppercase() != "#OIC" {
            text.push_str(&self.current_tok);
            text.push(' ');
            self.advance();

            if self.current_tok.is_empty() {
                eprintln!("Syntax error: TITLE missing #OIC terminator");
                std::process::exit(1);
            }
        }

        self.expect("#OIC");

        self.append(&format!("<i>{}</i>", text));
    }

    /// #MAEK LIST list items #MKAY
    fn parse_list(&mut self) {
        self.expect("#MAEK");
        self.expect("LIST");

        self.append("<ul>\n");

        self.parse_list_items();

        self.expect("#MKAY");
        self.append("</ul>\n");
    }

    /// #GIMMEH ITEM inner list #OIC
    fn parse_list_items(&mut self) {
        while self.current_tok.to_uppercase() == "#GIMMEH" {
            let next = self.lexer.tokens.last().unwrap_or(&"".into()).to_uppercase();
            if next == "ITEM" {
                self.advance();
                self.expect("ITEM");

                self.append("<li>");

                while !self.current_tok.is_empty() && self.current_tok.to_uppercase() != "#OIC" {
                    self.parse_inner_list();
                }

                self.expect("#OIC");
                self.append("</li>\n");

            } else {
                break;
            }
        }
    }

    /// (bold | italics | link | text | variable use)*
    fn parse_inner_list(&mut self) {
        match self.current_tok.to_uppercase().as_str() {
            "#GIMMEH" => {
                let next = self.lexer.tokens.last().unwrap_or(&"".into()).to_uppercase();
                if next == "BOLD" {
                    self.parse_bold();
                } else if next == "ITALICS" {
                    self.parse_italics();
                } else if next == "LINX" {
                    self.parse_link();
                } else if next == "NEWLINE" {
                    self.parse_newline();
                } else {
                    eprintln!("Syntax error: unexpected token '{}' after #GIMMEH", next);
                    std::process::exit(1);
                }
           }
            "#LEMMESEE" => self.parse_variable_use(),
            _ => self.parse_text(),
        }
    }

    /// #GIMMEH LINX address #OIC
    fn parse_link(&mut self) {
        
        self.expect("#GIMMEH");
        self.expect("LINX");
        
        let addr = self.current_tok.clone();
        self.advance();

        self.expect("#OIC");

        self.append(&format!("<a href=\"{}\">{}</a>", addr, addr));
    }

    /// #GIMMEH NEWLINE
    fn parse_newline(&mut self) {
        self.expect("#GIMMEH");
        self.expect("NEWLINE");
        self.append("<br>\n");
    }

    /// Non token words here
    fn parse_text(&mut self) {
        if self.current_tok.starts_with("#") {
            eprintln!("Syntax error: unexpected keyword '{}'", self.current_tok);
            std::process::exit(1);
        }
        let t = self.current_tok.clone();
        self.advance();
        self.append(&format!("{} ", t));
    }
}


/// Compiler Implementation
impl Compiler for LolcodeCompiler {
    fn compile(&mut self, source: &str) {
        self.lexer = SimpleLexicalAnalyzer::new(source);
        self.lexer.tokenize();
        self.start();
    }

    fn next_token(&mut self) -> String {
        self.advance();
        self.current_tok.clone()
    }

    fn parse(&mut self) {
        self.parse_lolcode();
    }

    fn current_token(&self) -> String {
        self.current_tok.clone()
    }

    fn set_current_token(&mut self, tok: String) {
        self.current_tok = tok;
    }
}

/// Opens html file in default browser (tested in windows)
fn open_in_browser(file: &str) {
    let path = Path::new(file).canonicalize().expect("File not found");

    #[cfg(target_os = "macos")]
    Command::new("open").arg(path).spawn().expect("failed to open browser");

    #[cfg(target_os = "linux")]
    Command::new("xdg-open").arg(path).spawn().expect("failed to open browser");

    /// Tested and works in Windows
    #[cfg(target_os = "windows")]
    Command::new("cmd")
        .args(["/C", "start", path.to_str().unwrap()])
        .spawn()
        .expect("failed to open browser");
}

/// Main function
fn main() {
    /// Checks if file input is .lol file
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: lolcompiler <file.lol>");
        std::process::exit(1);
    }

    let filename = &args[1];

    if !filename.ends_with(".lol") {
        eprintln!("Error: file must end with .lol");
        std::process::exit(1);
    }

    let source = fs::read_to_string(filename).unwrap_or_else(|err| {
        eprintln!("Error reading {}: {}", filename, err);
        std::process::exit(1);
    });

    /// Compiles code
    let mut compiler = LolcodeCompiler::new();
    compiler.compile(&source);
    compiler.parse();

    /// Replaces lol code with html
    let output = filename.replace(".lol", ".html");
    fs::write(&output, compiler.html).expect("Failed to write HTML");

    open_in_browser(&output);
}
