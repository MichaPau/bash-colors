use std::{collections::HashMap, ops::{Deref, DerefMut}, str::FromStr};

use clap::Subcommand;

#[derive(Debug)]
pub struct ColorMap(HashMap<String, LsItem>);

impl Default for ColorMap {
    fn default() -> Self {
        let mut color_decs = HashMap::new();
        color_decs.insert("bd".to_string(),LsItem { style: None, color: None, bg: None, description: "block device".to_string(), order: 15 });
        color_decs.insert("ca".to_string(),LsItem { style: None, color: None, bg: None, description: "file with capability".to_string(), order: 2 });
        color_decs.insert("cd".to_string(),LsItem { style: None, color: None, bg: None, description: "character device".to_string(), order: 3 });
        color_decs.insert("di".to_string(),LsItem { style: None, color: None, bg: None, description: "directory".to_string(), order: 4 });
        color_decs.insert("do".to_string(),LsItem { style: None, color: None, bg: None, description: "door".to_string(), order: 5 });
        color_decs.insert("ex".to_string(),LsItem { style: None, color: None, bg: None, description: "executable file".to_string(), order: 6 });
        color_decs.insert("fi".to_string(),LsItem { style: None, color: None, bg: None, description: "regular file".to_string(), order: 7 });
        color_decs.insert("ln".to_string(),LsItem { style: None, color: None, bg: None, description: "symbolic link".to_string(), order: 8 });
        color_decs.insert("mh".to_string(),LsItem { style: None, color: None, bg: None, description: "multi-hardlink".to_string(), order: 9 });
        color_decs.insert("mi".to_string(),LsItem { style: None, color: None, bg: None, description: "missing file".to_string(), order: 10 });
        color_decs.insert("no".to_string(),LsItem { style: None, color: None, bg: None, description: "normal non-filename text".to_string(), order: 11 });
        color_decs.insert("or".to_string(),LsItem { style: None, color: None, bg: None, description: "orphan symlink".to_string(), order: 12 });
        color_decs.insert("ow".to_string(),LsItem { style: None, color: None, bg: None, description: "other-writable directory".to_string(), order: 13 });
        color_decs.insert("pi".to_string(),LsItem { style: None, color: None, bg: None, description: "named pipe, AKA FIFO".to_string(), order: 14 });
        color_decs.insert("rs".to_string(),LsItem { style: None, color: None, bg: None, description: "reset to no color".to_string(), order: 1 });
        color_decs.insert("sg".to_string(),LsItem { style: None, color: None, bg: None, description: "set-group-ID".to_string(), order: 16 });
        color_decs.insert("so".to_string(),LsItem { style: None, color: None, bg: None, description: "socket".to_string(), order: 17 });
        color_decs.insert("st".to_string(),LsItem { style: None, color: None, bg: None, description: "sticky directory".to_string(), order: 18 });
        color_decs.insert("su".to_string(),LsItem { style: None, color: None, bg: None, description: "set-user-ID".to_string(), order: 19 });
        color_decs.insert("tw".to_string(),LsItem { style: None, color: None, bg: None, description: "sticky and other-writable directory".to_string(), order: 20 });
        Self(color_decs)
    }
}
impl Deref for ColorMap {
    type Target = HashMap<String, LsItem>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for ColorMap {
   
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl ColorMap {
    
    pub fn parse_env_string(&mut self, env_string: String) {
        let color_split: Vec<_> = env_string.split(':').map(|s| s.to_string()).filter(|s| s.len() > 0).collect();

            for item in &color_split {
                //println!("{:?}", item);
                let (key, values) = item.split_once('=').unwrap();
                let (style_split, fg_split, bg_split) = parse_values(values);

                let style: Option<Style>;
                let fg: Option<Color>;
                let bg: Option<Background>;

                if let Some(style_str) = style_split {
                    //Ansi color 
                    if style_str == "38" {
                        style = None;
                    } else {
                        style = Style::from_str(style_str).ok();
                    }
                } else {
                    style = None;
                }

                if let Some(fg_str) = fg_split {
                    fg = Color::from_str(fg_str).ok();
                } else {
                    fg = None;
                }

                if let Some(bg_str) = bg_split {
                    bg = Background::from_str(bg_str).ok();
                } else {
                    bg = None;
                }

                if let Some(ls_item) = self.get_mut(key) {
                    ls_item.style = style;
                    ls_item.color = fg;
                    ls_item.bg = bg;
                } else {
                    let order = self.len() + 1;
                    self.insert(key.to_owned(), LsItem { style, color: fg, bg, description: format!("file extension: {}", key), order});
                }
            }
    }
}
//https://askubuntu.com/questions/466198/how-do-i-change-the-color-for-directories-with-ls-in-the-console
//https://www.bigsoft.co.uk/blog/2008/04/11/configuring-ls_colors
// https://askubuntu.com/questions/17299/what-do-the-different-colors-mean-in-ls

//https://en.wikipedia.org/wiki/ANSI_escape_code#8-bit
#[derive(Debug, PartialEq, PartialOrd)]
pub enum Style {
    None,
    Bold,
    Underscore,
    Blink,
    Reverse, 
    Concealed,
}
impl FromStr for Style {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "00" => Ok(Self::None),
            "01" => Ok(Self::Bold),
            "04" => Ok(Self::Underscore),
            "05" => Ok(Self::Blink),
            "07" => Ok(Self::Reverse),
            "08" => Ok(Self::Concealed),
            _ => Err(format!("Style: {} not implemented!", s)),

        }
    }
}

