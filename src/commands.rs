
use std::str::FromStr;

use crate::{Background, Color, ColorMap, LsItem, Style};


//mpl std::error::Error for String {}
const HELP: &str = r###"
    Help:
    Single Commands: 
        view -> show preview,
        help -> show help, 
        export -> prints the result
        quit -> back to your prompt

    Change commands:
    set [which] [what] [value] -> updates an entry ex.: 'set fi color 31' (sets regular file to color red) 
        [which]: the LS_COLORS entry (fi, de, ex ..)
        [what] : style | color | bg
        [value]: the value
"###;

const HELP_VALUES: &str = r###"
    Style values: 
        00 -> None, 01 -> Bold, 04 -> Underscore, 05 -> Blink, 07 -> Reverse, 08 -> Concealed
    Color values: 
        30 - 37: Black, Red, Green, Yellow, Bkue, Magenta, Cyan, White
        91 - 97: Same but bright
    Background values:
        40 - 47 Black, Red, Green, Yellow, Bkue, Magenta, Cyan, White
        100 - 107: Same but bright

"###;
pub fn set_action(map: &mut ColorMap, which: &str, what: &str, value: &str) -> Result<bool, Box<dyn std::error::Error>> {
    if map.contains_key(which) {
        match what {
            "style" => {
                let style_value = Style::from_str(value)?;
                map.entry(which.to_string()).and_modify(|item| {
                    item.style = Some(style_value);
                });
                return Ok(true);
                
            },
            "color" => {
                let color_value = Color::from_str(value)?;
                map.entry(which.to_string()).and_modify(|item| {
                    item.color = Some(color_value);
                });
                return Ok(true);
            },
            "bg" => {
                let bg_value = Background::from_str(value)?;
                map.entry(which.to_string()).and_modify(|item| {
                    item.bg = Some(bg_value);
                });
                return Ok(true);
            },
            _ => return Err("invalid action..".into())
        }
    } else {
       return  Err("Invalid key".into())
    }
    
}

pub fn add_action(map: &mut ColorMap, which: &str, what: &str, value: &str) -> Result<bool, Box<dyn std::error::Error>> {
    if map.contains_key(which) {
        return Err(format!("item: {} already in the list", which).into());
    } else if !which.starts_with("*.") {
        return Err("Only file extensions can be added to the list (*.ext)".into());
    } else {
        let (style, color, bg) = match what {
            "style" => (Some(Style::from_str(value)?), None, None),
            "color" => (None, Some(Color::from_str(value)?), None),
            "bg" =>    (None, None, Some(Background::from_str(value)?)),
            _ => return Err(format!("can't set {} for new item (valid: style, color, bg)", what).into())
        };
        let item = LsItem {
            style, color, bg,
            description: format!("file extension: {}", which),
            order: map.len() + 1,

        };
        map.insert(which.to_string(), item);
        Ok(true)
    }
}
pub fn remove_action(map: &mut ColorMap, which: &str) -> Result<bool, Box<dyn std::error::Error>> {
    if which.starts_with("*.") {
        let r = map.remove(which);
        Ok(r.is_some())
    } else if map.contains_key(which) {
        map.entry(which.to_string()).and_modify(|item| {
            item.bg = None;
            item.color = None;
            item.style = None;
        });
        Ok(true)
    } else {
        Err(format!("cannot remove or unset: {}", which).into())
    }
}

pub fn print_help() {
    println!("{}", HELP);
    println!("{}", HELP_VALUES);
}