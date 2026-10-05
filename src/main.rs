use rusqlite::Result;
use std::io;
mod database;
mod input;

fn main() -> Result<()> {
    let connection = database::connect_database()?;
    database::initialize_database(&connection)?;

    println!("a -> Add New Password \n u -> Update Password \n g -> Get Password \n d -> Delete Password");

    let mut user_query = String::new();
    io::stdin()
        .read_line(&mut user_query)
        .expect("Failed to get input!");
    let user_query = user_query.trim();

    match user_query {
        "a" => {
            let website = input::gimmi("Website");
            let username = input::gimmi("Username");
            let password = input::gimmi("Password");

            database::store_new_password(&username, &website, &password, &connection)?;
        }

        "u" => {
            let website = input::gimmi("Website");
            let username = input::gimmi("Username");
            let new_password = input::gimmi("New Password");

            database::update_password(&username, &website, &new_password, &connection)?;
        }

        "g" => {
            let website = input::gimmi("Website");
            let username: String = input::gimmi("Username");

            let retrieved_password = database::get_password(&username, &website, &connection);
            match retrieved_password {
                Ok(password) => println!("Retrieved Password: {password}"),
                Err(error) => println!("Failed to retrieve password: {error}"),
            }
        }

        "d" => {
            let website = input::gimmi("Website");
            let username: String = input::gimmi("Username");

            database::delete_password(&username, &website, &connection)?;
        }

        _ => {
            println!("Invalid command");
        }
    }

    Ok(())
}
