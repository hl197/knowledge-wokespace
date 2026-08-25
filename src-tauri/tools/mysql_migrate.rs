use keyring::Entry;
use mysql::{params, prelude::Queryable, Opts, Pool};
use rusqlite::Connection;
use sha2::{Digest, Sha256};
use std::{fs, path::Path};

const DATA_ROOT: &str = r"D:\工作台数据";
const CREDENTIAL_SERVICE: &str = "HermesWorkbench/MySQL";
const MYSQL_USER: &str = "root";
const DATABASE: &str = "personal_ai_workbench";

fn mysql_pool() -> Result<Pool, String> {
    let password = Entry::new_with_target(CREDENTIAL_SERVICE, CREDENTIAL_SERVICE, MYSQL_USER)
        .map_err(|e| format!("credential entry: {e}"))?
        .get_password()
        .map_err(|e| format!("credential read: {e}"))?;
    let opts = mysql::OptsBuilder::new()
        .ip_or_hostname(Some("127.0.0.1"))
        .tcp_port(3306)
        .user(Some(MYSQL_USER))
        .pass(Some(password))
        .db_name(Some(DATABASE));
    Pool::new(Opts::from(opts)).map_err(|e| format!("mysql pool: {e}"))
}

fn sha256(bytes: &[u8]) -> String { format!("{:x}", Sha256::digest(bytes)) }
fn unix_to_mysql(value: &str) -> i64 { value.parse::<i64>().unwrap_or(0) }

fn migrate_cards(sqlite: &Connection, mysql: &mut mysql::PooledConn) -> Result<(usize, usize), String> {
    let mut stmt = sqlite.prepare("SELECT id,title,summary,card_type,tags,source,source_path,visibility,status,favorite,created_at,updated_at,deleted_at,content_path FROM cards").map_err(|e| e.to_string())?;
    let mut rows = stmt.query([]).map_err(|e| e.to_string())?;
    let mut cards = 0usize;
    let mut contents = 0usize;
    while let Some(row) = rows.next().map_err(|e| e.to_string())? {
        let id: String = row.get(0).map_err(|e| e.to_string())?;
        let content_path: String = row.get(13).map_err(|e| e.to_string())?;
        let content = fs::read_to_string(&content_path).unwrap_or_default();
        let created = unix_to_mysql(&row.get::<_, String>(10).map_err(|e| e.to_string())?);
        let updated = unix_to_mysql(&row.get::<_, String>(11).map_err(|e| e.to_string())?);
        mysql.exec_drop(r#"INSERT INTO cards
            (id,title,summary,card_type,tags,source,source_path,visibility,status,favorite,created_at,updated_at,deleted_at)
            VALUES (:id,:title,:summary,:card_type,:tags,:source,:source_path,:visibility,:status,:favorite,
                    FROM_UNIXTIME(:created),FROM_UNIXTIME(:updated),
                    IF(:deleted IS NULL,NULL,FROM_UNIXTIME(:deleted)))
            ON DUPLICATE KEY UPDATE id=id"#, params! {
            "id" => id.clone(), "title" => row.get::<_, String>(1).map_err(|e| e.to_string())?,
            "summary" => row.get::<_, String>(2).map_err(|e| e.to_string())?,
            "card_type" => row.get::<_, String>(3).map_err(|e| e.to_string())?,
            "tags" => row.get::<_, String>(4).map_err(|e| e.to_string())?,
            "source" => row.get::<_, String>(5).map_err(|e| e.to_string())?,
            "source_path" => row.get::<_, Option<String>>(6).map_err(|e| e.to_string())?,
            "visibility" => row.get::<_, String>(7).map_err(|e| e.to_string())?,
            "status" => row.get::<_, String>(8).map_err(|e| e.to_string())?,
            "favorite" => row.get::<_, i64>(9).map_err(|e| e.to_string())? != 0,
            "created" => created, "updated" => updated,
            "deleted" => row.get::<_, Option<String>>(12).map_err(|e| e.to_string())?.map(|v| unix_to_mysql(&v)),
        }).map_err(|e| format!("card {id}: {e}"))?;
        mysql.exec_drop(r#"INSERT INTO card_contents (card_id,content,content_sha256,updated_at)
            VALUES (:id,:content,:hash,FROM_UNIXTIME(:updated))
            ON DUPLICATE KEY UPDATE card_id=card_id"#, params! {
            "id" => id, "content" => content.clone(), "hash" => sha256(content.as_bytes()), "updated" => updated,
        }).map_err(|e| format!("content: {e}"))?;
        cards += 1;
        contents += 1;
    }
    Ok((cards, contents))
}

fn migrate_versions(sqlite: &Connection, mysql: &mut mysql::PooledConn) -> Result<usize, String> {
    let mut stmt = sqlite.prepare("SELECT id,card_id,title,summary,tags,status,content,created_at FROM card_versions").map_err(|e| e.to_string())?;
    let rows = stmt.query_map([], |row| Ok((row.get::<_, i64>(0)?, row.get::<_, String>(1)?, row.get::<_, String>(2)?, row.get::<_, String>(3)?, row.get::<_, String>(4)?, row.get::<_, String>(5)?, row.get::<_, String>(6)?, row.get::<_, String>(7)?))).map_err(|e| e.to_string())?;
    let mut count = 0;
    for row in rows {
        let (id, card_id, title, summary, tags, status, content, created_at) = row.map_err(|e| e.to_string())?;
        mysql.exec_drop(r#"INSERT INTO card_versions (id,card_id,title,summary,tags,status,content,content_sha256,created_at)
            VALUES (:id,:card_id,:title,:summary,:tags,:status,:content,:hash,FROM_UNIXTIME(:created))
            ON DUPLICATE KEY UPDATE id=id"#, params! {
            "id" => id, "card_id" => card_id, "title" => title, "summary" => summary, "tags" => tags,
            "status" => status, "content" => content.clone(), "hash" => sha256(content.as_bytes()), "created" => unix_to_mysql(&created_at),
        }).map_err(|e| format!("version {id}: {e}"))?;
        count += 1;
    }
    Ok(count)
}

