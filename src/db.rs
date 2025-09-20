use std::env;
use std::sync::{ Mutex, OnceLock };
use chrono::{ DateTime, Utc, Duration };
use rusqlite::{ Connection, params, Result };
use nanoid::nanoid;
use tracing::{ info, warn, error, instrument, trace };

use crate::models::{ Snippet, CreateSnippetRequest };

/// 数据库管理器
pub struct SnippetDatabase {
    connection: Mutex<Connection>,
}

impl SnippetDatabase {
    /// 创建新的数据库管理器实例
    fn new() -> Result<Self, Box<dyn std::error::Error>> {
        trace!("尝试加载 .env 文件");
        dotenvy::dotenv().ok();
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL 环境变量未设置");
        info!("正在连接到数据库: {}", database_url);
        let connection = Connection::open(database_url)?;
        info!("成功连接到数据库.");

        Ok(SnippetDatabase {
            connection: Mutex::new(connection),
        })
    }

    /// 获取全局数据库实例
    pub fn instance() -> &'static SnippetDatabase {
        static DB: OnceLock<SnippetDatabase> = OnceLock::new();
        DB.get_or_init(|| {
            info!("正在初始化全局数据库实例.");
            SnippetDatabase::new().expect("初始化数据库失败")
        })
    }

    /// 初始化数据库表
    #[instrument(skip(self))]
    pub fn init(&self) -> Result<(), Box<dyn std::error::Error>> {
        info!("正在初始化数据库表.");
        let query =
            r#"
            CREATE TABLE IF NOT EXISTS snippets (
                id TEXT PRIMARY KEY,
                title TEXT NOT NULL,
                language TEXT NOT NULL,
                content TEXT NOT NULL,
                description TEXT,
                password_protected INTEGER NOT NULL,
                password TEXT,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL,
                expires_at TEXT,
                author TEXT,
                view_count INTEGER NOT NULL DEFAULT 0,
                tags TEXT,
                visibility INTEGER NOT NULL
            );
        "#;

        let conn = self.connection.lock().unwrap();
        match conn.execute(query, []) {
            Ok(_) => {
                info!("数据库表初始化成功.");
                Ok(())
            }
            Err(e) => {
                error!("初始化数据库表失败: {:?}", e);
                Err(Box::new(e))
            }
        }
    }

    /// 创建 snippet
    #[instrument(skip(self, req))]
    pub fn create_snippet(&self, req: CreateSnippetRequest) -> Result<Snippet> {
        info!("正在创建新的代码片段，标题: {}", req.title);
        let now: DateTime<Utc> = Utc::now();
        let expires_at = req.expires_in_seconds.map(|sec| now + Duration::seconds(sec.into()));

        let snippet = Snippet {
            id: nanoid!(6),
            title: req.title,
            language: req.language,
            content: req.content,
            description: req.description.unwrap_or_default(),
            password_protected: req.password.is_some(),
            password: req.password,
            created_at: now,
            updated_at: now,
            expires_at,
            author: req.author,
            view_count: 0,
            tags: req.tags,
            visibility: req.visibility.unwrap_or(0),
        };

        let conn = self.connection.lock().unwrap();
        match
            conn.execute(
                r#"
            INSERT INTO snippets (
                id, title, language, content, description, password_protected, password,
                created_at, updated_at, expires_at, author, view_count, tags, visibility
            ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14)
            "#,
                params![
                    snippet.id,
                    snippet.title,
                    snippet.language,
                    snippet.content,
                    snippet.description,
                    snippet.password_protected as i32,
                    snippet.password,
                    snippet.created_at.to_rfc3339(),
                    snippet.updated_at.to_rfc3339(),
                    snippet.expires_at.map(|dt| dt.to_rfc3339()),
                    snippet.author,
                    snippet.view_count,
                    snippet.tags,
                    snippet.visibility
                ]
            )
        {
            Ok(_) => {
                info!("代码片段 {} 创建成功.", snippet.id);
                Ok(snippet)
            }
            Err(e) => {
                error!("创建代码片段失败: {:?}", e);
                Err(e)
            }
        }
    }

    /// 获取 snippet
    #[instrument(skip(self), fields(snippet_id = id))]
    pub fn get_snippet(&self, id: &str) -> Result<Option<Snippet>> {
        trace!("尝试获取 ID 为 {} 的代码片段.", id);
        let conn = self.connection.lock().unwrap();
        let mut stmt = conn.prepare(
            r#"
            SELECT
                id, title, language, content, description, password_protected, password,
                created_at, updated_at, expires_at, author, view_count, tags, visibility
            FROM snippets
            WHERE id = ?1
            "#
        )?;

        let mut rows = stmt.query(params![id])?;

        if let Some(row) = rows.next()? {
            info!("找到 ID 为 {} 的代码片段.", id);
            let snippet = Snippet {
                id: row.get(0)?,
                title: row.get(1)?,
                language: row.get(2)?,
                content: row.get(3)?,
                description: row.get(4)?,
                password_protected: row.get::<_, i32>(5)? != 0,
                password: row.get(6)?,
                created_at: DateTime::parse_from_rfc3339(&row.get::<_, String>(7)?)
                    .unwrap()
                    .with_timezone(&Utc),
                updated_at: DateTime::parse_from_rfc3339(&row.get::<_, String>(8)?)
                    .unwrap()
                    .with_timezone(&Utc),
                expires_at: match row.get::<_, Option<String>>(9)? {
                    Some(dt) =>
                        Some(DateTime::parse_from_rfc3339(&dt).unwrap().with_timezone(&Utc)),
                    None => None,
                },
                author: row.get(10)?,
                view_count: row.get(11)?,
                tags: row.get(12)?,
                visibility: row.get(13)?,
            };
            Ok(Some(snippet))
        } else {
            warn!("未找到 ID 为 {} 的代码片段.", id);
            Ok(None)
        }
    }

    /// 删除 snippet
    #[instrument(skip(self), fields(snippet_id = id))]
    pub fn delete_snippet(&self, id: &str) -> Result<(), Box<dyn std::error::Error>> {
        info!("尝试删除 ID 为 {} 的代码片段.", id);
        let conn = self.connection.lock().unwrap();
        match conn.execute("DELETE FROM snippets WHERE id = ?1", params![id]) {
            Ok(rows) => {
                if rows > 0 {
                    info!("成功删除了 {} 个代码片段，ID 为 {}.", rows, id);
                } else {
                    warn!("未删除任何代码片段，ID 为 {} 的记录可能不存在.", id);
                }
                Ok(())
            }
            Err(e) => {
                error!("删除 ID 为 {} 的代码片段失败: {:?}", id, e);
                Err(Box::new(e))
            }
        }
    }

    /// 删除所有过期的代码片段
    #[instrument(skip(self))]
    pub fn remove_expired_snippets(&self) -> Result<usize, Box<dyn std::error::Error>> {
        info!("正在移除所有过期的代码片段.");
        let now = Utc::now().to_rfc3339();

        let conn = self.connection.lock().unwrap();
        // 删除 expires_at 小于当前时间的记录
        match
            conn.execute("DELETE FROM snippets WHERE expires_at IS NOT NULL AND expires_at < ?1", [
                &now,
            ])
        {
            Ok(rows_deleted) => {
                info!("成功删除了 {} 个过期的代码片段.", rows_deleted);
                Ok(rows_deleted)
            }
            Err(e) => {
                error!("移除过期代码片段失败: {:?}", e);
                Err(Box::new(e))
            }
        }
    }

    /// 增加查看次数
    #[instrument(skip(self), fields(snippet_id = id))]
    pub fn increment_view_count(&self, id: &str) -> Result<(), Box<dyn std::error::Error>> {
        info!("正在为 ID 为 {} 的代码片段增加查看次数.", id);
        let conn = self.connection.lock().unwrap();
        match
            conn.execute(
                "UPDATE snippets SET view_count = view_count + 1 WHERE id = ?1",
                params![id]
            )
        {
            Ok(_) => {
                info!("ID 为 {} 的代码片段查看次数已增加.", id);
                Ok(())
            }
            Err(e) => {
                error!("增加 ID 为 {} 的代码片段查看次数失败: {:?}", id, e);
                Err(Box::new(e))
            }
        }
    }

    /// 更新 snippet
    #[instrument(skip(self, req), fields(snippet_id = id))]
    pub fn update_snippet(
        &self,
        id: &str,
        req: CreateSnippetRequest
    ) -> Result<bool, Box<dyn std::error::Error>> {
        info!("正在更新 ID 为 {} 的代码片段.", id);
        let now = Utc::now();
        let expires_at = req.expires_in_seconds.map(|sec| now + Duration::seconds(sec.into()));

        let conn = self.connection.lock().unwrap();
        match
            conn.execute(
                r#"
            UPDATE snippets SET
                title = ?1,
                language = ?2,
                content = ?3,
                description = ?4,
                password_protected = ?5,
                password = ?6,
                updated_at = ?7,
                expires_at = ?8,
                author = ?9,
                tags = ?10,
                visibility = ?11
            WHERE id = ?12
            "#,
                params![
                    req.title,
                    req.language,
                    req.content,
                    req.description.unwrap_or_default(),
                    req.password.is_some() as i32,
                    req.password,
                    now.to_rfc3339(),
                    expires_at.map(|dt| dt.to_rfc3339()),
                    req.author,
                    req.tags,
                    req.visibility.unwrap_or(0),
                    id
                ]
            )
        {
            Ok(rows_affected) => {
                if rows_affected > 0 {
                    info!("成功更新了 ID 为 {} 的代码片段.", id);
                    Ok(true)
                } else {
                    warn!("更新 ID 为 {} 的代码片段失败，该记录可能不存在.", id);
                    Ok(false)
                }
            }
            Err(e) => {
                error!("更新 ID 为 {} 的代码片段时发生错误: {:?}", id, e);
                Err(Box::new(e))
            }
        }
    }

    /// 获取公开的 snippets 列表（分页）
    #[instrument(skip(self), fields(limit, offset))]
    pub fn get_public_snippets(
        &self,
        limit: i32,
        offset: i32
    ) -> Result<Vec<Snippet>, Box<dyn std::error::Error>> {
        info!("正在获取公开的代码片段列表，限制 {} 条，偏移量 {}.", limit, offset);
        let conn = self.connection.lock().unwrap();
        let mut stmt = conn.prepare(
            r#"
            SELECT
                id, title, language, content, description, password_protected, password,
                created_at, updated_at, expires_at, author, view_count, tags, visibility
            FROM snippets
            WHERE visibility = 0 AND (expires_at IS NULL OR expires_at > datetime('now'))
            ORDER BY created_at DESC
            LIMIT ?1 OFFSET ?2
            "#
        )?;

        let snippet_iter = stmt.query_map(params![limit, offset], |row| {
            Ok(Snippet {
                id: row.get(0)?,
                title: row.get(1)?,
                language: row.get(2)?,
                content: row.get(3)?,
                description: row.get(4)?,
                password_protected: row.get::<_, i32>(5)? != 0,
                password: row.get(6)?,
                created_at: DateTime::parse_from_rfc3339(&row.get::<_, String>(7)?)
                    .unwrap()
                    .with_timezone(&Utc),
                updated_at: DateTime::parse_from_rfc3339(&row.get::<_, String>(8)?)
                    .unwrap()
                    .with_timezone(&Utc),
                expires_at: match row.get::<_, Option<String>>(9)? {
                    Some(dt) =>
                        Some(DateTime::parse_from_rfc3339(&dt).unwrap().with_timezone(&Utc)),
                    None => None,
                },
                author: row.get(10)?,
                view_count: row.get(11)?,
                tags: row.get(12)?,
                visibility: row.get(13)?,
            })
        })?;

        let mut snippets = Vec::new();
        for snippet in snippet_iter {
            snippets.push(snippet?);
        }
        info!("成功检索到 {} 个公开代码片段.", snippets.len());
        Ok(snippets)
    }
}

pub fn init_db() -> Result<(), Box<dyn std::error::Error>> {
    info!("正在启动数据库初始化进程.");
    let db = SnippetDatabase::instance();
    db.init()
}

// 全局实例访问的便捷函数
pub fn get_db() -> &'static SnippetDatabase {
    SnippetDatabase::instance()
}
