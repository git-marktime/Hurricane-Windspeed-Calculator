use std::io::{self, Write};
use colored::Colorize;
use crossterm::{cursor, terminal, execute};

fn main() {
    println!("HURRICANE SIMULATOR");
    println!("---------------------");

    execute!(io::stdout(), terminal::SetTitle("Hurricane Windspeed Predictor")).expect("bad");
    execute!(io::stdout(), cursor::MoveTo(0,3)).expect("bad");
    execute!(io::stdout(), cursor::SavePosition).expect("bad");
    execute!(io::stdout(), cursor::MoveTo(0,2)).expect("bad");

    loop {
        execute!(
            io::stdout(),
            cursor::MoveTo(0,2)
        ).expect("bad");
        
        let windspeed = getinput("Input windspeed (MPH): ");
        let shear = getinput("Input shear: ");
        let pressure = getinput("Input pressure: ");

        println!();

        let h1 = predict_windspeed(windspeed, shear, pressure, 1.0);
        let h5 = predict_windspeed(windspeed, shear, pressure, 5.0);
        let h10 = predict_windspeed(windspeed, shear, pressure, 10.0);
        let d1 = predict_windspeed(windspeed, shear, pressure, 24.0);
        let d7 = predict_windspeed(windspeed, shear, pressure, 168.0);

        println!("After 1 hour: {} MPH ({})                   ", h1.to_string().color(colorhandler(h1)), categoryhandler(h1));
        println!("After 5 hours: {} MPH ({})                  ", h5.to_string().color(colorhandler(h5)), categoryhandler(h5));
        println!("After 10 hours: {} MPH ({})                 ", h10.to_string().color(colorhandler(h10)), categoryhandler(h10));
        println!("After 1 day: {} MPH ({})                    ", d1.to_string().color(colorhandler(d1)), categoryhandler(d1));
        println!("After 7 days: {} MPH ({})                   ", d7.to_string().color(colorhandler(d7)), categoryhandler(d7));
        println!();
    }
}

fn getinput(prompt: &str) -> f32 {
    println!("{prompt}");
    {
        let pos = cursor::position().unwrap();
        print!("                        ");
        io::stdout().flush().expect("bad");
        execute!(io::stdout(), cursor::MoveTo(pos.0, pos.1)).expect("bad");
    }
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("crap");
        let input = input.trim();
        if input.parse::<f32>().is_ok() {
            return input.parse::<f32>().unwrap();
        } else {
            execute!(io::stdout(), cursor::MoveUp(1)).expect("bad");
            let pos = cursor::position().unwrap();
            print!("                        ");
            io::stdout().flush().expect("bad");
            execute!(io::stdout(), cursor::MoveTo(pos.0, pos.1)).expect("bad");
        }
    }
}

fn colorhandler(windspeed: f32) -> colored::Color {
    if windspeed >= 200.0 {
        colored::Color::Magenta
    } else if windspeed >= 150.0 {
        colored::Color::BrightMagenta
    } else if windspeed >= 100.0 {
        colored::Color::Red
    } else if windspeed >= 80.0 {
        colored::Color::BrightRed
    } else if windspeed >= 50.0 {
        colored::Color::BrightYellow
    } else if windspeed >= 20.0 {
        colored::Color::Green
    } else {
        colored::Color::BrightGreen
    }
}

fn categoryhandler(windspeed: f32) -> String {
    if windspeed >= 157.0 {
        "Category 5".to_string()
    } else if windspeed >= 130.0 {
        "Category 4".to_string()
    } else if windspeed >= 111.0 {
        "Category 3".to_string()
    } else if windspeed >= 96.0 {
        "Category 2".to_string()
    } else if windspeed >= 74.0 {
        "Category 1".to_string()
    } else if windspeed >= 39.0 {
        "Tropical Storm".to_string()
    } else if windspeed > 1.0 {
        "Tropical Depression".to_string()
    } else {
        "Dead".to_string()
    }
}

fn predict_windspeed(windspeed: f32, shear: f32, pressure: f32, time: f32) -> f32 {
    let decrease_constant = 10.0;
    let pressure_factor = (pressure - 1013.25) / 100.0; // adjust for pressure
    let rateofchange = if shear <= 0.325 {
        (-3.0*((shear - 0.4).powi(3)))+0.5
    } else {
        (-decrease_constant*((shear - 0.325).powi(2)))+0.5
    };
    let max_change = 1.0 + pressure_factor; // adjust for pressure
    let delta = rateofchange * time * max_change; // adjust for pressure
    let result = windspeed + delta;

    if result > 0.0 {
        result
    } else {
        0.0
    }
}