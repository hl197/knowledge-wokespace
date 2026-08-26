use rusqlite::{params, Connection};
use mysql::{Opts, Pool, PooledConn};
use mysql::prelude::Queryable;
use sha2::Digest;
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::{Read, Write};
use std::process::Command;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};
use tiny_http::{Header, Method, Request, Response, Server};

const DATA_ROOT: &str = r"D:\工作台数据";

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NewCard {
    pub title: String,
    #[serde(default)] pub summary: String,
    #[serde(rename = "type", alias = "cardType")] pub card_type: String,
    #[serde(default)] pub tags: Vec<String>,
    #[serde(default)] pub source: String,
    #[serde(default = "default_visibility")] pub visibility: String,
    #[serde(default = "default_status")] pub status: String,
    #[serde(default)] pub content: String,
    #[serde(default)] pub actor: String,
}

#[derive(Debug, Deserialize, Default)]
struct CardPatch {
    title: Option<String>, summary: Option<String>, tags: Option<Vec<String>>,
    status: Option<String>, favorite: Option<bool>, content: Option<String>,
    #[serde(alias = "_actor")] actor: Option<String>,
}

#[derive(Debug, Deserialize)]
struct ImportRequest { path: String, #[serde(default)] actor: String }

fn default_visibility() -> String { "本机助手可读".into() }
fn default_status() -> String { "草稿".into() }
const MYSQL_CREDENTIAL_SERVICE: &str = "HermesWorkbench/MySQL";
const MYSQL_USER: &str = "root";
const MYSQL_DATABASE: &str = "personal_ai_workbench";
fn data_root() -> PathBuf { PathBuf::from(DATA_ROOT) }

fn mysql_password() -> Result<String, String> {
    keyring::Entry::new_with_target(MYSQL_CREDENTIAL_SERVICE, MYSQL_CREDENTIAL_SERVICE, MYSQL_USER)
        .map_err(|e| format!("创建 Windows 凭据条目失败: {e}"))?
        .get_password()
        .map_err(|e| format!("读取 Windows 凭据失败，请检查 {MYSQL_CREDENTIAL_SERVICE}: {e}"))
}

fn mysql_connection() -> Result<PooledConn, String> {
    let password = mysql_password()?;
    let builder = mysql::OptsBuilder::new()
        .ip_or_hostname(Some("127.0.0.1"))
        .tcp_port(3306)
        .user(Some(MYSQL_USER))
        .pass(Some(password))
        .db_name(Some(MYSQL_DATABASE));
    Pool::new(Opts::from(builder))
        .map_err(|e| format!("创建 MySQL 连接池失败: {e}"))?
        .get_conn()
        .map_err(|e| format!("连接 MySQL 失败: {e}"))
}

fn now_string() -> String { chrono_like_now().to_string() }
fn chrono_like_now() -> u64 { SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default().as_secs() }
fn json_response(value: serde_json::Value, status: u16) -> Response<std::io::Cursor<Vec<u8>>> {
    Response::from_string(value.to_string()).with_status_code(status)
        .with_header(Header::from_bytes("Content-Type", "application/json; charset=utf-8").unwrap())
        .with_header(Header::from_bytes("Access-Control-Allow-Origin", "*").unwrap())
        .with_header(Header::from_bytes("Access-Control-Allow-Methods", "GET, POST, PATCH, DELETE, OPTIONS").unwrap())
        .with_header(Header::from_bytes("Access-Control-Allow-Headers", "Content-Type").unwrap())
}
fn error_response(message: &str, status: u16) -> Response<std::io::Cursor<Vec<u8>>> { json_response(serde_json::json!({"error": message}), status) }

fn db_connection() -> Result<Connection, String> {
    let root = data_root();
    for folder in ["cards", "originals", "assets", "inbox", "exports"] { fs::create_dir_all(root.join(folder)).map_err(|e| format!("创建数据目录失败: {e}"))?; }
    let db = Connection::open(root.join("workbench.db")).map_err(|e| format!("打开 SQLite 失败: {e}"))?;
    db.execute_batch("PRAGMA journal_mode = WAL;
      CREATE TABLE IF NOT EXISTS cards (id TEXT PRIMARY KEY, title TEXT NOT NULL, summary TEXT NOT NULL DEFAULT '', card_type TEXT NOT NULL, tags TEXT NOT NULL DEFAULT '[]', source TEXT NOT NULL DEFAULT '', source_path TEXT, visibility TEXT NOT NULL DEFAULT '本机助手可读', status TEXT NOT NULL DEFAULT '草稿', favorite INTEGER NOT NULL DEFAULT 0, content_path TEXT NOT NULL, created_at TEXT NOT NULL, updated_at TEXT NOT NULL, deleted_at TEXT);
      CREATE INDEX IF NOT EXISTS idx_cards_type ON cards(card_type);
      CREATE INDEX IF NOT EXISTS idx_cards_updated ON cards(updated_at);
      CREATE VIRTUAL TABLE IF NOT EXISTS cards_fts USING fts5(id UNINDEXED, title, summary, content, tags, source);
      CREATE TABLE IF NOT EXISTS card_versions (id INTEGER PRIMARY KEY AUTOINCREMENT, card_id TEXT NOT NULL, title TEXT NOT NULL, summary TEXT NOT NULL, tags TEXT NOT NULL, status TEXT NOT NULL, content TEXT NOT NULL, created_at TEXT NOT NULL);
      CREATE TABLE IF NOT EXISTS audit_log (id INTEGER PRIMARY KEY AUTOINCREMENT, actor TEXT NOT NULL, action TEXT NOT NULL, target_id TEXT, detail TEXT, created_at TEXT NOT NULL);
      CREATE TABLE IF NOT EXISTS card_relations (from_card_id TEXT NOT NULL, to_card_id TEXT NOT NULL, relation_type TEXT NOT NULL, created_at TEXT NOT NULL, PRIMARY KEY (from_card_id,to_card_id,relation_type));").map_err(|e| format!("初始化 SQLite 表失败: {e}"))?;
    let has_deleted_at: bool = db.query_row("SELECT COUNT(*) FROM pragma_table_info('cards') WHERE name='deleted_at'", [], |row| row.get::<_, i64>(0)).map_err(|e| e.to_string())? > 0;
    if !has_deleted_at { db.execute("ALTER TABLE cards ADD COLUMN deleted_at TEXT", []).map_err(|e| format!("升级 SQLite 表失败: {e}"))?; }
    Ok(db)
}

fn color_for_type(card_type: &str) -> &'static str { match card_type { "技能" => "#b69cff", "知识" => "#66d9ef", "用户画像" => "#f7a8d8", "偏好" => "#ffb86b", "AI 产出" => "#91e6a1", _ => "#ffd479" } }
fn read_content(path: &str) -> String { fs::read_to_string(path).unwrap_or_default() }
fn card_json(row: &rusqlite::Row<'_>) -> rusqlite::Result<serde_json::Value> {
    let content_path: String = row.get(9)?;
    let deleted_at: Option<String> = row.get(13)?;
    Ok(serde_json::json!({"id": row.get::<_, String>(0)?, "title": row.get::<_, String>(1)?, "summary": row.get::<_, String>(2)?, "type": row.get::<_, String>(3)?, "tags": serde_json::from_str::<serde_json::Value>(&row.get::<_, String>(4)?).unwrap_or(serde_json::json!([])), "source": row.get::<_, String>(5)?, "sourcePath": row.get::<_, Option<String>>(6)?, "visibility": row.get::<_, String>(7)?, "status": row.get::<_, String>(8)?, "favorite": row.get::<_, i64>(10)? == 1, "content": read_content(&content_path), "contentPath": content_path, "accent": color_for_type(&row.get::<_, String>(3)?), "createdAt": row.get::<_, String>(11)?, "updatedAt": row.get::<_, String>(12)?, "deletedAt": deleted_at}))
}

fn insert_card(db: &Connection, card: &NewCard, source_path: Option<&str>, id: String) -> Result<String, String> {
    validate_card(card)?;
    let stamp = now_string();
    let content_path = data_root().join("cards").join(format!("{id}.md"));
    let tags = serde_json::to_string(&card.tags).map_err(|e| e.to_string())?;
    let markdown = format!("---\ntitle: {}\ntype: {}\ntags: [{}]\nsource: {}\nstatus: {}\ncreated_at: {}\n---\n\n{}\n", card.title, card.card_type, card.tags.join(", "), card.source, card.status, stamp, card.content);
    fs::write(&content_path, markdown).map_err(|e| format!("写入 Markdown 失败: {e}"))?;
    let result = db.execute("INSERT INTO cards (id,title,summary,card_type,tags,source,source_path,visibility,status,content_path,created_at,updated_at,deleted_at) VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9,?10,?11,?11,NULL)", params![id, card.title.trim(), card.summary, card.card_type, tags, card.source, source_path, card.visibility, card.status, content_path.to_string_lossy(), stamp]);
    if let Err(e) = result { let _ = fs::remove_file(&content_path); return Err(format!("写入 SQLite 失败: {e}")); }
    db.execute("INSERT INTO cards_fts (id,title,summary,content,tags,source) VALUES (?1,?2,?3,?4,?5,?6)", params![id, card.title, card.summary, card.content, tags, card.source]).map_err(|e| format!("写入搜索索引失败: {e}"))?;
    audit(db, &card.actor, "create_card", Some(&id), "create card")?;
    Ok(id)
}

fn validate_card(card: &NewCard) -> Result<(), String> {
    if card.title.trim().is_empty() { return Err("标题不能为空".into()); }
    if card.content.len() > 2_000_000 { return Err("正文超过 2MB 限制".into()); }
    if card.card_type.trim().is_empty() { return Err("卡片类型不能为空".into()); }
    if card.actor.trim().is_empty() { return Err("缺少 actor，拒绝写入操作".into()); }
    Ok(())
}

fn seed_defaults(db: &Connection) -> Result<(), String> {
    let count: i64 = db.query_row("SELECT COUNT(*) FROM cards", [], |row| row.get(0)).map_err(|e| e.to_string())?;
    if count > 0 { return Ok(()); }
    let defaults = [
      NewCard { title: "工作台使用说明".into(), summary: "本地优先保存技能、知识、偏好和 AI 产出。".into(), card_type: "知识".into(), tags: vec!["工作台".into(), "本地优先".into()], source: "工作台初始化".into(), visibility: default_visibility(), status: "已验证".into(), content: "# 工作台使用说明\n\n正文保存在 Markdown，索引保存在 SQLite。".into(), actor: "system".into() },
      NewCard { title: "个人 AI 工作台愿景".into(), summary: "统一管理 AI 生成的技能、知识、用户画像、偏好和项目资料。".into(), card_type: "AI 产出".into(), tags: vec!["产品构想".into(), "桌面应用".into()], source: "需求沟通".into(), visibility: default_visibility(), status: "草稿".into(), content: "# 愿景\n\n把重要的 AI 产出沉淀为可检索、可追溯的个人资产。".into(), actor: "system".into() },
    ];
    for (index, card) in defaults.iter().enumerate() { insert_card(db, card, None, format!("seed-{index}"))?; }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn card(title: &str, content: &str) -> NewCard {
        NewCard { title: title.into(), summary: String::new(), card_type: "知识".into(), tags: vec![], source: "test".into(), visibility: default_visibility(), status: default_status(), content: content.into(), actor: "test".into() }
    }

    #[test]
    fn rejects_empty_title() {
        assert_eq!(validate_card(&card("  ", "正文")).unwrap_err(), "标题不能为空");
    }

    #[test]
    fn rejects_content_over_two_megabytes() {
        assert_eq!(validate_card(&card("标题", &"x".repeat(2_000_001))).unwrap_err(), "正文超过 2MB 限制");
    }

    #[test]
    fn accepts_valid_card() {
        assert!(validate_card(&card("标题", "正文")).is_ok());
    }
}

fn mysql_insert_card(card: &NewCard, source_path: Option<&str>, id: String) -> Result<String, String> {
    validate_card(card)?;
    let content = card.content.clone(); let tags = serde_json::to_string(&card.tags).map_err(|e| e.to_string())?; let stamp = chrono_like_now() as i64;
    let mut conn = mysql_connection()?;
    conn.query_drop("START TRANSACTION").map_err(|e|e.to_string())?;
    let result = (|| {
        conn.exec_drop("INSERT INTO cards (id,title,summary,card_type,tags,source,source_path,visibility,status,favorite,created_at,updated_at) VALUES (:id,:title,:summary,:card_type,:tags,:source,:source_path,:visibility,:status,FALSE,FROM_UNIXTIME(:stamp),FROM_UNIXTIME(:stamp))", mysql::Params::Named(std::collections::HashMap::from([(b"id".to_vec(),mysql::Value::from(id.clone())),(b"title".to_vec(),mysql::Value::from(card.title.trim())),(b"summary".to_vec(),mysql::Value::from(card.summary.clone())),(b"card_type".to_vec(),mysql::Value::from(card.card_type.clone())),(b"tags".to_vec(),mysql::Value::from(tags.clone())),(b"source".to_vec(),mysql::Value::from(card.source.clone())),(b"source_path".to_vec(),mysql::Value::from(source_path.map(str::to_string))), (b"visibility".to_vec(),mysql::Value::from(card.visibility.clone())),(b"status".to_vec(),mysql::Value::from(card.status.clone())),(b"stamp".to_vec(),mysql::Value::from(stamp))]))).map_err(|e|e.to_string())?;
        conn.exec_drop("INSERT INTO card_contents (card_id,content,content_sha256,updated_at) VALUES (:id,:content,:hash,FROM_UNIXTIME(:stamp))", mysql::Params::Named(std::collections::HashMap::from([(b"id".to_vec(),mysql::Value::from(id.clone())),(b"content".to_vec(),mysql::Value::from(content.clone())),(b"hash".to_vec(),mysql::Value::from(format!("{:x}",sha2::Sha256::digest(content.as_bytes())))),(b"stamp".to_vec(),mysql::Value::from(stamp))]))).map_err(|e|e.to_string())?;
        conn.exec_drop("INSERT INTO audit_log (actor,action,target_id,detail,created_at) VALUES (:actor,'create_card',:id,'create card',FROM_UNIXTIME(:stamp))", mysql::Params::Named(std::collections::HashMap::from([(b"actor".to_vec(),mysql::Value::from(card.actor.clone())),(b"id".to_vec(),mysql::Value::from(id.clone())),(b"stamp".to_vec(),mysql::Value::from(stamp))]))).map_err(|e|e.to_string())?;
        Ok::<_,String>(())
    })();
    match result { Ok(()) => { conn.query_drop("COMMIT").map_err(|e|e.to_string())?; Ok(id) }, Err(e) => { let _=conn.query_drop("ROLLBACK"); Err(e) } }
}

fn mysql_update_card(id: &str, patch: CardPatch) -> Result<serde_json::Value, String> {
    let mut conn = mysql_connection()?;
    let current: mysql::Row = conn.exec_first("SELECT title,summary,tags,status,favorite,COALESCE(cc.content,'') FROM cards c LEFT JOIN card_contents cc ON cc.card_id=c.id WHERE c.id=:id", mysql::Params::Named(std::collections::HashMap::from([(b"id".to_vec(), mysql::Value::from(id))]))).map_err(|e|e.to_string())?.ok_or_else(|| "卡片不存在".to_string())?;
    let old_content = current.get::<String,usize>(5).unwrap_or_default();
    let content = patch.content.clone().unwrap_or(old_content.clone()); if content.len() > 2_000_000 { return Err("正文超过 2MB 限制".into()); }
    let actor = patch.actor.as_deref().unwrap_or("").trim(); if actor.is_empty() { return Err("缺少 actor，拒绝写入操作".into()); }
    let stamp=chrono_like_now() as i64; let title=patch.title.clone().unwrap_or_else(||current.get::<String,usize>(0).unwrap_or_default()); let summary=patch.summary.clone().unwrap_or_else(||current.get::<String,usize>(1).unwrap_or_default()); let tags=serde_json::to_string(&patch.tags.clone().unwrap_or_else(||serde_json::from_str(&current.get::<String,usize>(2).unwrap_or_else(||"[]".into())).unwrap_or_default())).map_err(|e|e.to_string())?; let status=patch.status.clone().unwrap_or_else(||current.get::<String,usize>(3).unwrap_or_default()); let favorite=patch.favorite.unwrap_or_else(||current.get::<bool,usize>(4).unwrap_or(false));
    conn.query_drop("START TRANSACTION").map_err(|e|e.to_string())?;
    let result=(||{ let mut p=std::collections::HashMap::new(); p.insert(b"card_id".to_vec(),mysql::Value::from(id)); p.insert(b"title".to_vec(),mysql::Value::from(current.get::<String,usize>(0).unwrap_or_default())); p.insert(b"summary".to_vec(),mysql::Value::from(current.get::<String,usize>(1).unwrap_or_default())); p.insert(b"tags".to_vec(),mysql::Value::from(current.get::<String,usize>(2).unwrap_or_else(||"[]".into()))); p.insert(b"status".to_vec(),mysql::Value::from(current.get::<String,usize>(3).unwrap_or_default())); p.insert(b"content".to_vec(),mysql::Value::from(old_content.clone())); p.insert(b"hash".to_vec(),mysql::Value::from(format!("{:x}",sha2::Sha256::digest(old_content.as_bytes())))); p.insert(b"stamp".to_vec(),mysql::Value::from(stamp)); conn.exec_drop("INSERT INTO card_versions (card_id,title,summary,tags,status,content,content_sha256,created_at) VALUES (:card_id,:title,:summary,:tags,:status,:content,:hash,FROM_UNIXTIME(:stamp))",mysql::Params::Named(p)).map_err(|e|e.to_string())?; let mut p=std::collections::HashMap::new(); p.insert(b"id".to_vec(),mysql::Value::from(id)); p.insert(b"title".to_vec(),mysql::Value::from(title)); p.insert(b"summary".to_vec(),mysql::Value::from(summary)); p.insert(b"tags".to_vec(),mysql::Value::from(tags)); p.insert(b"status".to_vec(),mysql::Value::from(status)); p.insert(b"favorite".to_vec(),mysql::Value::from(favorite)); p.insert(b"stamp".to_vec(),mysql::Value::from(stamp)); conn.exec_drop("UPDATE cards SET title=:title,summary=:summary,tags=:tags,status=:status,favorite=:favorite,updated_at=FROM_UNIXTIME(:stamp) WHERE id=:id",mysql::Params::Named(p)).map_err(|e|e.to_string())?; let mut p=std::collections::HashMap::new(); p.insert(b"id".to_vec(),mysql::Value::from(id)); p.insert(b"content".to_vec(),mysql::Value::from(content.clone())); p.insert(b"hash".to_vec(),mysql::Value::from(format!("{:x}",sha2::Sha256::digest(content.as_bytes())))); p.insert(b"stamp".to_vec(),mysql::Value::from(stamp)); conn.exec_drop("UPDATE card_contents SET content=:content,content_sha256=:hash,updated_at=FROM_UNIXTIME(:stamp) WHERE card_id=:id",mysql::Params::Named(p)).map_err(|e|e.to_string())?; let mut p=std::collections::HashMap::new(); p.insert(b"actor".to_vec(),mysql::Value::from(actor)); p.insert(b"id".to_vec(),mysql::Value::from(id)); p.insert(b"stamp".to_vec(),mysql::Value::from(stamp)); conn.exec_drop("INSERT INTO audit_log(actor,action,target_id,detail,created_at) VALUES(:actor,'update_card',:id,'update card',FROM_UNIXTIME(:stamp))",mysql::Params::Named(p)).map_err(|e|e.to_string())?; Ok::<_,String>(()) })(); match result { Ok(())=>{conn.query_drop("COMMIT").map_err(|e|e.to_string())?; mysql_get_card(id)}, Err(e)=>{let _=conn.query_drop("ROLLBACK");Err(e)} }
}

fn mysql_add_relation(from_id:&str, request:RelationRequest)->Result<(),String>{ if request.actor.trim().is_empty(){return Err("缺少 actor，拒绝写入操作".into())} let allowed=["关联项目","相关知识","来源于","依赖"]; if !allowed.contains(&request.relation_type.as_str()){return Err("不支持的关系类型".into())} let mut conn=mysql_connection()?; let changed=conn.exec_drop("INSERT INTO card_relations(from_card_id,to_card_id,relation_type,created_at) VALUES(:from_id,:to_id,:relation_type,NOW(6))",mysql::Params::Named(std::collections::HashMap::from([(b"from_id".to_vec(),mysql::Value::from(from_id)),(b"to_id".to_vec(),mysql::Value::from(request.to_card_id)),(b"relation_type".to_vec(),mysql::Value::from(request.relation_type))]))); changed.map_err(|e|e.to_string())?; Ok(()) }

fn mysql_restore_version(id:&str, version_id:i64)->Result<serde_json::Value,String>{ let mut conn=mysql_connection()?; let row: mysql::Row = conn.exec_first("SELECT title,summary,tags,status,content FROM card_versions WHERE id=:version_id AND card_id=:id",mysql::Params::Named(std::collections::HashMap::from([(b"version_id".to_vec(),mysql::Value::from(version_id)),(b"id".to_vec(),mysql::Value::from(id))]))).map_err(|e|e.to_string())?.ok_or_else(||"版本不存在".to_string())?; let title=row.get::<String,usize>(0).unwrap_or_default(); let summary=row.get::<String,usize>(1).unwrap_or_default(); let tags=row.get::<String,usize>(2).unwrap_or_else(||"[]".into()); let status=row.get::<String,usize>(3).unwrap_or_default(); let content=row.get::<String,usize>(4).unwrap_or_default(); let patch=CardPatch{title:Some(title),summary:Some(summary),tags:serde_json::from_str(&tags).ok(),status:Some(status),favorite:None,content:Some(content),actor:Some("desktop-user".into())}; mysql_update_card(id,patch) }

fn mysql_card_json(row: mysql::Row) -> Result<serde_json::Value, String> {
    let get = |index: usize| row.get::<String, usize>(index).ok_or_else(|| format!("MySQL 卡片列 {index} 缺失"));
    let id = get(0)?; let title = get(1)?; let summary = get(2)?; let card_type = get(3)?; let tags = get(4)?; let source = get(5)?; let source_path = row.get::<Option<String>, usize>(6).unwrap_or(None); let visibility = get(7)?; let status = get(8)?; let favorite = row.get::<bool, usize>(9).unwrap_or(false); let created_at = row.get::<i64, usize>(10).unwrap_or(0); let updated_at = row.get::<i64, usize>(11).unwrap_or(0); let deleted_at = row.get::<Option<i64>, usize>(12).unwrap_or(None); let content = get(13)?;
    Ok(serde_json::json!({"id":id,"title":title,"summary":summary,"type":card_type,"tags":serde_json::from_str::<serde_json::Value>(&tags).unwrap_or(serde_json::json!([])),"source":source,"sourcePath":source_path,"visibility":visibility,"status":status,"favorite":favorite,"content":content,"accent":color_for_type(&card_type),"createdAt":created_at.to_string(),"updatedAt":updated_at.to_string(),"deletedAt":deleted_at,"contentPath":source_path}))
}


fn mysql_list_cards(params: &std::collections::HashMap<String, String>) -> Result<serde_json::Value, String> {
    let mut conn = mysql_connection()?;
    let query = params.get("query").cloned().unwrap_or_default();
    let mut sql = String::from("SELECT c.id,c.title,c.summary,c.card_type,c.tags,c.source,c.source_path,c.visibility,c.status,c.favorite,CAST(UNIX_TIMESTAMP(c.created_at) AS SIGNED),CAST(UNIX_TIMESTAMP(c.updated_at) AS SIGNED),CAST(UNIX_TIMESTAMP(c.deleted_at) AS SIGNED),COALESCE(cc.content,'') FROM cards c LEFT JOIN card_contents cc ON cc.card_id=c.id WHERE 1=1");
    let mut named: std::collections::HashMap<Vec<u8>, mysql::Value> = std::collections::HashMap::new();
    if !query.trim().is_empty() { sql.push_str(" AND (c.title LIKE :q OR c.summary LIKE :q OR c.tags LIKE :q OR cc.content LIKE :q)"); named.insert(b"q".to_vec(), mysql::Value::from(format!("%{}%", query))); }
    if let Some(v) = params.get("type") { sql.push_str(" AND c.card_type=:type"); named.insert(b"type".to_vec(), mysql::Value::from(v.clone())); }
    if let Some(v) = params.get("status") { sql.push_str(" AND c.status=:status"); named.insert(b"status".to_vec(), mysql::Value::from(v.clone())); }
    if let Some(v) = params.get("tag") { sql.push_str(" AND JSON_CONTAINS(c.tags, JSON_QUOTE(:tag))"); named.insert(b"tag".to_vec(), mysql::Value::from(v.clone())); }
    if params.get("include_deleted").map(String::as_str) != Some("1") { sql.push_str(" AND c.deleted_at IS NULL"); }
    sql.push_str(" ORDER BY c.updated_at DESC");
    let rows: Vec<mysql::Row> = if named.is_empty() { conn.query(sql).map_err(|e| e.to_string())? } else { conn.exec(sql, mysql::Params::Named(named)).map_err(|e| e.to_string())? };
    rows.into_iter().map(mysql_card_json).collect::<Result<Vec<_>,_>>().map(serde_json::Value::Array)
}

fn mysql_get_card(id: &str) -> Result<serde_json::Value, String> {
    let mut conn = mysql_connection()?;
    let row = conn.exec_first("SELECT c.id,c.title,c.summary,c.card_type,c.tags,c.source,c.source_path,c.visibility,c.status,c.favorite,CAST(UNIX_TIMESTAMP(c.created_at) AS SIGNED),CAST(UNIX_TIMESTAMP(c.updated_at) AS SIGNED),CAST(UNIX_TIMESTAMP(c.deleted_at) AS SIGNED),COALESCE(cc.content,'') FROM cards c LEFT JOIN card_contents cc ON cc.card_id=c.id WHERE c.id=:id", mysql::Params::Named(std::collections::HashMap::from([(b"id".to_vec(), mysql::Value::from(id))]))).map_err(|e| e.to_string())?.ok_or_else(|| "卡片不存在".to_string())?;
    mysql_card_json(row)
}

fn mysql_list_versions(id: &str) -> Result<serde_json::Value, String> {
    let mut conn = mysql_connection()?;
    let rows: Vec<mysql::Row> = conn.exec("SELECT id,card_id,title,summary,tags,status,CAST(UNIX_TIMESTAMP(created_at) AS SIGNED) FROM card_versions WHERE card_id=:id ORDER BY id DESC", mysql::Params::Named(std::collections::HashMap::from([(b"id".to_vec(), mysql::Value::from(id))]))).map_err(|e|e.to_string())?;
    let mut result = Vec::new();
    for row in rows { let (id,card_id,title,summary,tags,status,created_at):(i64,String,String,String,String,String,i64)=mysql::from_row_opt(row).map_err(|e|e.to_string())?; result.push(serde_json::json!({"id":id,"cardId":card_id,"title":title,"summary":summary,"tags":serde_json::from_str::<serde_json::Value>(&tags).unwrap_or(serde_json::json!([])),"status":status,"createdAt":created_at.to_string()})); }
    Ok(serde_json::Value::Array(result))
}

fn mysql_list_relations(id: &str) -> Result<serde_json::Value, String> {
    let mut conn = mysql_connection()?;
    let rows: Vec<mysql::Row> = conn.exec("SELECT r.to_card_id,r.relation_type,c.title,c.card_type FROM card_relations r LEFT JOIN cards c ON c.id=r.to_card_id WHERE r.from_card_id=:id ORDER BY r.created_at DESC", mysql::Params::Named(std::collections::HashMap::from([(b"id".to_vec(), mysql::Value::from(id))]))).map_err(|e|e.to_string())?;
    let mut result=Vec::new(); for row in rows { let (card_id,relation_type,title,card_type):(String,String,Option<String>,Option<String>)=mysql::from_row_opt(row).map_err(|e|e.to_string())?; result.push(serde_json::json!({"cardId":card_id,"relationType":relation_type,"title":title,"type":card_type})); } Ok(serde_json::Value::Array(result))
}

fn mysql_search_context(params: &std::collections::HashMap<String, String>) -> Result<serde_json::Value, String> {
    let query = params.get("q").or_else(|| params.get("query")).cloned().unwrap_or_default();
    if query.trim().is_empty() { return Err("缺少搜索关键词 q".into()); }
    let mut search_params = params.clone(); search_params.insert("query".into(), query.clone());
    let results = mysql_list_cards(&search_params)?;
    let count = results.as_array().map(|v| v.len()).unwrap_or(0);
    Ok(serde_json::json!({"query":query,"count":count,"results":results}))
}

fn mysql_list_audit(params: &std::collections::HashMap<String, String>) -> Result<serde_json::Value, String> {
    let limit = params.get("limit").and_then(|v| v.parse::<usize>().ok()).unwrap_or(50).min(200);
    let mut conn = mysql_connection()?;
    let rows: Vec<mysql::Row> = conn.exec("SELECT id,actor,action,target_id,detail,CAST(UNIX_TIMESTAMP(created_at) AS SIGNED) FROM audit_log ORDER BY id DESC LIMIT :limit", mysql::Params::Named(std::collections::HashMap::from([(b"limit".to_vec(), mysql::Value::from(limit as u64))]))).map_err(|e|e.to_string())?;
    let mut result=Vec::new(); for row in rows { let id=row.get::<i64,usize>(0).unwrap_or(0); result.push(serde_json::json!({"id":id,"actor":row.get::<String,usize>(1).unwrap_or_default(),"action":row.get::<String,usize>(2).unwrap_or_default(),"targetId":row.get::<Option<String>,usize>(3).unwrap_or(None),"detail":row.get::<Option<String>,usize>(4).unwrap_or(None),"createdAt":row.get::<i64,usize>(5).unwrap_or(0).to_string()})); } Ok(serde_json::Value::Array(result))
}

fn parse_query(url: &str) -> (String, std::collections::HashMap<String, String>) {
    let mut parts = url.splitn(2, '?'); let path = parts.next().unwrap_or_default().to_string(); let mut map = std::collections::HashMap::new();
    if let Some(query) = parts.next() { for pair in query.split('&') { let mut kv = pair.splitn(2, '='); if let (Some(k), Some(v)) = (kv.next(), kv.next()) { map.insert(percent_decode(k), percent_decode(v)); } } }
    (path, map)
}

fn percent_decode(value: &str) -> String {
    let bytes = value.as_bytes(); let mut out = Vec::with_capacity(bytes.len()); let mut index = 0;
    while index < bytes.len() {
        if bytes[index] == b'%' && index + 2 < bytes.len() {
            let hex = |b: u8| -> Option<u8> { match b { b'0'..=b'9' => Some(b - b'0'), b'a'..=b'f' => Some(b - b'a' + 10), b'A'..=b'F' => Some(b - b'A' + 10), _ => None } };
            if let (Some(high), Some(low)) = (hex(bytes[index + 1]), hex(bytes[index + 2])) { out.push(high * 16 + low); index += 3; continue; }
        }
        out.push(if bytes[index] == b'+' { b' ' } else { bytes[index] }); index += 1;
    }
    String::from_utf8_lossy(&out).into_owned()
}

fn list_cards(params: &std::collections::HashMap<String, String>) -> Result<serde_json::Value, String> {
    let db = db_connection()?;
    let query = params.get("query").map(|v| v.to_lowercase());
    let by_type = params.get("type").map(|v| v.to_string());
    let by_status = params.get("status").map(|v| v.to_string());
    let by_tag = params.get("tag").map(|v| v.to_string());
    if query.is_none() && by_type.is_none() && by_status.is_none() && by_tag.is_none() {
        let mut stmt = db.prepare("SELECT id,title,summary,card_type,tags,source,source_path,visibility,status,content_path,favorite,created_at,updated_at,deleted_at FROM cards WHERE deleted_at IS NULL ORDER BY updated_at DESC").map_err(|e| e.to_string())?;
        let rows = stmt.query_map([], card_json).map_err(|e| e.to_string())?;
        let mut cards = Vec::new(); for row in rows { cards.push(row.map_err(|e| e.to_string())?); }
        return Ok(serde_json::json!(cards));
    }
    let mut clauses = Vec::new(); let mut values: Vec<String> = Vec::new();
    if let Some(q) = query { clauses.push("cards_fts MATCH ?".to_string()); values.push(format!("{q}*")); }
    if let Some(v) = by_type { clauses.push("cards.card_type = ?".to_string()); values.push(v); }
    if let Some(v) = by_status { clauses.push("cards.status = ?".to_string()); values.push(v); }
    if let Some(v) = by_tag { clauses.push("EXISTS (SELECT 1 FROM json_each(cards.tags) WHERE json_each.value = ?)".to_string()); values.push(v); }
    let sql = format!("SELECT cards.id,cards.title,cards.summary,cards.card_type,cards.tags,cards.source,cards.source_path,cards.visibility,cards.status,cards.content_path,cards.favorite,cards.created_at,cards.updated_at,deleted_at FROM cards JOIN cards_fts ON cards_fts.id = cards.id WHERE {} ORDER BY cards.updated_at DESC", clauses.join(" AND "));
    let mut stmt = db.prepare(&sql).map_err(|e| e.to_string())?;
    let refs: Vec<&dyn rusqlite::ToSql> = values.iter().map(|v| v as &dyn rusqlite::ToSql).collect();
    let rows = stmt.query_map(refs.as_slice(), card_json).map_err(|e| e.to_string())?;
    let mut cards = Vec::new(); for row in rows { cards.push(row.map_err(|e| e.to_string())?); }
    Ok(serde_json::json!(cards))
}

fn search_context(params: &std::collections::HashMap<String, String>, context_only: bool) -> Result<serde_json::Value, String> {
    let query = params.get("q").or_else(|| params.get("query")).cloned().unwrap_or_default();
    if query.trim().is_empty() { return Err("缺少搜索关键词 q".into()); }
    let limit = params.get("limit").and_then(|v| v.parse::<usize>().ok()).unwrap_or(5).min(20);
    let db = db_connection()?;
    let mut stmt = db.prepare("SELECT cards.id,cards.title,cards.summary,cards.card_type,cards.tags,cards.source,cards.source_path,cards.visibility,cards.status,cards.content_path,cards.favorite,cards.created_at,cards.updated_at,cards.deleted_at FROM cards JOIN cards_fts ON cards_fts.id=cards.id WHERE cards.deleted_at IS NULL AND cards_fts MATCH ?1 ORDER BY bm25(cards_fts) LIMIT ?2").map_err(|e| e.to_string())?;
    let rows = stmt.query_map(params![format!("{}*", query), limit as i64], card_json).map_err(|e| e.to_string())?;
    let mut cards = Vec::new();
    for row in rows { let card = row.map_err(|e| e.to_string())?; if context_only { cards.push(serde_json::json!({"id":card["id"],"title":card["title"],"type":card["type"],"summary":card["summary"],"content":card["content"],"source":card["source"]})); } else { cards.push(card); } }
    Ok(serde_json::json!({"query":query,"count":cards.len(),"results":cards}))
}

#[derive(Debug, Deserialize)]
struct RelationRequest { to_card_id: String, relation_type: String, actor: String }

fn list_relations(id: &str) -> Result<serde_json::Value, String> {
    let db = db_connection()?; let mut stmt = db.prepare("SELECT r.to_card_id,r.relation_type,c.title,c.card_type FROM card_relations r LEFT JOIN cards c ON c.id=r.to_card_id WHERE r.from_card_id=?1 ORDER BY r.created_at DESC").map_err(|e| e.to_string())?;
    let rows = stmt.query_map([id], |row| Ok(serde_json::json!({"cardId":row.get::<_,String>(0)?,"relationType":row.get::<_,String>(1)?,"title":row.get::<_,Option<String>>(2)?,"type":row.get::<_,Option<String>>(3)?}))).map_err(|e| e.to_string())?; let mut result=Vec::new(); for row in rows { result.push(row.map_err(|e| e.to_string())?); } Ok(serde_json::json!(result))
}

fn add_relation(from_id: &str, request: RelationRequest) -> Result<(), String> {
    if request.actor.trim().is_empty() { return Err("缺少 actor，拒绝写入操作".into()); }
    let allowed=["关联项目","相关知识","来源于","依赖"]; if !allowed.contains(&request.relation_type.as_str()) { return Err("不支持的关系类型".into()); }
    let db=db_connection()?; let exists:i64=db.query_row("SELECT COUNT(*) FROM cards WHERE id=?1",[&request.to_card_id],|r|r.get(0)).map_err(|e|e.to_string())?; if exists==0{return Err("目标卡片不存在".into());}
    let changed=db.execute("INSERT OR IGNORE INTO card_relations (from_card_id,to_card_id,relation_type,created_at) VALUES (?1,?2,?3,?4)",params![from_id,request.to_card_id,request.relation_type,now_string()]).map_err(|e|e.to_string())?; if changed==0{return Err("关系已存在".into());} audit(&db,&request.actor,"add_relation",Some(from_id),"add card relation")
}

fn list_audit(params: &std::collections::HashMap<String, String>) -> Result<serde_json::Value, String> {
    let limit=params.get("limit").and_then(|v|v.parse::<usize>().ok()).unwrap_or(50).min(200); let db=db_connection()?; let mut stmt=db.prepare("SELECT id,actor,action,target_id,detail,created_at FROM audit_log ORDER BY id DESC LIMIT ?1").map_err(|e|e.to_string())?; let rows=stmt.query_map([limit as i64],|r|Ok(serde_json::json!({"id":r.get::<_,i64>(0)?,"actor":r.get::<_,String>(1)?,"action":r.get::<_,String>(2)?,"targetId":r.get::<_,Option<String>>(3)?,"detail":r.get::<_,Option<String>>(4)?,"createdAt":r.get::<_,String>(5)?}))).map_err(|e|e.to_string())?; let mut result=Vec::new(); for row in rows{result.push(row.map_err(|e|e.to_string())?);} Ok(serde_json::json!(result))
}

fn read_body(request: &mut Request) -> Result<String, String> {
    let length = request.headers().iter().find(|header| header.field.equiv("Content-Length")).and_then(|header| header.value.as_str().parse::<usize>().ok());
    let mut bytes = Vec::new();
    match length {
        Some(length) => { bytes.resize(length, 0); request.as_reader().read_exact(&mut bytes).map_err(|e| format!("读取请求体失败: {e}"))?; }
        None => { request.as_reader().read_to_end(&mut bytes).map_err(|e| format!("读取分块请求体失败: {e}"))?; }
    }
    String::from_utf8(bytes).map_err(|e| format!("请求体不是有效 UTF-8: {e}"))
}
fn handle_request(mut request: Request) {
    let (path, params) = parse_query(request.url());
    let response = match (request.method(), path.as_str()) {
      (&Method::Options, _) => json_response(serde_json::json!({"ok":true}), 204),
      (&Method::Get, "/api/health") => json_response(serde_json::json!({"ok":true,"service":"personal-ai-workbench","storage":"mysql","host":"127.0.0.1","port":3306,"database":MYSQL_DATABASE}), 200),
      (&Method::Get, "/api/search") => mysql_search_context(&params).map(|v| json_response(v, 200)).unwrap_or_else(|e| error_response(&e, 400)),
      (&Method::Get, "/api/context") => mysql_search_context(&params).map(|v| json_response(v, 200)).unwrap_or_else(|e| error_response(&e, 400)),
      (&Method::Get, "/api/audit") => mysql_list_audit(&params).map(|v|json_response(v,200)).unwrap_or_else(|e|error_response(&e,500)),
      (&Method::Get, "/api/cards") => mysql_list_cards(&params).map(|v| json_response(v, 200)).unwrap_or_else(|e| error_response(&e, 500)),
      (&Method::Get, p) if p.starts_with("/api/cards/") && p.ends_with("/relations") => { let id=&p[11..p.len()-10]; mysql_list_relations(id).map(|v|json_response(v,200)).unwrap_or_else(|e|error_response(&e,404)) },
      (&Method::Post, p) if p.starts_with("/api/cards/") && p.ends_with("/relations") => { let id=&p[11..p.len()-10]; match read_body(&mut request).and_then(|b|serde_json::from_str::<RelationRequest>(&b).map_err(|e|e.to_string())).and_then(|r|mysql_add_relation(id,r)){Ok(())=>json_response(serde_json::json!({"status":"created"}),201),Err(e)=>error_response(&e,400)} },
      (&Method::Get, p) if p.starts_with("/api/cards/") && p.ends_with("/versions") => { let id = &p[11..p.len()-9]; mysql_list_versions(id).map(|v| json_response(v, 200)).unwrap_or_else(|e| error_response(&e, 404)) },
      (&Method::Post, p) if p.starts_with("/api/cards/") && p.contains("/versions/") && p.ends_with("/restore") => { let prefix = &p[11..p.len()-8]; let mut parts = prefix.split("/versions/"); let id = parts.next().unwrap_or_default(); let version_id = parts.next().and_then(|v| v.parse::<i64>().ok()).unwrap_or(0); mysql_restore_version(id, version_id).map(|v| json_response(v, 200)).unwrap_or_else(|e| error_response(&e, 400)) },
      (&Method::Get, p) if p.starts_with("/api/cards/") => { let id = &p[11..]; match mysql_get_card(id) { Ok(card) => json_response(card, 200), Err(_) => error_response("卡片不存在", 404) } },
      (&Method::Post, "/api/backup") => match create_backup() { Ok(path) => { if let Ok(db)=db_connection(){ let _=audit(&db,"desktop-user","backup",None,&path); } json_response(serde_json::json!({"status":"created","path":path}), 201) }, Err(e) => error_response(&e, 500) },
      (&Method::Post, "/api/cards") => match read_body(&mut request).and_then(|body| serde_json::from_str::<NewCard>(&body).map_err(|e| format!("请求格式错误: {e}"))).and_then(|card| mysql_insert_card(&card, None, format!("card-{}", chrono_like_now()))) { Ok(id) => json_response(serde_json::json!({"id": id, "status":"created"}), 201), Err(e) => error_response(&e, 400) },
      (&Method::Post, "/api/import") => match read_body(&mut request).and_then(|body| serde_json::from_str::<ImportRequest>(&body).map_err(|e| format!("请求格式错误: {e}"))).and_then(|req| import_file(&req)) { Ok(id) => json_response(serde_json::json!({"id": id, "status":"imported"}), 201), Err(e) => error_response(&e, 400) },
      (&Method::Patch, p) if p.starts_with("/api/cards/") => { let id = &p[11..].to_string(); match read_body(&mut request).and_then(|body| serde_json::from_str::<CardPatch>(&body).map_err(|e| format!("请求格式错误: {e}"))).and_then(|patch| mysql_update_card(id, patch)) { Ok(card) => json_response(card, 200), Err(e) => error_response(&e, 400) } },
      (&Method::Delete, p) if p.starts_with("/api/cards/") => error_response("删除已有卡片不被支持", 405),
      _ => error_response("not found", 404),
    }; let _ = request.respond(response);
}

fn save_version(db: &Connection, id: &str, content_path: &str) -> Result<(), String> {
    let (title, summary, tags, status): (String, String, String, String) = db.query_row("SELECT title,summary,tags,status FROM cards WHERE id=?1", [id], |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?, row.get(3)?))).map_err(|_| "卡片不存在".to_string())?;
    db.execute("INSERT INTO card_versions (card_id,title,summary,tags,status,content,created_at) VALUES (?1,?2,?3,?4,?5,?6,?7)", params![id, title, summary, tags, status, read_content(content_path), now_string()]).map_err(|e| format!("保存版本失败: {e}"))?;
    Ok(())
}

