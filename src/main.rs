use std::string::String;
use std::vec::Vec;
use std::fs::File;
use std::io::BufReader;
use serde::{ Deserialize, Serialize };
use clap::Parser;
use fltk::{ app, draw, prelude::* };
use fltk::enums::{ Color, Align, Font, FrameType };
use fltk::window::Window;
use fltk::group::Flex;
use fltk::widget::Widget;


/// Shrots
#[derive(Parser, Debug)]
struct Args {
    /// The JSON configuration file
    #[arg(short, long)]
    config: String,
}


#[derive(Debug, Serialize, Deserialize)]
struct Config {
    width: i32,
    height: i32,
    opacity: f64,
    border_size: i32,
    heading_size: i32,
    shrot_size: i32,
    bg_col: String,
    fg_col: String,
    pr_col: String,
    sh_col: String,
    br_col: String,
    ab_col: String,
    columns: Vec<Vec<Block>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Block {
    title: String,
    rel_size: f64,
    shrots: Vec<Entry>
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Entry {
    bind: String,
    about: String,
}


fn draw_block(block: Block, config: &Config, rel_y: i32, rel_h: i32) -> Widget {
    let mut widget = Widget::default();

    let padding_x: i32 = 8;
    let padding_y: i32 = 0;

    let bg_color = rgb(&config.bg_col);
    let fg_color = rgb(&config.fg_col);
    let sh_color = rgb(&config.sh_col);
    let ab_color = rgb(&config.ab_col);

    // Sets the custom draw method
    // For the widget
    widget.draw(move |w| {
        // Set position & size
        w.set_pos(w.x(), rel_y);
        w.set_size(w.w(), rel_h);

        draw::draw_box(FrameType::FlatBox, w.x(), w.y(), w.w(), w.h(), bg_color);
        
        // Calculate the drawed header size
        draw::set_font(Font::HelveticaBold, 24);
        let (_w_header, h_header) = draw::measure(&block.title, false);
        
        // Draw the header
        draw::set_draw_color(fg_color);
        draw::draw_text2(&block.title,
                         w.x() + padding_x,
                         w.y() + padding_y,
                         w.w(), w.h(),
                         Align::TopLeft);

        // Calculate the current line coordinate
        let mut line_y = padding_y + h_header;

        // Initialize variables to find the maximum shortcut size
        let mut max_len: i32 = 0;
        let mut max_shortcut: String = String::new();

        //
        // Find the biggest shortcut length
        // Used to calculate the about text's offset
        //
        for shortcut in &block.shrots {
            // Get shortcut text and length
            let s = &shortcut.bind;
            let len = s.len() as i32;
            // Check if length is greater than current maximum
            if len > max_len {
                // If so asign the length and shortcut
                max_len = len;
                max_shortcut = s.to_string();
            }
        }

        // Calculate the padding for the about section
        // It is equal to the sum of the maximum shortcut width and spacing
        draw::set_font(Font::HelveticaBold, 12);
        let padding_about: i32 = draw::width2(&max_shortcut.as_str(), max_len).ceil() as i32 + 32;

        // Draw shortcuts and abouts
        for shortcut in &block.shrots {
            //
            // TODO: Implement error parsing
            //

            // Draw the shortcut text
            draw::set_draw_color(sh_color);
            draw::draw_text2(&shortcut.bind,
                             w.x() + padding_x,
                             w.y() + line_y,
                             w.w(), w.h(),
                             Align::TopLeft);

            // Get the about texts height
            // It is assumend that shortcuts do not contain newlines
            let (_w_about, h_about) = draw::measure(&shortcut.about, false);
            // Draw the about text
            draw::set_draw_color(ab_color);
            draw::draw_text2(&shortcut.about,
                             w.x() + padding_x + padding_about,
                             w.y() + line_y,
                             w.w(), w.h(),
                             Align::TopLeft);
            
            // Increment to the next line
            line_y += h_about
        }
    });
    
    // Return the widget
    return widget;
}


fn read_json_config(path: String) -> Config {
    let file = File::open(path).unwrap();
    let buf_reader = BufReader::new(file);
    serde_json::from_reader(buf_reader).unwrap() 
}


fn rgb(hex_str: &str) -> Color {
    let hex_trim = hex_str.trim_start_matches("#");
    let parsed = u32::from_str_radix(hex_trim, 16).unwrap();
    Color::from_u32(parsed)
}


fn main() {
    let args = Args::parse(); 

    let config = read_json_config(args.config);
    
    let app = app::App::default();

    // Create the application's window
    let mut window = Window::default()
        .with_pos(0, 0)
        .with_size(config.width, config.height);
    window.set_border(false);
    window.make_resizable(false);
    window.set_color(rgb(&config.br_col));

    // Create flexbox on the X axis
    let mut flexbox_x = Flex::default_fill().row();

    // Since the window is of border color
    // Margin & padding must be of border size
    flexbox_x.set_margin(config.border_size);
    flexbox_x.set_pad(config.border_size);
    
    // Variables for drawing
    let mut rel_y;
    let mut rel_h;
    let mut count;
    let mut amount;

    // Loop over columns & draw all blocks
    for column in &config.columns {
        rel_y = 0;
        count = 0;
        amount = column.len();

        let flexbox_y = Flex::default_fill().column();

        for block in column {
            rel_y += config.border_size;
            
            // If this is the last block
            // Account for the final border size
            count += 1;
            if count == amount {
                rel_h = (block.rel_size * config.height as f64) as i32;
                rel_h -= config.border_size * 2;
            } else {
                rel_h = (block.rel_size * config.height as f64) as i32;
                rel_h -= config.border_size;
            }
            
            draw_block(block.clone(), &config, rel_y, rel_h);
            rel_y += rel_h;
        }

        flexbox_y.end();
    } 
    flexbox_x.end();

    window.end();
    window.show();
    window.set_opacity(config.opacity);
    app.run().unwrap();
}
