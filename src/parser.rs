use regex::Regex;
use crate:: types::{
    LineType as LT, StoryError, Token
};


/// Parses inputs to a stack of Tokens to be handled externally.
pub struct Parser {
    character_reg : Regex,
    line_reg : Regex,
    tokens : Vec<Token>,
    setup_separator : String,
    token : char,
}

impl Parser {
    pub fn new(separator:&str, token : char)->Parser {
        let line_reg = Regex::new(r"^(?<name>\w{1,3});(?<line>.+$)").expect("Line Regex init failed");
        let character_reg = Regex::new(r"(?<name>\w{1,3});(?<red>\d{1,3});(?<gre>\d{1,3});(?<blu>\d{1,3})").expect("Character Regex init failed");
        let tokens : Vec<Token> = vec!();

        Parser {
            character_reg,
            line_reg,
            tokens,
            token,
            setup_separator : separator.to_string(),
        }
    }

    pub fn tokenize(&mut self,full_text : String)-> Result<&Vec<Token>,StoryError> {
        self.tokens = vec!(Token::Init);
        let split = self.split(full_text);

        self.parse_characters(&split.0)?;
        self.tokens.push(Token::Begin);
        self.parse_story(&split.1);
        self.tokens.push(Token::End);

        Ok(&self.tokens)
    }


    pub fn split(&self, full_text : String) -> (String,String) {
        let mut split_index : u8 = 0;
        let mut has_setup = false;
        let mut nl_count : u8 = 0;

        for (i,v) in full_text.lines().enumerate() {
            if v.contains(&self.setup_separator) {
                split_index = if i > 0 { i as u8 -1} else { 0 };
                has_setup = true;
                let _ = full_text.replace(&self.setup_separator, "\n");
                break;
            }
        }
        if !has_setup {
            return (String::new(),full_text)
        }

        // Find indexes to slice at at the (split_index)th newline
        let setup_end_index : u64;
        let story_start_index : u64;
        for (i,v) in full_text.chars().enumerate() {
            if v == '\n' {
                if nl_count >= split_index {
                    setup_end_index = i as u64;
                    story_start_index = i as u64 + self.setup_separator.len() as u64;
                    return (full_text[..setup_end_index as usize].to_string() , full_text[story_start_index as usize+1.. ].to_string())
                } else {
                    nl_count += 1;
                }
            }
        }
        
        (String::new(), full_text)
    }

    /// Parses the setup string to tokens
    fn parse_characters(&mut self,lines : &str) ->Result<(),StoryError> {
        for l in lines.lines(){
            if l.trim().is_empty() || l.trim().starts_with('#') {
                continue
            }
             let ch = self.parse_setup_to_char(l)?;
             self.tokens.push(ch);
        }

        Ok(())
    }
    fn parse_setup_to_char(&self, line: &str) -> Result<Token,StoryError> {
        if let Some(c) = self.character_reg.captures(line) {
            let (_,[name, red, gre, blu]) = c.extract();
            let token = Token::CharacterDef(name.to_string(), red.to_string(), gre.to_string(), blu.to_string());
            Ok(token)
        } else {
            Err(StoryError::RegexNoMatchedLine(line.to_string()))
        }
    }

    /// Parses all lines in the story string to tokens
    fn parse_story(&mut self, lines : &str) {
        for l in lines.lines() {
            match self.get_line_type(l) {
                LT::Empty => {
                    continue
                }
                LT::Spacing => {
                    self.tokens.push(self.get_spacing_token(&l[1..]));
                }
                LT::Text => {
                    self.tokenize_text(l);
                }
            }
        }
    }

    fn tokenize_text(&mut self, line : &str) {
        let mut last_consumed : usize = 0;
        let mut has_name = false;
        let mut start_index : usize = 0;
        if let Some(c) = self.line_reg.captures(line) {
            let name = c["name"].to_string();
            has_name = true;
            last_consumed = name.len()+1;
            start_index = last_consumed;
            self.tokens.push(Token::ColorStart(name));
            self.tokens.push(Token::DialogueStart)
        } else {
            self.tokens.push(Token::NarratorStart);
        }
        for (i, c) in line[last_consumed..].chars().enumerate() {
            let j = start_index + i;
            if c == self.token {
                self.tokens.push(Token::Text(line.chars().take(j).skip(last_consumed).collect()));
                // Have to Take chars instead of slicing the str, because of multi-byte chars like
                // `'`
                let cmd : String = line.chars().take(j+3).skip(j+1).collect();
                let cmd_token = self.get_token_from_command(&cmd);
                last_consumed = j+3;
                self.tokens.push(cmd_token);
            }
        }
        if last_consumed != line.len() {
            self.tokens.push(Token::Text(line.chars().skip(last_consumed).collect()))
        }

        if has_name {
            self.tokens.push(Token::DialogueStop);
        } else {
            self.tokens.push(Token::NarratorStop);
        }
    }

    fn get_token_from_command(&self, command : &str)-> Token {
        match command {
            "is" => Token::ItalicStart,
            "ie" => Token::ItalicStop,
            "bs" => Token::BoldStart,
            "be" => Token::BoldStop,
            "as" => Token::InsertStart,
            "ae" => Token::InsertStop,
            "ss" => Token::SmallStart,
            "se" => Token::SmallStop,
            rest => Token::Text(rest.to_string())
        }
    }


    fn get_line_type(&self, line : &str) -> LT {
        let l = line.trim();
        if l.is_empty() {
            return LT::Empty
        }
        if l.starts_with('#') {
            return LT::Empty
        }
        if l.starts_with(self.token) {
            if l.contains("lb") {
                return LT::Spacing
            }
            if l.contains("sb") {
                return LT::Spacing
            }
            if l.contains("nl"){
                return LT::Spacing
            }
            if l.contains("ns") {
                return LT::Spacing
            }
        }
        return LT::Text
    }

    fn get_spacing_token(&self, line : &str) -> Token {
        match line.to_lowercase().as_str() {
            "lb" => Token::LongBreak,
            "sb" => Token::ShortBreak,
            "nl" => Token::NewLong,
            "ns" => Token::NewShort,
            def => Token::Text(def.to_string())
        }
    }


    /*
    fn get_message_from_line(line :&str) -> Message {
        let c = line.chars().nth(2).unwrap();
        let side = match c {
            'l' | 'L' => false,
            'r' | 'R' => true,
            _ => true
        };
        let mut msg = String::new();
        let mut ch = String::new();
        match c {
            'l' | 'r' => {
                match get_short_name(&line[4..]) {
                    Some(c) => {
                        ch = c.0;
                        msg = c.1;
                    }
                    None => {}
                }
            }
            'L' | 'R' => {
                msg = String::from(&line[3..])
            },
            _ => {}
        };

        Message::new(side,ch,msg)
    }
    */

}