fn audit(db: &Connection, actor: &str, action: &str, target_id: Option<&str>, detail: &str) -> Result<(), String> {
    let actor = actor.trim(); if actor.is_empty() { return Err("缺少 actor，拒绝写入操作".into()); }
    db.execute("INSERT INTO audit_log (actor,action,target_id,detail,created_at) VALUES (?1,?2,?3,?4,?5)", params![actor, action, target_id, detail, now_string()]).map_err(|e| format!("写入审计日志失败: {e}"))?; Ok(())
}

fn update_card(id: &str, patch: CardPatch) -> Result<serde_json::Value, String> {
    let db = db_connection()?; let mut stmt = db.prepare("SELECT content_path FROM cards WHERE id=?1").map_err(|e| e.to_string())?; let content_path: String = stmt.query_row([id], |row| row.get(0)).map_err(|_| "卡片不存在".to_string())?;
    let current = read_content(&content_path); let content = patch.content.clone().unwrap_or(current); if content.len() > 2_000_000 { return Err("正文超过 2MB 限制".into()); }
    save_version(&db, id, &content_path)?;
    audit(&db, patch.actor.as_deref().unwrap_or(""), "update_card", Some(id), "PATCH card")?;
    if let Some(value) = patch.content.as_ref() { fs::write(&content_path, value).map_err(|e| e.to_string())?; }
    let stamp = now_string(); let mut sets = vec!["updated_at=?1".to_string()]; let mut values: Vec<String> = vec![stamp.clone()];
    macro_rules! add { ($field:expr, $value:expr) => { if let Some(value) = $value { values.push(value); sets.push(format!("{}=?{}", $field, values.len())); } }; }
    add!("title", patch.title); add!("summary", patch.summary); add!("status", patch.status); if let Some(tags) = patch.tags { values.push(serde_json::to_string(&tags).map_err(|e| e.to_string())?); sets.push(format!("tags=?{}", values.len())); } if let Some(favorite) = patch.favorite { values.push(if favorite { "1".into() } else { "0".into() }); sets.push(format!("favorite=?{}", values.len())); }
    let sql = format!("UPDATE cards SET {} WHERE id=?{}", sets.join(","), values.len() + 1); values.push(id.into()); let refs: Vec<&dyn rusqlite::ToSql> = values.iter().map(|v| v as &dyn rusqlite::ToSql).collect(); db.execute(&sql, refs.as_slice()).map_err(|e| e.to_string())?;
    db.execute("DELETE FROM cards_fts WHERE id=?1", [id]).map_err(|e| e.to_string())?; let _ = db.execute("INSERT INTO cards_fts (id,title,summary,content,tags,source) SELECT id,title,summary,?1,tags,source FROM cards WHERE id=?2", params![content, id]);
    let mut s = db.prepare("SELECT id,title,summary,card_type,tags,source,source_path,visibility,status,content_path,favorite,created_at,updated_at,deleted_at FROM cards WHERE id=?1").map_err(|e| e.to_string())?; s.query_row([id], card_json).map_err(|e| e.to_string())
}