fn migrate_relations(sqlite: &Connection, mysql: &mut mysql::PooledConn) -> Result<usize, String> {
    let mut stmt = sqlite.prepare("SELECT from_card_id,to_card_id,relation_type,created_at FROM card_relations").map_err(|e| e.to_string())?;
    let rows = stmt.query_map([], |row| Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?, row.get::<_, String>(2)?, row.get::<_, String>(3)?))).map_err(|e| e.to_string())?;
    let mut count = 0;
    for row in rows {
        let (from_id, to_id, relation_type, created_at) = row.map_err(|e| e.to_string())?;
        mysql.exec_drop(r#"INSERT INTO card_relations (from_card_id,to_card_id,relation_type,created_at)
            VALUES (:from_id,:to_id,:relation_type,FROM_UNIXTIME(:created))
            ON DUPLICATE KEY UPDATE from_card_id=from_card_id"#, params! { "from_id" => from_id, "to_id" => to_id, "relation_type" => relation_type, "created" => unix_to_mysql(&created_at) }).map_err(|e| e.to_string())?;
        count += 1;
    }
    Ok(count)
}

fn migrate_audit(sqlite: &Connection, mysql: &mut mysql::PooledConn) -> Result<usize, String> {
    let mut stmt = sqlite.prepare("SELECT actor,action,target_id,detail,created_at FROM audit_log").map_err(|e| e.to_string())?;
    let rows = stmt.query_map([], |row| Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?, row.get::<_, Option<String>>(2)?, row.get::<_, Option<String>>(3)?, row.get::<_, String>(4)?))).map_err(|e| e.to_string())?;
    let mut count = 0;
    for row in rows {
        let (actor, action, target_id, detail, created_at) = row.map_err(|e| e.to_string())?;
        mysql.exec_drop(r#"INSERT INTO audit_log (actor,action,target_id,detail,created_at)
            VALUES (:actor,:action,:target_id,:detail,FROM_UNIXTIME(:created))"#, params! { "actor" => actor, "action" => action, "target_id" => target_id, "detail" => detail, "created" => unix_to_mysql(&created_at) }).map_err(|e| e.to_string())?;
        count += 1;
    }
    Ok(count)
}

fn migrate_files(mysql: &mut mysql::PooledConn) -> Result<usize, String> {
    let root = Path::new(DATA_ROOT);
    let mut count = 0;
    for (folder, kind) in [("originals", "original"), ("backups", "backup")] {
        let dir = root.join(folder);
        if !dir.exists() { continue; }
        for entry in fs::read_dir(dir).map_err(|e| e.to_string())? {
            let path = entry.map_err(|e| e.to_string())?.path();
            if !path.is_file() { continue; }
            let bytes = fs::read(&path).map_err(|e| e.to_string())?;
            let id = format!("{}-{}", kind, sha256(&bytes));
            let file_name = path.file_name().and_then(|v| v.to_str()).unwrap_or("file").to_string();
            mysql.exec_drop(r#"INSERT INTO file_objects (id,object_kind,file_name,mime_type,size_bytes,sha256,content,created_at)
                VALUES (:id,:kind,:name,NULL,:size,:hash,:content,NOW(6))
                ON DUPLICATE KEY UPDATE id=id"#, params! { "id" => id, "kind" => kind, "name" => file_name, "size" => bytes.len() as u64, "hash" => sha256(&bytes), "content" => bytes }).map_err(|e| e.to_string())?;
            count += 1;
        }
    }
    Ok(count)
}

fn main() -> Result<(), String> {
    let sqlite = Connection::open(Path::new(DATA_ROOT).join("workbench.db")).map_err(|e| format!("sqlite: {e}"))?;
    let pool = mysql_pool()?;
    let mut mysql = pool.get_conn().map_err(|e| e.to_string())?;
    mysql.query_drop("START TRANSACTION").map_err(|e| e.to_string())?;
    let result = (|| {
        let cards = migrate_cards(&sqlite, &mut mysql)?;
        let versions = migrate_versions(&sqlite, &mut mysql)?;
        let relations = migrate_relations(&sqlite, &mut mysql)?;
        let audit = migrate_audit(&sqlite, &mut mysql)?;
        let files = migrate_files(&mut mysql)?;
        Ok::<_, String>((cards, versions, relations, audit, files))
    })();
    match result {
        Ok((cards, versions, relations, audit, files)) => {
            mysql.query_drop("COMMIT").map_err(|e| e.to_string())?;
            println!("migration_ok cards={} contents={} versions={} relations={} audit={} files={}", cards.0, cards.1, versions, relations, audit, files);
            Ok(())
        }
        Err(error) => {
            let _ = mysql.query_drop("ROLLBACK");
            Err(format!("migration_rolled_back: {error}"))
        }
    }
}
