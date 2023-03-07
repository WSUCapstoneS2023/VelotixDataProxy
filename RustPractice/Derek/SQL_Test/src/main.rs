// sql
use rusqlite::{params, Connection, Result};

// display
use druid::widget::{Align, Flex, Label, TextBox, Button};
use druid::{AppLauncher, Data, Env, Lens, LocalizedString, Widget, WindowDesc, WidgetExt};

#[derive(Debug)]
struct User {
    id: i32,
    name: String,
    email: String,
}

fn main() -> Result<()> {
    // Open a connection to a new or existing SQLite database file
    let conn = set_conn((&"example.db").to_string())?;

    // Create a new table named `users`
    conn.execute(
        "CREATE TABLE IF NOT EXISTS users (
                  id              INTEGER PRIMARY KEY,
                  name            TEXT NOT NULL,
                  email           TEXT NOT NULL
                  );",
        [],
    )?;

    let me = User {
        id: 0,
        name: "Derek".to_string(),
        email: "Sadler_Derek@comcast.net".to_string(),
    };

    conn.execute(
        "
        INSERT OR IGNORE INTO users (id, name, email) 
        VALUES (?1, ?2, ?3)", 
        (&me.id, &me.name, &me.email)
    )?;

    let mut stmt = conn.prepare("SELECT id, name, email FROM users")?;
    let person_iter = stmt.query_map([], |row| {
        Ok(User {
            id: row.get(0)?,
            name: row.get(1)?,
            email: row.get(2)?,
        })
    })?;

    for person in person_iter {
        println!("Found person {:?}", person.unwrap());
    }


    // describe the main window
    let main_window = WindowDesc::new(build_root_widget())
        .title("SQL Test")
        .window_size((400.0, 400.0));

    let data = "Hello World".to_string();

    // start the application
    AppLauncher::with_window(main_window)
        .launch(data)
        .expect("Failed to launch application");

    Ok(())
}

// Opens a database and returns the connection
fn set_conn(database: String) ->  Result<Connection>
{
    // Open a connection to a new or existing SQLite database file
    let conn = Connection::open(database)?;

    // Return the connection
    Ok(conn)
}


fn build_root_widget() -> impl Widget<String> {
    Flex::column()
        // Add a label widget
        .with_child(Label::new(|data: &String, _env: &_| data.clone()))
        // Add a button widget to the Flex widget
        .with_child(Button::new("Click me!").on_click(|_, _, _| {
            println!("Button clicked!");
        }))
}