use crate::{
    _dev_utils::seed_posts,
    ctx::Ctx,
    model::{
        ModelManager,
        user::{User, UserBmc},
    },
};
use sqlx::{Pool, Postgres, postgres::PgPoolOptions};
use std::{
    fs,
    path::{Path, PathBuf},
    time::Duration,
};
use tracing::info;

type Db = Pool<Postgres>;

// NOTE: Hardcode to prevent deployed system db update.
const PG_DEV_POSTGRES_URL: &str = "postgres://postgres:welcome@127.0.0.1:5433/postgres";
const PG_DEV_APP_URL: &str = "postgres://app_user:dev_only_pwd@127.0.0.1:5433/app_db";

// sql dirs/files
const SQL_RECREATE_DB_FILE_NAME: &str = "00-recreate-db.sql";
const DEV_SQL_DIR: &str = "sql/dev_initial";
const MIGRATIONS_DIR: &str = "sql/migrations";

const DEMO_PWD: &str = "welcome";

pub async fn init_dev_db() -> Result<(), Box<dyn std::error::Error>> {
    info!("{:<12} - init_dev_db()", "FOR-DEV-ONLY");

    // -- Resolve project root dir (cargo run / cargo test safe)
    let current_dir = std::env::current_dir().unwrap();
    let parts: Vec<_> = current_dir.components().collect();
    let base_dir = if parts.get(parts.len().wrapping_sub(3)).map(|c| c.as_os_str() == "crates") == Some(true) {
        parts[..parts.len() - 3].iter().collect::<PathBuf>()
    } else {
        current_dir.clone()
    };

    let dev_sql_dir = base_dir.join(DEV_SQL_DIR);
    let migrations_dir = base_dir.join(MIGRATIONS_DIR);

    // -- Recreate database (postgres db)
    {
        let recreate_file = dev_sql_dir.join(SQL_RECREATE_DB_FILE_NAME);
        let root_db = new_db_pool(PG_DEV_POSTGRES_URL).await?;
        pexec(&root_db, &recreate_file).await?;
    }

    // -- App database pool
    let app_db = new_db_pool(PG_DEV_APP_URL).await?;

    // -- Run migrations (schema only)
    let mut migrations: Vec<PathBuf> = fs::read_dir(migrations_dir)?.filter_map(|e| e.ok().map(|e| e.path())).collect();
    migrations.sort();

    for path in migrations {
        if path.extension().and_then(|s| s.to_str()) == Some("sql") {
            pexec(&app_db, &path).await?;
        }
    }

    // -- Run dev-only seed data
    let mut dev_files: Vec<PathBuf> = fs::read_dir(dev_sql_dir)?.filter_map(|e| e.ok().map(|e| e.path())).collect();
    dev_files.sort();

    for path in dev_files {
        if path.file_name().and_then(|s| s.to_str()) == Some(SQL_RECREATE_DB_FILE_NAME) {
            continue;
        }

        if path.extension().and_then(|s| s.to_str()) == Some("sql") {
            pexec(&app_db, &path).await?;
        }
    }

    // -- Init model layer
    let mm = ModelManager::new().await?;
    let ctx = Ctx::root_ctx();

    // -- Set demo0 password
    let demo0_user: User = UserBmc::first_by_username(&ctx, &mm, "demo0")
        .await?
        .expect("demo0 user must exist in dev seed");
    UserBmc::update_pwd_hash(&ctx, &mm, &demo0_user.id, DEMO_PWD.to_string()).await?;

    let titles = ["title_1", "title_2", "title_3"];
    let descriptions = ["description_1", "description_2", "description_3"];

    seed_posts(&ctx, &mm, &titles, &descriptions).await?;

    info!("{:<12} - init_dev_db finished successfully", "FOR-DEV-ONLY");

    Ok(())
}

async fn pexec(db: &Db, file: &Path) -> Result<(), sqlx::Error> {
    info!("{:<12} - pexec: {:?}", "FOR-DEV-ONLY", file);

    let content = fs::read_to_string(file)?;

    for sql in content.split(';') {
        let sql = sql.trim();
        if sql.is_empty() {
            continue;
        }

        sqlx::query(sql).execute(db).await.map_err(|e| {
            println!("pexec error while running:\n{sql}");
            println!("cause:\n{e}");
            e
        })?;
    }

    Ok(())
}

async fn new_db_pool(db_url: &str) -> Result<Db, sqlx::Error> {
    PgPoolOptions::new()
        .max_connections(1)
        .acquire_timeout(Duration::from_millis(500))
        .connect(db_url)
        .await
}
