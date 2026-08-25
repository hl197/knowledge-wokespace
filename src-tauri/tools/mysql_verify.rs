use keyring::Entry;
use mysql::{prelude::Queryable, Opts, Pool};

fn main() -> Result<(), String> {
    let password = Entry::new_with_target("HermesWorkbench/MySQL", "HermesWorkbench/MySQL", "root")
        .map_err(|e| e.to_string())?.get_password().map_err(|e| e.to_string())?;
    let opts = mysql::OptsBuilder::new().ip_or_hostname(Some("127.0.0.1")).tcp_port(3306).user(Some("root")).pass(Some(password)).db_name(Some("personal_ai_workbench"));
    let pool = Pool::new(Opts::from(opts)).map_err(|e| e.to_string())?;
    let mut conn = pool.get_conn().map_err(|e| e.to_string())?;
    let counts: (u64,u64,u64,u64,u64,u64) = conn.query_first("SELECT (SELECT COUNT(*) FROM cards),(SELECT COUNT(*) FROM card_contents),(SELECT COUNT(*) FROM card_versions),(SELECT COUNT(*) FROM card_relations),(SELECT COUNT(*) FROM audit_log),(SELECT COUNT(*) FROM file_objects)").map_err(|e| e.to_string())?.ok_or_else(|| "no count row".to_string())?;
    println!("mysql_readback cards={} contents={} versions={} relations={} audit={} files={}", counts.0,counts.1,counts.2,counts.3,counts.4,counts.5);
    Ok(())
}
