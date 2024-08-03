#![allow(dead_code)]
#![allow(unused_imports)]

use std::str::FromStr;
use std::{collections::HashMap, env, ops::Deref};
use std::io::{self, Write};

use rustyline::error::ReadlineError;
use rustyline::{DefaultEditor, Result};

mod types;
mod commands;
//use commands::*;
use types::*;


fn main() {

    let mut color_decs = ColorMap::default();
    match env::var_os("LS_COLORS") {
        Some(colors) => { 
            let color_str = colors.into_string().unwrap();
            color_decs.parse_env_string(color_str);
            
            
        },
        None => panic!("$LS_COLORS is not set"),
    };

    //start_loop().unwrap();
    let mut rl = DefaultEditor::new().unwrap();
    loop {
       let buffer = rl.readline(">> ").unwrap();
        let _ = rl.add_history_entry(buffer.as_str());
        let commands:Vec<_> = buffer.split(' ').collect();
        match *commands.as_slice() {
            [single_cmd] => {
                match single_cmd {
                    "help" => {
                        commands::print_help();
                    }
                    "quit" => {break;},
                    "view" => {
                        print_preview_map(&color_decs);
        
                    },
                    "export" => {
                        let ls_str = create_ls_string(&color_decs);
                        println!("LS_COLORS=\"{}\"", ls_str);
                    },
                    unknown => println!("Unknown single command: {}", unknown)
                }
            },
            [action, which, what, value] => {
                if action == "set" {
                    match commands::set_action(&mut color_decs, which, what, value) {
                        Ok(_) => {
                            println!("Success set action.");
                            print_preview_map(&color_decs);
                        },
                        Err(e) => println!("{}", e),
                    }
                }

                if action == "add" {
                    match commands::add_action(&mut color_decs, which, what, value) {
                        Ok(_) => {
                            println!("Success add action.");
                            print_preview_map(&color_decs);
                        },
                        Err(e) => println!("{}", e),
                    }
                }
            },
            [action, which] => {
                if action == "remove" {
                    match commands::remove_action(&mut color_decs, which) {
                        Ok(_) => {
                            println!("Success remove action.");
                            print_preview_map(&color_decs);
                        },
                        Err(e) => println!("{}", e),
                    }
                }
            }
            //
            
            [..] => {
                println!("unknown instruction: {}", buffer);
            }
        }
    }
    //print_preview_map(&color_decs);
    //println!("{:?}", &color_decs);
    //println!("{}", create_ls_string(&color_decs));
    
    //do so
}

fn print_preview_map(color_decs: &ColorMap) {
    let mut preview_map: HashMap<String, Vec<(&str, &LsItem)>> = HashMap::new();

    for (key, item) in color_decs.deref() {
        
        //let k = item.make_color_id();
        let mut k = item.to_string();
        if !key.starts_with("*.") {
            k.push_str(" - ");
            k.push_str(key);
            
            preview_map.insert(k, vec![(key, &item)]);
            
        } else {
            k.push_str(" - ext");
            preview_map.entry(k).and_modify(|v| v.push((key, &item))).or_insert(vec![(key, &item)]);
        }
        //println!("{}: {}", key, item.preview());
    }
    // for (_color_id, item) in &preview_map {
    //     let value: String;
    //     if item.len() > 1 {
    //         let d = item.iter().map(|e| e.0).collect::<Vec<_>>().join(";");
    //         value = item[0].1.color_helper(d);
    //     } else {
    //         value = format!("[{}] {}", item[0].0, item[0].1._preview());
    //     }
    //     //println!("{} : {}", item[0].1.display_colors_values(), value);
    //     //println!("{:<13}: {}", color_id, value);
    // }
    let mut map_vec: Vec<_> = preview_map.values().collect();
    map_vec.sort_by_key(|items| items[0].1.order);
    for item in map_vec {
        let value: String;
        if item.len() > 1 {
            // let d = item.iter().map(|e| e.0).collect::<Vec<_>>().join(";");
            // value = item[0].1.color_helper(d);
            let files_vec = item.iter().map(|e| e.0).collect::<Vec<_>>();
            let mut chunks = files_vec.chunks(13);
            println!("{} : {}", item[0].1.display_colors_values(), item[0].1.color_helper(chunks.next().unwrap().join(";").as_ref()));
            //let spacer = "                     ";
            while let Some(c) = chunks.next() {
                let line = c.join(";");
                println!("{:>width$}", item[0].1.color_helper(&line), width=line.len() + 33);
            }
        } else {
            value = format!("[{}] {}", item[0].0, item[0].1._preview());
            println!("{} : {}", item[0].1.display_colors_values(), value);
        }
        
        //println!("{}", value);
    }
}

fn create_ls_string(color_decs: &ColorMap) -> String {
    let mut key_vec: Vec<(&String, &LsItem)> = color_decs.iter().collect();
    key_vec.sort_by_key(|&item| item.1.order);
    let ls_values: Vec<String> = key_vec.iter().map(|&entry| format!("{}={}", entry.0, entry.1.to_string())).collect();
    ls_values.join(":")
}
