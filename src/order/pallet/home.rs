use console_engine::{
    events::Event,
    forms::{Checkbox, Form, FormField, FormOptions, FormStyle, FormValue, Radio, Text},
    rect_style::BorderStyle,
    ConsoleEngine, KeyCode, KeyModifiers,
};
use crossterm::event::KeyEvent;

use std::collections::HashMap;

// fn main() {
pub(crate) fn doit() {
    // Initialize the engine
    let mut engine = ConsoleEngine::init(48, 24, 5).unwrap();

    // Define a theme for the form
    let theme = FormStyle {
        border: Some(BorderStyle::new_light()),
        ..Default::default()
    };

    // Create a new Form with two text inputs in it
    let mut form = Form::new(
        42,
        22,
        FormOptions {
            style: theme,
            ..Default::default()
        },
    );
    // you either need to create your form entry directly from add_field ...
    // (We don't care about the width of our input, since it'll be resized inside the form)
    form.add_field(
        "first_name",
        Text::new(
            0,
            FormOptions {
                style: theme,
                label: Some("First Name"),
                ..Default::default()
            },
        ),
    );
    // ... or let the form build it for you
    form.build_field::<Text>(
        "last_name",
        FormOptions {
            style: theme,
            label: Some("Last Name"),
            ..Default::default()
        },
    );

    let check_choices = vec![
        String::from("First"),
        String::from("Second"),
        String::from("Third"),
    ];

    form.build_field::<Checkbox>(
        "checkbox",
        FormOptions {
            style: theme,
            label: Some("Please select something"),
            custom: HashMap::from([(
                String::from("choices"),
                FormValue::List(check_choices.clone()),
            )]),
            ..Default::default()
        },
    );
    form.build_field::<Radio>(
        "radio",
        FormOptions {
            style: theme,
            label: Some("Do you enjoy this demo?"),
            custom: HashMap::from([(
                String::from("choices"),
                FormValue::List(vec![String::from("Yes"), String::from("No")]),
            )]),
            ..Default::default()
        },
    );

    form.set_active(true);

    while !form.is_finished() {
        // loop {
        // Poll next event
        match engine.poll() {
            // A frame has passed
            Event::Frame => {
                engine.clear_screen();
                engine.print_screen(0, 0, form.draw((engine.frame_count % 8 > 3) as usize));
                // engine.print_screen(0, 0, form.draw((engine.frame_count % 8 > 3) as usize));
                engine.draw();
            }

            // exit with Escape
            Event::Key(KeyEvent {
                code: KeyCode::Esc,
                modifiers: _,
                kind: _,
                state: _,
            }) => {
                break;
            }

            // exit with CTRL+C
            Event::Key(KeyEvent {
                code: KeyCode::Char('c'),
                modifiers: KeyModifiers::CONTROL,
                kind: _,
                state: _,
            }) => {
                break;
            }
            // Let the form handle the unhandled events
            event => form.handle_event(event),
        }
    }

    // we don't need the engine anymore, dropping it will close the fullscreen mode and bring us back to our terminal
    drop(engine);

    if form.is_finished() {
        let mut first_name = String::new();
        let mut last_name = String::new();

        // Get the output of each fields
        if let Ok(FormValue::String(name)) = form.get_validated_field_output("first_name") {
            first_name = name;
        }
        if let Ok(FormValue::String(name)) = form.get_validated_field_output("last_name") {
            last_name = name;
        }

        println!("Hello, {} {}!", first_name, last_name);
    } else {
        println!("Form cancelled");
    }
}
/////////////////////
