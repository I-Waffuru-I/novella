use std::fmt::Display;

/// A catch-all enum to represent errors during parsing & building.
/// Entry names try to be self-explanatory.
#[derive(Debug,PartialEq)]
pub enum StoryError {
    UnknownError,
    RegexNoMatchedLine(String),
}
impl Display for StoryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug)]
pub enum LineType {
    Empty,
    Text,
    Spacing
}

/// Tokens describe what the input file is like. An iterable of these is provided by the Parser to the Builder.
#[derive(Debug,PartialEq)]
pub enum Token {
    Init,
    Begin, End,
    CharacterDef(String,String,String,String),
    DialogueStart, DialogueStop,
    NarratorStart, NarratorStop,
    Text(String),
    ItalicStart, ItalicStop,
    BoldStart, BoldStop,
    InsertStart,InsertStop,
    SmallStart, SmallStop,
    ColorStart(String),
    ShortBreak,
    LongBreak,
    NewShort,
    NewLong,
    
    
}

#[derive(Debug)]
pub enum MessageType {
    Error,
    Start,
    Title,
    Info,
    Message,
    End,
}
#[derive(Debug)]
pub struct Message {
    sender: String,
    message: String,
    side : bool
}
impl Message {
    pub fn new(lr : bool, ch : String, msg : String)->Message{
        Message {
            side: lr,
            sender: ch,
            message: msg
        }
    }
}