fn list_versions(id: &str) -> Result<serde_json::Value, String> {
    let db = db_connection()?;
    let mut stmt = db.prepare("SELECT id,card_id,title,summary,tags,status,created_at FROM card_versions WHERE card_id=?1 ORDER BY id DESC").map_err(|e| e.to_string())?;
    let rows = stmt.query_map([id], |row| Ok(serde_json::json!({"id": row.get::<_, i64>(0)?, "cardId": row.get::<_, String>(1)?, "title": row.get::<_, String>(2)?, "summary": row.get::<_, String>(3)?, "tags": serde_json::from_str::<serde_json::Value>(&row.get::<_, String>(4)?).unwrap_or(serde_json::json!([])), "status": row.get::<_, String>(5)?, "createdAt": row.get::<_, String>(6)?}))).map_err(|e| e.to_string())?;
    let mut versions = Vec::new(); for row in rows { versions.push(row.map_err(|e| e.to_string())?); } Ok(serde_json::json!(versions))
}

fn restore_version(id: &str, version_id: i64) -> Result<serde_json::Value, String> {
    let db = db_connection()?;
    let mut card_stmt = db.prepare("SELECT content_path FROM cards WHERE id=?1").map_err(|e| e.to_string())?;
    let content_path: String = card_stmt.query_row([id], |row| row.get(0)).map_err(|_| "卡片不存在".to_string())?;
    save_version(&db, id, &content_path)?;
    let (title, summary, tags, status, content): (String, String, String, String, String) = db.query_row("SELECT title,summary,tags,status,content FROM card_versions WHERE id=?1 AND card_id=?2", params![version_id, id], |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?, row.get(3)?, row.get(4)?))).map_err(|_| "版本不存在".to_string())?;
    fs::write(&content_path, &content).map_err(|e| e.to_string())?;
    db.execute("UPDATE cards SET title=?1,summary=?2,tags=?3,status=?4,updated_at=?5 WHERE id=?6", params![title, summary, tags, status, now_string(), id]).map_err(|e| e.to_string())?;
    db.execute("DELETE FROM cards_fts WHERE id=?1", [id]).map_err(|e| e.to_string())?;
    db.execute("INSERT INTO cards_fts (id,title,summary,content,tags,source) SELECT id,title,summary,?1,tags,source FROM cards WHERE id=?2", params![content, id]).map_err(|e| e.to_string())?;
    let mut stmt = db.prepare("SELECT id,title,summary,card_type,tags,source,source_path,visibility,status,content_path,favorite,created_at,updated_at,deleted_at FROM cards WHERE id=?1").map_err(|e| e.to_string())?;
    stmt.query_row([id], card_json).map_err(|e| e.to_string())
}