impl ToString for Style {
    fn to_string(&self) -> String {
        match self {
            Self::None => "00".into(),
            Self::Bold => "01".into(),
            Self::Underscore => "04".into(),
            Self::Blink => "05".into(),
            Self::Reverse => "07".into(),
            Self::Concealed => "08".into(),
        }
    }
}
#[derive(Debug, PartialEq, PartialOrd)]
pub enum Color {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    BrightBlack,
    BrightRed,
    BrightGreen,
    BrightYellow,
    BrightBlue,
    BrightMagenta,
    BrightCyan,
    BrightWhite,
    _Ansi(u8), //38;5
    _RGB((u8, u8, u8)),
}

impl FromStr for Color {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "30" => Ok(Self::Black),
            "31" => Ok(Self::Red),
            "32" => Ok(Self::Green),
            "33" => Ok(Self::Yellow),
            "34" => Ok(Self::Blue),
            "35" => Ok(Self::Magenta),
            "36" => Ok(Self::Cyan),
            "37" => Ok(Self::White),
            "90" => Ok(Self::BrightBlack),
            "91" => Ok(Self::BrightRed),
            "92" => Ok(Self::BrightGreen),
            "93" => Ok(Self::BrightYellow),
            "94" => Ok(Self::BrightBlue),
            "95" => Ok(Self::BrightMagenta),
            "96" => Ok(Self::BrightCyan),
            "97" => Ok(Self::BrightWhite),
            _ => Err(format!("Color: {} not implemented!", s)),
        }
    }
}
impl ToString for Color {
    fn to_string(&self) -> String {
        match self {
            Self::Black => "30".into(),
            Self::Red => "31".into(),
            Self::Green => "32".into(),
            Self::Yellow => "33".into(),
            Self::Blue => "34".into(),
            Self::Magenta => "35".into(),
            Self::Cyan => "36".into(),
            Self::White => "37".into(),
            Self::BrightBlack => "90".into(),
            Self::BrightRed => "91".into(),
            Self::BrightGreen => "92".into(),
            Self::BrightYellow => "93".into(),
            Self::BrightBlue => "94".into(),
            Self::BrightMagenta => "95".into(),
            Self::BrightCyan => "96".into(),
            Self::BrightWhite => "97".into(),
            _ => "".into(),
        }
    }
} 
#[derive(Debug, PartialEq, PartialOrd)]
pub enum Background {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    BrightBlack,
    BrightRed,
    BrightGreen,
    BrightYellow,
    BrightBlue,
    BrightMagenta,
    BrightCyan,
    BrightWhite,
    _Ansi(u8),
    _RGB((u8, u8, u8)),
}


