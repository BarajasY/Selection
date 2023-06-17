use postgres::{Client, NoTls};

pub fn test() {
    //SETTING ENV VAR
    let connection_string:String = dotenvy::var("CONN_STR").expect("Error in env var");

    // CONNECTING TO PSQL USING PREVIOUS VARIABLE.
    let mut client = Client::connect(&connection_string, NoTls).expect("Error");

    // CREATING TABLE (COMMENTED OUT BECAUSE IT ALREADY EXISTS)
/*     client.batch_execute("
        CREATE TABLE test (
            id SERIAL PRIMARY KEY,
            name TEXT NOT NULL
        )
    ").expect("Error creating sql "); */

    let name: &str = "Corvette";

    client.execute("INSERT INTO test (name) VALUES ($1)", &[&name]).expect("Error executing sql");

    for row in client.query("SELECT * FROM test", &[]).expect("Error executing query") {
        let id: i32 = row.get(0);
        let name: &str = row.get(1);
        println!("{} {}", id, name);
    }
}
