use chrono::{ Duration, Utc };
use tokio::time::{ sleep_until, Instant };
use super::db::get_db;

pub async fn daily_task() {
    loop {
        // 运行任务
        match get_db().remove_expired_snippets() {
            Ok(count) => tracing::info!("已删除过期 snippet: {} 条", count),
            Err(e) => tracing::error!("清理失败: {}", e),
        }

        // 算出下一次执行的时间（+1 天）
        let now = Utc::now();
        let next_run = now + Duration::days(1);

        // 转换成 tokio Instant
        let dur = (next_run - now).to_std().unwrap();
        let next_instant = Instant::now() + dur;

        // 等到下一次
        sleep_until(next_instant).await;
    }
}
