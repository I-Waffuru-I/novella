
<div align="center"> 
<h1>Novella</h1>
<i>A simple declarative narrative writing tool.</i>
</div>
<br />

Write your story as a text file with inline tags, run it through *Novella* and receive a *pdf* as output with all decorations applied. This is a convenience tool made to facilitate the writing of narrative stories and exporting them easily. 

--- 
The project started as a way to provide customization options for styling text. I'm a fan of adding colourization to characters, tying in with their personality & vibe. When a story stretches on for hundreds of pages, adding the colour each time is a redundant and error-prone task. Even worse, deciding the colour of one character doesn't match anymore, and needing to update it *everywhere* in the document. Not cool.

This self-made problem is solved by this self-made solution :)

> This project is far from complete, but the core functionality is present. I intend to revamp a lot of tags to Markdown-like notation, as it's more intuitive, plus expanding the features available.

## Installation
Currently, the only way to install *Novella* is to build it from source. This will require the Rust toolchain to be installed on your device, as `cargo` is used.
1. Clone this repository
2. Move into the directory
3. run `cargo build --release`
4. The executable will be located at `./target/release/novella`, which you can move where you want, or add to your PATH. 


## Documentation 

Here is [an example story file](./examples/park_walk.txt), the features of which I'll explain as we go.

```
# character definitions
bob;120;40;130
mom;40;10;10
gd;40;10;10

# delimiter
$$STORY$$

# narration
The leaves fell out the tree, a choir of silent deaths. Wind scurries past the branches in chaotic yet arrhythmic fashion.
# dialogue
bob;It sure is cold today!
mom;You're right! Luckily we brought our jackets with us.

$sb

In the distance, a security guard walks into view and makes his way to the duo.
gd;Hello there, I was wondering if you two are sufficiently prepared for the cold weather up ahead. I'm telling ya, there might even be snow soon. $bsSnow$be, in this day and age?!
mom;Oh yes, thank you $isvery$ie much. We brought everything we need.
bob;Yes sir.$asBobby takes something out of their bag.$ae We even have gloves.
```

Every story file starts with character definitions. These are characters in the story that talk and act (and sometimes even think!). 
Each character definition features a shorthand name and the rgb-value for the wanted colour. For example, the line `bob;120;40;130` defines an abbreviation string we can use when Bobby talks. 
> For now, these abbreviated names must be 2-3 chars long, but this may change later. 

`$$STORY$$` is a delimiter that splits the file in two. Everything before it counts as 'setup', while everything after is the 'story'. You can omit this if no characters are defined.

Next comes the story itself. Every line results in a text block in the final output. A line can either be narrated or dialogue. The latter is defined by the character short name appearing at the start. If no character is mentioned at the start, Novella treats it as a narration line. Inline tags are handled in both cases.

Styling tags are delimited by a **s**tart tag and **e**nd tag. Currently supported tags are:
- `$as & $ae`: Insert narration within a dialogue line 
- `$is & $ie`: Makes the text italic
- `$bs & $be`: Makes the text bold
- `$ss & $se`: Makes the font size smaller

There are a few 'Spacing' tags to play with empty spaces between paragraphs. When any of these are used, the line should be empty, safe for the tag.
- `$lb`: A long horizontal bar, often used to signify a long jump in time or new chapter
- `$sb`: A short horizontal bar, used moreso to detail a shorter flash forward 
- `$nl`: Leaves a vertical space between the lines
- `$ns`: Leaves a smaller vertical space between the lines

Empty lines and lines starting with `#` (comments) are ignored and won't make any difference on the output.