fn extract_import_text(source: &PathBuf, extension: &str) -> Result<String, String> {
    match extension {
        "md" | "markdown" | "txt" => fs::read_to_string(source).map_err(|e| format!("读取文件失败: {e}")),
        "pdf" => { let output = Command::new("pdftotext").arg("-enc").arg("UTF-8").arg(source).arg("-").output().map_err(|e| format!("调用 pdftotext 失败: {e}"))?; if !output.status.success() { return Err("PDF 文本提取失败".into()); } String::from_utf8(output.stdout).map_err(|e| e.to_string()) }
        "docx" | "xlsx" => { let file = fs::File::open(source).map_err(|e| e.to_string())?; let mut archive = zip::ZipArchive::new(file).map_err(|e| format!("Office 文件解析失败: {e}"))?; let target = if extension == "docx" { "word/document.xml" } else { "xl/sharedStrings.xml" }; let mut xml = String::new(); archive.by_name(target).map_err(|_| "Office 文本内容不存在".to_string())?.read_to_string(&mut xml).map_err(|e| e.to_string())?; Ok(xml.replace("</w:p>", "\n").replace("</t>", " ").replace('<', " <").split('>').filter_map(|part| part.strip_prefix(" <w:t").or_else(|| part.strip_prefix(" <t"))).map(|part| part.split('>').last().unwrap_or(part)).collect::<Vec<_>>().join(" ")) }
        _ => Err("仅支持 Markdown、TXT、PDF、DOCX、XLSX".into()),
    }
}