impl FromStr for Background {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "40" => Ok(Self::Black),
            "41" => Ok(Self::Red),
            "42" => Ok(Self::Green),
            "43" => Ok(Self::Yellow),
            "44" => Ok(Self::Blue),
            "45" => Ok(Self::Magenta),
            "46" => Ok(Self::Cyan),
            "47" => Ok(Self::White),
            "100" => Ok(Self::BrightBlack),
            "101" => Ok(Self::BrightRed),
            "102" => Ok(Self::BrightGreen),
            "103" => Ok(Self::BrightYellow),
            "104" => Ok(Self::BrightBlue),
            "105" => Ok(Self::BrightMagenta),
            "106" => Ok(Self::BrightCyan),
            "107" => Ok(Self::BrightWhite),
            _ => Err(format!("Background: {} not implemented!", s)),
        }
    }
}
impl ToString for Background {
    fn to_string(&self) -> String {
        match self {
            Self::Black => "40".into(),
            Self::Red => "41".into(),
            Self::Green => "42".into(),
            Self::Yellow => "43".into(),
            Self::Blue => "44".into(),
            Self::Magenta => "45".into(),
            Self::Cyan => "46".into(),
            Self::White => "47".into(),
            Self::BrightBlack => "100".into(),
            Self::BrightRed => "101".into(),
            Self::BrightGreen => "102".into(),
            Self::BrightYellow => "103".into(),
            Self::BrightBlue => "104".into(),
            Self::BrightMagenta => "105".into(),
            Self::BrightCyan => "106".into(),
            Self::BrightWhite => "107".into(),
            _ => "".into(),
        }
    }
} 
#[derive(Debug, PartialEq, PartialOrd)]
pub struct LsItem {
    pub style: Option<Style>,
    pub color: Option<Color>,
    pub bg: Option<Background>,
    pub description: String,
    pub order: usize,
}

impl ToString for LsItem {
    fn to_string(&self) -> String {
        let mut values = vec![];
        if let Some(style) = &self.style {
            values.push(style.to_string())
        }
        if let Some(color) = &self.color {
            values.push(color.to_string())
        }
        if let Some(bg) = &self.bg {
            values.push(bg.to_string())
        }

        values.join(";")
    }
}
//https://en.wikipedia.org/wiki/ANSI_escape_code
impl LsItem {
    pub fn display_colors_values(&self) -> String {
        let style =  if let Some(style) = &self.style { style.to_string() } else { "--".to_string() };
        let fg =  if let Some(fg) = &self.color { fg.to_string() } else { "--".to_string() };
        let bg =  if let Some(bg) = &self.bg { bg.to_string() } else { "--".to_string() };

        format!("{};{:>7};{:>7}", style, fg, bg)
    }
    pub fn color_helper(&self, s: &str) -> String {
        let preffix = "\x1b[";
        let suffix  = "\x1b[0m";
        format!("{}{}m{}{}", preffix, self.make_color_id(), s, suffix)
    }
    pub fn _preview(&self) -> String {
        //println!("\x1b[0;31mSO\x1b[0m")
        let preffix = "\x1b[";
        let suffix  = "\x1b[0m";
        
        format!("{}{}m{}{}", preffix, self.make_color_id(), self.description, suffix)
    }

    pub fn make_color_id(&self) -> String {
        let mut values = vec![];
        if let Some(style) = &self.style {
            values.push(style.to_string())
        }
        if let Some(color) = &self.color {
            values.push(color.to_string())
        }
        if let Some(bg) = &self.bg {
            values.push(bg.to_string())
        }

        values.join(";")

    }
}

//TODO parse ansi value 38
pub fn parse_values(s: &str) -> (Option<&str>, Option<&str>, Option<&str>) {
    let split = s.splitn(3, ';');
    let sv: Vec<&str> = split.collect();
    let mut v = [None, None, None];
    for item in sv {
        
        match item.chars().next() {
            Some('0') => v[0] = Some(item),
            Some('3') => v[1] = Some(item),
            Some('4') => v[2] = Some(item),
            None => (),
            _ => (),
        }
    }
    
    
    (v[0], v[1], v[2])
}