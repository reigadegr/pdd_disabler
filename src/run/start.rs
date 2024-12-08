use crate::shared::get_top_app::get_topapp_pid_and_name;
use anyhow::Result;
use log::info;
use std::path::Path;
use std::thread;
use tokio::time::Duration;
pub async fn thread_start() -> Result<()> {
    // 启动运行线程任务
    let run_thread = tokio::spawn(async move { app_run() });

    // 等待
    let _ = run_thread.await?;

    Ok(())
}

fn app_run() -> Result<()> {
    let mut global_package = String::new();
    loop {
        let (_, name) = get_topapp_pid_and_name()?;

        if global_package == name {
            thread::sleep(Duration::from_millis(1000));
            continue;
        }
        global_package = name.clone();

        info!("日常app: {}", name);
        thread::sleep(Duration::from_millis(1000));
    }
}
