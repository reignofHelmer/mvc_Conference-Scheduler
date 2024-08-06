use actix_web::{App, HttpServer, web};
use diesel::r2d2::{self, ConnectionManager};
use diesel::SqliteConnection;
use dotenv::dotenv;
use std::env;
use std::io::{self, Write};
use std::str::FromStr;

mod models;
mod controllers;
mod schema;

type DbPool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

fn main() -> std::io::Result<()> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<SqliteConnection>::new(&database_url);
    let pool = r2d2::Pool::builder().build(manager).expect("Failed to create pool.");
    let server = HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .route("/sessions", web::get().to(controllers::list_sessions))
            .route("/sessions", web::post().to(controllers::add_session))
            .route("/sessions/{id}", web::delete().to(controllers::remove_session))
            .route("/sessions/{id}", web::put().to(controllers::modify_session))
    })
    .bind("127.0.0.1:8080")?
    .run();

    actix_rt::System::new().block_on(server);
    // User interaction loop
    loop {
        println!("Please choose an operation:");
        println!("1. Add Session");
        println!("2. Delete Session");
        println!("3. Update Session");
        println!("4. Exit");

        let mut input = String::new();
        print!("Enter your choice: ");
        io::stdout().flush().unwrap(); // Ensure prompt is printed before user input
        io::stdin().read_line(&mut input).expect("Failed to read input");
        let choice = input.trim();

        match choice {
            "1" => {
                // Handle adding a session
                println!("Add Session selected.");
                // Here you would typically call a function to handle the add session logic
            },
            "2" => {
                // Handle deleting a session
                println!("Delete Session selected.");
                // Here you would typically call a function to handle the delete session logic
            },
            "3" => {
                // Handle updating a session
                println!("Update Session selected.");
                // Here you would typically call a function to handle the update session logic
            },
            "4" => {
                // Confirm exit
                println!("Are you sure you want to exit? (yes/no)");
                io::stdin().read_line(&mut input).expect("Failed to read input");
                let confirm_exit = input.trim().to_lowercase();
                if confirm_exit == "yes" {
                    println!("Exiting the application...");
                    break;
                } else {
                    println!("Continuing...");
                }
            },
            _ => {
                println!("Invalid choice. Please enter a number between 1 and 4.");
            }
        }

        input.clear(); // Clear the input buffer for the next iteration
    }

    server_handle.abort(); // Stop the server task
    println!("Server stopped.");

    Ok(())
}