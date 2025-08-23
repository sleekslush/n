use rusqlite::Connection;

pub fn migrate<'a>(connection: &'a mut Connection) {
    let migration_version = get_migration_version(connection);
    if migration_version >= MIGRATIONS.len() {
        return;
    }

    let tx = connection.transaction().unwrap();

    MIGRATIONS
        .iter()
        .rev()
        .skip(migration_version)
        .for_each(|sql| {
            tx.execute_batch(sql).unwrap();
        });

    tx.execute(&format!("PRAGMA user_version = {}", MIGRATIONS.len()), [])
        .unwrap();

    tx.commit().unwrap();
}

pub fn get_migration_version(connection: &Connection) -> usize {
    connection
        .query_row("PRAGMA user_version", [], |row| row.get(0))
        .unwrap_or(0)
}

// Groups of migrations execute from bottom to top
const MIGRATIONS: &[&str] = &[include_str!("migrations/001-initial.sql")];