fn import_file(request: &ImportRequest) -> Result<String, String> {
    let source = PathBuf::from(&request.path);
    if !source.exists() { return Err("文件不存在".into()); }
    let extension = source.extension().and_then(|e| e.to_str()).unwrap_or_default().to_lowercase();
    if extension != "md" && extension != "txt" && extension != "markdown" && extension != "pdf" && extension != "docx" && extension != "xlsx" { return Err("仅支持 Markdown、TXT、PDF、DOCX、XLSX".into()); }
    let content = extract_import_text(&source, &extension)?;
    if content.len() > 2_000_000 { return Err("文件超过 2MB 限制".into()); }
    let originals_dir = data_root().join("originals");
    fs::create_dir_all(&originals_dir).map_err(|e| e.to_string())?;
    let file_name = source.file_name().and_then(|n| n.to_str()).unwrap_or("import").to_string();
    let copy_path = originals_dir.join(format!("{}-{}", chrono_like_now(), file_name));
    fs::copy(&source, &copy_path).map_err(|e| format!("复制原始文件失败: {e}"))?;
    let title = source.file_stem().and_then(|n| n.to_str()).unwrap_or("导入文档").to_string();
    let card = NewCard { title: title.clone(), summary: "从本地文件导入".into(), card_type: "知识".into(), tags: vec![extension], source: "本地文件导入".into(), visibility: default_visibility(), status: "待验证".into(), content: content.clone(), actor: if request.actor.is_empty() { "desktop-user".into() } else { request.actor.clone() } };
    let db = db_connection()?; let id = insert_card(&db, &card, Some(&copy_path.to_string_lossy()), format!("import-{}", chrono_like_now()))?;
    audit(&db, &card.actor, "import_card", Some(&id), "import local document")?;
    Ok(id)
}

