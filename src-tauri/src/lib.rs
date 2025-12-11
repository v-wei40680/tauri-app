// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}
use tauri_plugin_shell::ShellExt; // 导入扩展 Trait

#[tauri::command]
async fn run_file_list_command(app_handle: tauri::AppHandle) -> Result<String, String> {
    // 1. 根据平台确定命令
    let (cmd, args) = if cfg!(target_os = "windows") {
        ("cmd", vec!["/c", "dir"])
    } else {
        ("ls", vec!["-aF"])
    };

    // 2. 通过 app_handle.shell() 创建命令
    let output = app_handle
        .shell()
        .command(cmd)
        .args(args)
        .output()
        .await
        .map_err(|e| format!("执行失败: {}", e))?;

    // 3. 检查执行状态并返回结果
    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    } else {
        Err(String::from_utf8_lossy(&output.stderr).to_string())
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![run_file_list_command, greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
