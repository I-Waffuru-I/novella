
use crate::{
    builder::Builder, parser::{Parser,SETUP_SEPARATOR}, types::Token
};

//
// TEST TOKENS
//
#[test]
fn test_parser_characters(){
    let mut p = Parser::new();
    let text = format!("\
waf;10;20;130
be;199;230;1
{}
stuff
more stuff
", SETUP_SEPARATOR);
    let tokens : Vec<Token> = vec!(
        Token::Init,
        Token::CharacterDef("waf".to_string(), "10".to_string(), "20".to_string(), "130".to_string()),
        Token::CharacterDef("be".to_string(), "199".to_string(), "230".to_string(), "1".to_string()),
        Token::Begin,
        Token::NarratorStart,
        Token::Text("stuff".to_string()),
        Token::NarratorStop,
        Token::NarratorStart,
        Token::Text("more stuff".to_string()),
        Token::NarratorStop,
        Token::End);
    let res = p.tokenize(text);
    dbg!(&res);
    assert_eq!(Ok(&tokens), res);
}

#[test]
fn test_parser_dialogue(){
    let mut p = Parser::new();
    let text = format!("\
waf;10;20;130
{}
stuff
waf;more stuff
waf;yet another thing
", SETUP_SEPARATOR);
    let tokens : Vec<Token> = vec!(
        Token::Init,
        Token::CharacterDef("waf".to_string(), "10".to_string(), "20".to_string(), "130".to_string()),
        Token::Begin,
        Token::NarratorStart,
        Token::Text("stuff".to_string()),
        Token::NarratorStop,
        Token::ColorStart("waf".to_string()),
        Token::DialogueStart,
        Token::Text("more stuff".to_string()),
        Token::DialogueStop,
        Token::ColorStop,
        Token::ColorStart("waf".to_string()),
        Token::DialogueStart,
        Token::Text("yet another thing".to_string()),
        Token::DialogueStop,
        Token::ColorStop,
        Token::End);
    let res = p.tokenize(text);
    dbg!(&res);
    assert_eq!(Ok(&tokens), res);
}

#[test]
fn test_parser_styling(){
    let mut p = Parser::new();
    let text = format!("\
waf;10;20;130
{}
waf;more $isstuff$ie
waf;yet $bsanother$be thing
", SETUP_SEPARATOR);
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
        Token::ColorStop,
        Token::ColorStart("waf".to_string()),
        Token::DialogueStart,
        Token::Text("yet ".to_string()),
        Token::BoldStart,
        Token::Text("another".to_string()),
        Token::BoldStop,
        Token::Text(" thing".to_string()),
        Token::DialogueStop,
        Token::ColorStop,
        Token::End);
    let res = p.tokenize(text);
    dbg!(&res);
    assert_eq!(Ok(&tokens), res);
}

#[test]
fn test_parser_characters_no_story(){
    let mut p = Parser::new();
    let text = format!("\
waf;10;20;130
be;199;230;1
{}
", SETUP_SEPARATOR);
    let tokens : Vec<Token> = vec!(
        Token::Init,
        Token::CharacterDef("waf".to_string(), "10".to_string(), "20".to_string(), "130".to_string()),
        Token::CharacterDef("be".to_string(), "199".to_string(), "230".to_string(), "1".to_string()),
        Token::Begin,
        Token::End);
    let res = p.tokenize(text);
    dbg!(&res);
    assert_eq!(Ok(&tokens), res);
}

#[test]
fn test_parser_characters_no_chars(){
    let mut p = Parser::new();
    let text = format!("\
\n{}
stuff
more stuff
", SETUP_SEPARATOR);
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

    dbg!(&res);
    dbg!(&split);
    assert_eq!("", split.0);
    assert_eq!("stuff\nmore stuff", split.1.trim());

    assert_eq!(Ok(&tokens), res);
}
#[test]
fn test_parser_spacing(){
    let mut p = Parser::new();
    let text = format!("\
\n{}
stuff
$sb
$lb
$nl
$ns", SETUP_SEPARATOR);
    let tokens : Vec<Token> = vec!(
        Token::Init,
        Token::Begin,
        Token::NarratorStart,
        Token::Text("stuff".to_string()),
        Token::NarratorStop,
        Token::ShortBreak,
        Token::LongBreak,
        Token::NewLong,
        Token::NewShort,
        Token::End);

    let res = p.tokenize(text);

    assert_eq!(Ok(&tokens), res);
}




//
// TEST SPLIT
// 
#[test]
fn test_split(){
    let p = Parser::new();
    let text = format!("waf;10;20;130
{}
stuff
more stuff
", SETUP_SEPARATOR);

    let split = p.split(text);
    assert_eq!("waf;10;20;130", split.0);
    assert_eq!("stuff\nmore stuff", split.1.trim());
}

#[test]
fn test_split_no_chars(){
    let p = Parser::new();
    let text = format!("
{}
stuff
more stuff
", SETUP_SEPARATOR);

    let split = p.split(text);
    assert_eq!("", split.0);
    assert_eq!("stuff\nmore stuff", split.1.trim());
}

#[test]
fn test_split_no_story(){
    let p = Parser::new();
    let text = format!("waf;10;20;130
{}
", SETUP_SEPARATOR);

    let split = p.split(text);
    assert_eq!("waf;10;20;130", split.0);
    assert_eq!("", split.1.trim());
}





//
// TEST BUILDER
//
#[test]
fn test_builder() {
    let builder = Builder::new();
    let tokens : Vec<Token> = vec!(
        Token::Init,
        Token::Begin,
        Token::End);
    let res = builder.build_stack(&tokens);
    let expected = Ok(String::from("\
\\documentclass{report}
\\usepackage{xcolor}
\\begin{document}
\\end{document}"));
    dbg!(&res);

    assert_eq!(expected, res);
}

#[test]
fn test_builder_dialogue() {
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
        Token::DialogueStart,
        Token::Text("more stuff".to_string()),
        Token::DialogueStop,
        Token::End);
    let res = builder.build_stack(&tokens);
    let expected = Ok(String::from("\
\\documentclass{report}
\\usepackage{xcolor}
\\definecolor{be}{RGB}{199,230,1}
\\begin{document}
narrator talking

-``\\textcolor{be}{stuff}''

-``\\textcolor{be}{more stuff}''

\\end{document}"));
    dbg!(&res);

    assert_eq!(expected, res);
}

#[test]
fn test_builder_spacing(){
    let builder = Builder::new();
    let tokens : Vec<Token> = vec!(
        Token::Init,
        Token::Begin,
        Token::ShortBreak,
        Token::LongBreak,
        Token::NewLong,
        Token::NewShort,
        Token::End);
    let res = builder.build_stack(&tokens);
    let expected = Ok(String::from("\
\\documentclass{report}
\\usepackage{xcolor}
\\begin{document}
\\begin{center}\\noindent\\rule{8cm}{0.4pt}\\end{center}
\\begin{center}\\noindent\\rule{3cm}{0.4pt}\\end{center}
\\vspace{10mm}
\\vspace{5mm}
\\end{document}"));

    assert_eq!(expected, res)

}