fn create_backup() -> Result<String, String> {
    let root = data_root(); let backup_dir = root.join("backups"); fs::create_dir_all(&backup_dir).map_err(|e| e.to_string())?;
    let filename = format!("workbench-{}.zip", chrono_like_now()); let target = backup_dir.join(filename);
    let file = fs::File::create(&target).map_err(|e| format!("创建备份文件失败: {e}"))?;
    let mut archive = zip::ZipWriter::new(file); let options = zip::write::SimpleFileOptions::default().compression_method(zip::CompressionMethod::Deflated);
    let include = ["workbench.db", "cards", "originals", "assets", "inbox"];
    for entry in include {
        let path = root.join(entry);
        if path.is_file() { add_file_to_zip(&mut archive, &path, entry, options)?; }
        else if path.is_dir() { add_dir_to_zip(&mut archive, &path, entry, options)?; }
    }
    archive.finish().map_err(|e| format!("完成备份失败: {e}"))?;
    Ok(target.to_string_lossy().to_string())
}
fn add_file_to_zip<W: Write + std::io::Seek>(archive: &mut zip::ZipWriter<W>, path: &PathBuf, name: &str, options: zip::write::SimpleFileOptions) -> Result<(), String> { archive.start_file(name, options).map_err(|e| e.to_string())?; let mut file = fs::File::open(path).map_err(|e| e.to_string())?; std::io::copy(&mut file, archive).map_err(|e| e.to_string())?; Ok(()) }
fn add_dir_to_zip<W: Write + std::io::Seek>(archive: &mut zip::ZipWriter<W>, dir: &PathBuf, prefix: &str, options: zip::write::SimpleFileOptions) -> Result<(), String> { for entry in fs::read_dir(dir).map_err(|e| e.to_string())? { let path = entry.map_err(|e| e.to_string())?.path(); let name = format!("{}/{}", prefix, path.file_name().and_then(|v| v.to_str()).unwrap_or("file")); if path.is_dir() { add_dir_to_zip(archive, &path, &name, options)?; } else { add_file_to_zip(archive, &path, &name, options)?; } } Ok(()) }

