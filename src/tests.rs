
use crate::{
    builder::Builder,
    parser::Parser,
    statics::{SETUP_SEPARATOR, TOKEN},
    types::Token
};


fn help() -> (String, char){
    (SETUP_SEPARATOR.to_string(),TOKEN.chars().nth(0).expect("No 0th char in token"))
}

//
// TEST TOKENS
//

#[test]
fn test_parser(){
    let (sep, tok) = help();
    let mut p = Parser::new(&sep,tok);
    let text = format!("\
waf;10;20;130
{}
waf;more $isstuff$ie
narrator talking
$sb
$lb
$nl
$ns
waf;yet $bsanother$be thing
", sep);
    let tokens : Vec<Token> = vec!(
        Token::Init,
        Token::CharacterDef("waf".to_string(), "10".to_string(), "20".to_string(), "130".to_string()),
        Token::Begin,
        Token::ColorStart("waf".to_string()),
        Token::DialogueStart,
        Token::Text("more ".to_string()),
        Token::ItalicStart,
        Token::Text("stuff".to_string()),
        Token::ItalicStop,
        Token::DialogueStop,

        Token::NarratorStart,
        Token::Text("narrator talking".to_string()),
        Token::NarratorStop,

        Token::ShortBreak,
        Token::LongBreak,
        Token::NewLong,
        Token::NewShort,

        Token::ColorStart("waf".to_string()),
        Token::DialogueStart,
        Token::Text("yet ".to_string()),
        Token::BoldStart,
        Token::Text("another".to_string()),
        Token::BoldStop,
        Token::Text(" thing".to_string()),
        Token::DialogueStop,
        Token::End);
    let res = p.tokenize(text);
    assert_eq!(Ok(&tokens), res);
}


#[test]
fn test_parser_no_setup(){
    let (sep,tok) = help();
    let mut p = Parser::new(&sep,tok);
    let text = format!("\
stuff
more stuff", );
    let tokens : Vec<Token> = vec!(
        Token::Init,
        Token::Begin,
        Token::NarratorStart,
        Token::Text("stuff".to_string()),
        Token::NarratorStop,
        Token::NarratorStart,
        Token::Text("more stuff".to_string()),
        Token::NarratorStop,
        Token::End);

    let split = p.split(text.clone());
    let res = p.tokenize(text);

    assert_eq!("", split.0);
    assert_eq!("stuff\nmore stuff", split.1.trim());

    assert_eq!(Ok(&tokens), res);
}






//
// TEST BUILDER
//

#[test]
fn test_builder() {
    let builder = Builder::new();
    let tokens : Vec<Token> = vec!(
        Token::Init,
        Token::CharacterDef("be".to_string(), "199".to_string(), "230".to_string(), "1".to_string()),
        Token::Begin,
        Token::NarratorStart,
        Token::Text("narrator talking".to_string()),
        Token::NarratorStop,
        Token::ColorStart("be".to_string()),
        Token::DialogueStart,
        Token::Text("stuff".to_string()),
        Token::DialogueStop,
        Token::ShortBreak,
        Token::LongBreak,
        Token::NewLong,
        Token::NewShort,
        Token::DialogueStart,
        Token::Text("more stuff".to_string()),
        Token::DialogueStop,
        Token::End);
    let res = builder.build_stack(&tokens);
    let expected = Ok(String::from("\
\\documentclass{report}
\\usepackage{xcolor}
\\setlength\\parindent{0pt}
\\definecolor{be}{RGB}{199,230,1}
\\begin{document}
narrator talking

-``\\textcolor{be}{stuff}''

\\begin{center}\\noindent\\rule{8cm}{0.4pt}\\end{center}
\\begin{center}\\noindent\\rule{3cm}{0.4pt}\\end{center}
\\vspace{10mm}
\\vspace{5mm}
-``\\textcolor{be}{more stuff}''

\\end{document}"));

    assert_eq!(expected, res);
}





