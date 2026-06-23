use std::ops::Mul;

use crate::types::*;



pub struct Builder {

}
impl Builder {

    pub fn new() -> Builder {
        Builder {

        }
    }

    pub fn build_stack(&self, tokens : &Vec<Token>)-> Result<String,StoryError> {
        let mut tex_s = String::new();
        let mut current_color = String::from("black");
        let mut current_dialogue_color = String::from("black");
        let mut current_mode = TextMode::Normal;

        for token in tokens {
            match token {
                Token::Init => tex_s += "\\documentclass{report}\n\\usepackage{xcolor}\n\\setlength\\parindent{0pt}",
                Token::CharacterDef(n,r,g,b)=> tex_s += &self.define_character(&n, &r, &g, &b),
                Token::Begin=> tex_s += "\n\\begin{document}",
                Token::End => tex_s += "\n\\end{document}",

                Token::ColorStart(color) => (current_color, current_dialogue_color) = if current_mode == TextMode::Normal {("black".to_string(), color.clone())} 
                    else {("gray".to_string(), color.clone() +"f")},

                Token::Text(t) => tex_s += t,

                Token::NarratorStart => tex_s += &format!("\n\\textcolor{{{current_color}}}{{"),
                Token::NarratorStop => tex_s += "}\n",
                Token::DialogueStart => tex_s += &format!("\n-``\\textcolor{{{current_dialogue_color}}}{{"),
                Token::DialogueStop => tex_s += "}''\n",
                Token::InsertStart => tex_s += &format!("}}'' \\textcolor{{{current_color}}}{{"),
                Token::InsertStop => tex_s += &format!("}}`` \\textcolor{{{current_dialogue_color}}}{{"),

                Token::FlashbackStart => {
                    current_mode = TextMode::Flashback;
                    current_color = String::from("gray");
                }
                Token::FlashbackStop => {
                    current_mode = TextMode::Normal;
                    current_color = String::from("black");
                }

                Token::ItalicStart => tex_s += "\\textit{",
                Token::ItalicStop => tex_s += "} ",
                Token::BoldStart => tex_s += "\\textbf{",
                Token::BoldStop => tex_s += "} ",
                Token::SmallStart => tex_s += "\\begin{small}",
                Token::SmallStop => tex_s += "\\end{small}",

                Token::ShortBreak => tex_s += "\n\\begin{center}\\noindent\\rule{3cm}{0.4pt}\\end{center}\n",
                Token::LongBreak => tex_s += "\n\\begin{center}\\noindent\\rule{7cm}{0.4pt}\\end{center}\n",
                Token::NewLong => tex_s += "\n\\vspace{10mm}\n",
                Token::NewShort => tex_s += "\n\\vspace{5mm}\n",

            }
        }

        Ok(tex_s)

    }

    fn define_character(&self, name : &str, r:&str, g:&str, b:&str)-> String {
        let rf : u8 = ( u8::from_str_radix(r, 10).unwrap_or(1)  as f64).mul(1.4) as u8;
        let gf : u8 = ( u8::from_str_radix(g, 10).unwrap_or(1)  as f64).mul(1.4) as u8;
        let bf : u8 = ( u8::from_str_radix(b, 10).unwrap_or(1)  as f64).mul(1.4) as u8;
        let s = format!("\n\\definecolor{{{name}}}{{RGB}}{{{r},{g},{b}}}\n\\definecolor{{{name}f}}{{RGB}}{{{rf},{gf},{bf}}}");
        s
    }

    /*
    fn build_line(line : LineType) -> String {
        let mut result_line = String::from("\\noindent ");
        match line {
            LineType::Comment => {},
            LineType::Empty=> {},
            LineType::Dialogue(c,t ) => {
                let s = format!("-``\\textcolor{{{c}}}{{{t}}}''");
                let s_r = Self::build_line_styles(&s);
                result_line.push_str(&s_r);
            }
            LineType::Narrator(t) => {
                let s = &format!("{t}");
                let s_r = Self::build_line_styles(&s);
                result_line.push_str(&s_r);
            }
            LineType::Insert(c,d1,t,d2) => {
                let s = format!("-``\\textcolor{{{c}}}{{{d1}}}'' {t} ``\\textcolor{{{c}}}{{{d2}}}''");
                let s_r = Self::build_line_styles(&s);
                result_line.push_str(&s_r);
            }
            LineType::Message(m) => result_line.push_str(Self::build_message_line(m).as_str()),

            LineType::ShortBreak => result_line.push_str("\\begin{center}\\noindent\\rule{8cm}{0.4pt}\\end{center}"),
            LineType::LongBreak => result_line.push_str("\\begin{center}\\noindent\\rule{3cm}{0.4pt}\\end{center}"),
            LineType::NewLong => result_line.push_str("\\vspace{10mm}"),
            LineType::NewShort => result_line.push_str("\\vspace{5mm}"),
       };
        result_line.push_str("\r\n\r\n");
        result_line
    }
    */


/*
    fn build_message_line(input : MessageType) -> String {
        match input {
            MessageType::Start | MessageType::End => "\\begin{center}\\noindent\\rule{8cm}{0.4pt}\\end{center}".to_string(),
            MessageType::Title(title) => format!("\\begin{{center}}\\textit{{{title}}}\\end{{center}}"),
            MessageType::Error => String::new(),
            MessageType::Info(s,i) => {
                let side = if s {"flushright"} else {"flushleft"};
                format!("\\begin{{{side}}}{}\\end{{{side}}}",i)
            }
            MessageType::Message(m) => {
                let side = if m.side {"flushright"} else {"flushleft"};

                let s = format!("\\textcolor{{{}}}{{{}}}",m.sender, m.message);
                format!("\\begin{{{side}}}{}\\end{{{side}}}",s)

            }
        }
    }

*/


}