fn start_local_api() {
    std::thread::spawn(|| {
        for attempt in 0..10 {
            match Server::http("127.0.0.1:37821") {
                Ok(server) => {
                    for request in server.incoming_requests() { handle_request(request); }
                    return;
                }
                Err(error) => {
                    eprintln!("local_api_bind_failed attempt={} error={}", attempt + 1, error);
                    std::thread::sleep(std::time::Duration::from_millis(500));
                }
            }
        }
        eprintln!("local_api_unavailable after retries");
    });
}

mod commands {
    use super::*;
    #[tauri::command] pub fn seed_workspace() -> Result<String, String> { let db = db_connection()?; seed_defaults(&db)?; let count: i64 = db.query_row("SELECT COUNT(*) FROM cards WHERE deleted_at IS NULL", [], |row| row.get(0)).map_err(|e| e.to_string())?; Ok(format!("workspace_ready:cards={count}:root={DATA_ROOT}")) }
    #[tauri::command] pub fn data_location() -> String { DATA_ROOT.to_string() }
    #[tauri::command] pub fn soft_delete_card(id: String) -> Result<(), String> { let mut conn = mysql_connection()?; conn.exec_drop("UPDATE cards SET deleted_at=NOW(6), updated_at=NOW(6) WHERE id=:id AND deleted_at IS NULL", mysql::Params::Named(std::collections::HashMap::from([(b"id".to_vec(), mysql::Value::from(id.clone()))]))).map_err(|e| e.to_string())?; if conn.affected_rows() == 0 { return Err("卡片不存在或已在回收站".into()); } conn.exec_drop("INSERT INTO audit_log(actor,action,target_id,detail,created_at) VALUES('desktop-user','soft_delete',:id,'move card to recycle bin',NOW(6))", mysql::Params::Named(std::collections::HashMap::from([(b"id".to_vec(), mysql::Value::from(id))]))).map_err(|e| e.to_string())?; Ok(()) }
    #[tauri::command] pub fn restore_card(id: String) -> Result<(), String> { let mut conn = mysql_connection()?; conn.exec_drop("UPDATE cards SET deleted_at=NULL, updated_at=NOW(6) WHERE id=:id AND deleted_at IS NOT NULL", mysql::Params::Named(std::collections::HashMap::from([(b"id".to_vec(), mysql::Value::from(id.clone()))]))).map_err(|e| e.to_string())?; if conn.affected_rows() == 0 { return Err("回收站中没有这张卡片".into()); } conn.exec_drop("INSERT INTO audit_log(actor,action,target_id,detail,created_at) VALUES('desktop-user','restore_card',:id,'restore card',NOW(6))", mysql::Params::Named(std::collections::HashMap::from([(b"id".to_vec(), mysql::Value::from(id))]))).map_err(|e| e.to_string())?; Ok(()) }
    #[tauri::command] pub fn permanently_delete_card(id: String) -> Result<(), String> { let mut conn = mysql_connection()?; conn.query_drop("START TRANSACTION").map_err(|e| e.to_string())?; let result = (|| { conn.exec_drop("DELETE FROM card_relations WHERE from_card_id=:id OR to_card_id=:id", mysql::Params::Named(std::collections::HashMap::from([(b"id".to_vec(), mysql::Value::from(id.clone()))]))).map_err(|e| e.to_string())?; conn.exec_drop("DELETE FROM card_versions WHERE card_id=:id", mysql::Params::Named(std::collections::HashMap::from([(b"id".to_vec(), mysql::Value::from(id.clone()))]))).map_err(|e| e.to_string())?; conn.exec_drop("DELETE FROM card_contents WHERE card_id=:id", mysql::Params::Named(std::collections::HashMap::from([(b"id".to_vec(), mysql::Value::from(id.clone()))]))).map_err(|e| e.to_string())?; conn.exec_drop("DELETE FROM audit_log WHERE target_id=:id", mysql::Params::Named(std::collections::HashMap::from([(b"id".to_vec(), mysql::Value::from(id.clone()))]))).map_err(|e| e.to_string())?; conn.exec_drop("DELETE FROM cards WHERE id=:id AND deleted_at IS NOT NULL", mysql::Params::Named(std::collections::HashMap::from([(b"id".to_vec(), mysql::Value::from(id))]))).map_err(|e| e.to_string())?; if conn.affected_rows() == 0 { return Err("只允许永久删除回收站中的卡片".into()); } Ok::<(),String>(()) })(); match result { Ok(()) => { conn.query_drop("COMMIT").map_err(|e| e.to_string())?; Ok(()) }, Err(e) => { let _ = conn.query_drop("ROLLBACK"); Err(e) } } }
    #[tauri::command] pub fn mysql_status() -> Result<serde_json::Value, String> { let mut conn = mysql_connection()?; let version: String = conn.query_first("SELECT VERSION()").map_err(|e| format!("读取 MySQL 版本失败: {e}"))?.ok_or_else(|| "MySQL 未返回版本".to_string())?; Ok(serde_json::json!({"ok":true,"host":"127.0.0.1","port":3306,"database":MYSQL_DATABASE,"user":MYSQL_USER,"version":version})) }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() { start_local_api(); tauri::Builder::default().plugin(tauri_plugin_opener::init()).plugin(tauri_plugin_dialog::init()).invoke_handler(tauri::generate_handler![commands::seed_workspace, commands::data_location, commands::soft_delete_card, commands::restore_card, commands::permanently_delete_card, commands::mysql_status]).run(tauri::generate_context!()).expect("error while running tauri application"); }
