// Standard stuff
use std::string::String; 

// FLTK UI framework
use fltk::{ app, draw, prelude::* };
use fltk::enums::{ Color, Align, Font, FrameType };
use fltk::window::Window;
use fltk::group::Flex;
use fltk::widget::Widget;


// Local crates
// Configuration
mod config;


/// Creates a shortcut block
/// 
/// # Arguments
///
/// * `program`     - A string of the program name
/// * `shortcuts`   - An entry type of all shortcuts for the program
/// * `config`      - The configuration options object
/// 
/// Returns a `Widget` object
fn draw_block(program: String, shortcuts: config::Entry, config: &config::Config) -> Widget {
    // Create a mutable widget
    // Value that will be returned
    let mut widget = Widget::default();

    // Padding values in pixels
    let padding_x: i32 = 8;
    let padding_y: i32 = 0;

    // Get configuration from borrowed config object
    let bg_color = config.colors.background;
    let fg_color = config.colors.foreground;
    let sh_color = config.colors.shrot;
    let ab_color = config.colors.about;

    // Sets the custom draw method
    widget.draw(move |w| {
        // Create a frame with a background color
        draw::draw_box(FrameType::FlatBox, w.x(), w.y(), w.w(), w.h(), Color::from_u32(bg_color));
        
        // Calculate the drawed header size
        draw::set_font(Font::HelveticaBold, 24);
        let (_w_header, h_header) = draw::measure(&program, false);
        
        // Draw the header
        draw::set_draw_color(Color::from_u32(fg_color));
        draw::draw_text2(&program,
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
        for shortcut in &shortcuts {
            // Get shortcut text and length
            let s = shortcut["bind"].clone();
            let len = s.len() as i32;
            // Check if length is greater than current maximum
            if len > max_len {
                // If so asign the length and shortcut
                max_len = len;
                max_shortcut = s;
            }
        }
        

        // Calculate the padding for the about section
        // It is equal to the sum of the maximum shortcut width and spacing
        draw::set_font(Font::HelveticaBold, 12);
        let padding_about: i32 = draw::width2(&max_shortcut.as_str(), max_len).ceil() as i32 + 32;


        // Draw shortcuts and abouts
        for shortcut in &shortcuts {
            //
            // TODO: Implement error parsing
            //

            // Draw the shortcut text
            draw::set_draw_color(Color::from_u32(sh_color));
            draw::draw_text2(&shortcut["bind"],
                             w.x() + padding_x,
                             w.y() + line_y,
                             w.w(), w.h(),
                             Align::TopLeft);

            // Get the about texts height
            // It is assumend that shortcuts do not contain newlines
            let (_w_about, h_about) = draw::measure(&shortcut["about"], false);
            // Draw the about text
            draw::set_draw_color(Color::from_u32(ab_color));
            draw::draw_text2(&shortcut["about"],
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


/// Creates an empty block widget
/// Used if there are more columns than widgets
/// 
/// # Arguments
///
/// * `config`  - The configuration options object
/// 
/// Returns a `Widget` object
fn draw_empty_block(config: &config::Config) -> Widget {
    // Create a mutable widget
    // Value that will be returned
    let mut widget = Widget::default();
    
    // Get the background color from borrowed config
    let bg_color = config.colors.background;

    // Sets the custom draw method
    widget.draw(move |w| {
        // Create a frame with a background color
        draw::draw_box(FrameType::FlatBox,
                       w.x(), w.y(),
                       w.w(), w.h(),
                       Color::from_u32(bg_color));
    });

    // Return the widget
    return widget;
}


/// The application's entrypoint
fn main() {
    // Create the application
    let app = app::App::default();

    let config = config::get_config();

    // Set width and height variables from the config
    let width = config.general.width;
    let height = config.general.height;


    // Create the application's window
    let mut window = Window::default()
        .with_pos(0, 0)
        .with_size(width, height);
    window.set_border(false);
    window.make_resizable(false);
    window.set_color(Color::from_u32(config.colors.border));
    

    // Create flexbox on the X axis
    let mut flexbox_x = Flex::default_fill()
        .row();

    // Set the margin & padding to match the configuration
    flexbox_x.set_margin(config.general.border);
    flexbox_x.set_pad(config.general.border);
    
    
    // Get the column amount and all of the shortcuts
    let mut widgets = config.shrots.len();
    let mut shrots = config.shrots.clone();
    
    // Calculate the total amount of rows needed
    // Depends on the widgets needed and the column amount
    let rows = (widgets as f64 / config.general.columns as f64).ceil() as i32;
    

    //
    // Loop over the amount of columns
    // Create a flexbox on the Y axis
    //
    for _ in 0..config.general.columns {
        // Create a flexbox on the Y axis
        let mut flexbox_y = Flex::default_fill()
            .column();

        // Set margin & padding
        // The padding matches the configuration
        flexbox_y.set_margin(0);
        flexbox_y.set_pad(config.general.border);

        // Check if all widgets have been created
        // If so create an empty widget and break the loop
        if widgets == 0 {
            draw_empty_block(&config);
            break;
        }

        //
        // Loop over the amount of rows
        // Fill the rows with shortcut widget blocks
        //
        for _ in 0..rows {
            // Check if all widgets have been created
            // If so break the loop
            if widgets == 0 {
                break;
            }
            
            // Pop a shrot of the BTreeMap
            // Ensures that all widgets are displayed once in the same order
            let (program, shortcuts) = shrots.pop_first().unwrap();
            // Create the widget
            draw_block(program, shortcuts, &config);
            // Decrement widgets
            widgets -= 1;
        } 
        
        // Close the Y flexbox for this column
        flexbox_y.end();
    }
    
    // Close the X flexbox
    flexbox_x.end();

    //
    // Finish the window setup
    // Set the opacity from the config
    //
    window.end();
    window.show();
    window.set_opacity(config.general.opacity);
    app.run().unwrap();

}
