
<div align="center"> 
<h1>Stylo</h1>
A simple declarative narrative writing tool.
</div>

Write your story as a normal text file with inline tags, run it through Stylo and receive a pdf as output with all decorations applied. 

This project is far from complete, but the core functionality is present. I intend to revamp a lot of tags to Markdown-like notation, as it's more intuitive for a lot of people.
## Documentation 

Here is an example story file, the features of which I'll explain as we go.

```
bob;120;40;130
mom;40;10;10
gd;40;10;10

$$STORY$$

The leaves fell out the tree, a choir of silent deaths. Wind scurries past the branches in chaotic yet arrhythmic fashion.
bob;It sure is cold today!
mom;You're right! Luckily we brought our jackets with us.

In the distance, a security guard walks into view and makes his way to the duo.

gd;Hello there, I was wondering if you two are sufficiently prepared for the cold weather up ahead. I'm telling ya, there might even be snow soon. $bsSnow$be, in this day and age?!
mom;Oh yes, thank you $isvery$ie much. We brought everything we need.
bob;Yes sir.$asBobby takes something out of their bag.$ae We even have gloves.
```

Every story file starts with character definitions. These are characters in the story that talk and act (and sometimes even think!). This project started as a way to easily style these dialogues, for example by having each person be tied to a colour in which their dialogue is rendered.

Each character definition features a shorthand name and the rgb-value for the wanted colour. The line `bob;120;40;130` defines an abbreviation string we can use when Bobby talks. For now, these abbreviated names must be 2-3 chars long, but this may change later.

`$$STORY$$` is a delimiter that splits the file in two. Everything before it counts as 'setup', while everything after is the 'story', or 'content'. 




Styling tags are delimited by a **s**tart tag and **e**nd tag. Currently supported tags are:
- `$as & $ae`: Insert narration within a dialogue line 
- `$is & $ie`: Sets italic mode
- `$bs & $be`: Sets bold mode
- `$ss & $se`: Sets small font mode

There are a few 'Spacing' tags to play with empty spaces between paragraphs.
- `$lb`: A long horizontal bar, often used to signify a long jump in time or new chapter
- `$sb`: A short horizontal bar, used moreso to detail a shorter flash forward e.g.
- `$nl`: Leaves a vertical space between the lines
- `$ns`: Leaves a smaller vertical space between the lines
