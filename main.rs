mod weather;
mod api;

use std::io;

use api::get_weather;


// ========================================
// GET USER INPUT
// ========================================

fn get_input(prompt: &str) -> String {

    println!("{}", prompt);

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    input.trim().to_string()
}


// ========================================
// MAIN
// ========================================

fn main() {

    println!();
    println!("========================================");
    println!("          🌤️ WEATHER CLI");
    println!("========================================");

    loop {

        let city =
            get_input(
                "\nEnter a city (or type 'exit' to quit):"
            );


        // --------------------------------
        // EXIT PROGRAM
        // --------------------------------

        if city.to_lowercase() == "exit" {

            println!();
            println!(
                "Thank you for using Weather CLI!"
            );

            println!("Goodbye! 👋");

            break;
        }


        // --------------------------------
        // VALIDATE INPUT
        // --------------------------------

        if city.is_empty() {

            println!(
                "Please enter a city name."
            );

            continue;
        }


        // --------------------------------
        // FETCH WEATHER
        // --------------------------------

        println!();
        println!(
            "Fetching weather for {}...",
            city
        );


        match get_weather(&city) {

            Ok(weather) => {

                weather.display();
            }


            Err(error) => {

                println!();
                println!(
                    "❌ Error: {}",
                    error
                );
            }
        }
    }
}