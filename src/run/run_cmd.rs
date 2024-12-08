//From ab/SummerSK/puff/Aestas.Britannia
use log::info;
use std::process::Command;
pub fn kill_app(package_name: &str) {
    let _ = Command::new("nohup")
        .args(["am", "force-stop", package_name, ">/dev/null", "2>&1", "&"])
        .output();
}

pub fn precise_kill(package_name: &str) {
    let output = Command::new("sh")
        .args([
            "-c",
            "ps -ef | grep com.xunmeng.pinduoduo | grep -v grep | wc -l",
        ])
        .output()
        .unwrap();
    if output.clone().status.success() {
        // 将输出转换为字符串并打印
        let output_str: &str = &String::from_utf8_lossy(&output.stdout);
        let output_str = output_str.trim();
        let output_str: i32 = output_str.parse().unwrap_or_default();
        if output_str < 4 && output_str > 0 {
            info!("清算{}", package_name);
            kill_app(package_name);
        }
    } else {
        // 如果命令执行失败，打印错误信息
        let error_str = String::from_utf8_lossy(&output.stderr);
        println!("Error: {}", error_str);
    }
}
