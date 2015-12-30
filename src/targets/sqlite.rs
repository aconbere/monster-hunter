use rusqlite::Connection;
use rawsql::Loader;

const TABLES: &'static [ &'static str ] = ["users", "game_data", "user_features"];

fn create_tables(conn:Connection, tables:[str]) {
    let queries = Loader::get_queries_from("examples/postgre.sql").unwrap().queries;
    for table in tables {
        let create = queries.get(format!("create_{}", table)).unwrap();
        conn.execute(create, &[]).unwrap()
    }
}

pub fn export(character:Character) {
    let conn = Connection::open_in_memory().unwrap();

    create_tables(conn, TABLES)
}
