# Markdown-To-HTML

*Markdown-to-html* is a Rust CLI application that lets you easily convert markdown documents to HTML ones. It's main usage is for me to easily write blogposts. 

*Please be aware that Markdown-To-HTML is not fully [CommonMark](https://commonmark.org) compatible as of this moment*

## Installation

Use the Rust [Cargo](https://doc.rust-lang.org/cargo/) compiler to build, and optionally use the `install` command on MacOS or Linux to install it. 

```bash
cargo build -r
sudo install target/release/markdown /usr/local/bin
```

## Usage

```bash
markdown <INPUT FILE NAME> -o <OUTPUT FILE NAME> [-css <PATH TO CSS FILE>][--utf8][--custombody <PATH TO CUSTOMBODY>][--title|-t <TITLE>][-h]

<INPUT FILE NAME> - the path to your markdown .md file
<OUTPUT FILE NAME> - the path to the HTML file you wish to create
[-css <PATH TO CSS FILE>] - An optional path to a CSS file to stylize your HTML
[--utf8] - Makes the HTML output display be encoded with UTF-8. (THIS IS REQUIRED TO DISPLAY EMOJIS)
[--custombody <PATH TO CUSTOMBODY>] - An optional path to a file containing 2 lines. First line will be included at the start of the HTML body
                                    - Second line will be included at the end of the HTML body (for adding a container around the HTML output)
[--title|-t <TITLE>] - An optional title which will be displayed at the top of the tab when the HTML is opened in a browser
[-h] Opens a help menu

**Example**
markdown markdown.md -o markdown.html --css ../style.css -t "Markdown" --custombody customheader.html
```

## Feature Support
As stated above, this application is not fully CommonMark compatible. However it does support:  
```
\*\*bold\*\* or \_\_bold\_\_  
\*Italic\* or \_Italic\_  
\~\~Strikethrough\~\~  
\`Code\`  
==Highlight==  
\~Subscript\~  
^Superscript^  
\# Headers (1-6)  
links \[link name\]\(link source\)  
images \![image name\]\(image source\)  
\* unordered lists  
\1. ordered lists  
\> quotes

As well as tables  
\| header 1 \| header 2\|  
\| ------------------ \|  
\| data 1     \| data 2 \|  
\| data 3     \| data 4  \|  
```


## Contributing

This project is not currently in active development. Feel free to contribute changes, however I cannot promise I will notice and/or accept them

## License

[GNU GPL v3](https://www.gnu.org/licenses/gpl-3.0.html)
