use regex::Regex;
use serde::Deserialize;
use serde::Serialize;
use std::fs;
use std::io;
use std::path::Path;
use tauri::{command, AppHandle, Emitter};

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")] // ここでキャメルケースに変換
struct ScanRequest {
    root_path: String,
    expected_depth: u32,
    limit_depth: Option<u32>,
    dir_conds: Vec<DirCond>,
}
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")] // ここでキャメルケースに変換
struct DirCond {
    pattern: String,
    depth: Option<u32>,
    is_exclusion: bool,
}

#[command]
fn start_long_task(app: AppHandle, req: ScanRequest) -> Result<String, String> {
    println!("{:?}", req);
    let root = &req.root_path;
    let mut counter: u32 = 0;
    let mut result: String = String::new();
    search_file_rec(
        root,
        &app,
        0,
        &mut counter,
        &req.dir_conds,
        req.limit_depth,
        &mut result,
    )
    .expect("expect_err expect_err");
    // 完了通知
    app.emit("progress_done", true).unwrap();

    Ok(result)
}

#[derive(Serialize)]
struct Progress {
    path: String,
    name: String,
    depth: u32,
    counter: u32,
}

fn search_file_rec(
    dir: &str,
    app: &AppHandle,
    depth: u32,
    counter: &mut u32,
    dir_conds: &Vec<DirCond>,
    limit_depth: Option<u32>,
    result: &mut String,
) -> Result<(), io::Error> {
    *counter += 1;

    let name = Path::new(dir).file_name().unwrap().to_str().unwrap();
    // println!("{:?}", dir_conds);
    let is_accept = dir_conds.iter().all(|cond| {
        if cond.depth.map_or(true, |d| d != depth) {
            return true;
        }
        wildcard_match(&cond.pattern, name) != cond.is_exclusion
    });
    if depth == 1 {
        println!("{}---{}: {}", depth, is_accept, name);
    }
    if !is_accept {
        println!("unmatch! {}", name);
        return Ok(());
    }
    let data = Progress {
        path: dir.to_string(),
        name: name.to_string(),
        depth,
        counter: *counter,
    };
    // フロントにイベント送信
    app.emit("progress", &data).unwrap();

    let dir_entries = fs::read_dir(dir)?;
    for file in dir_entries {
        let file = file.ok().unwrap();
        let path = file.path();

        if path.is_dir() {
            // リミットが設定されている場合に限り、リミットに達していたらそれ以上深く走査しない
            if let Some(limit) = limit_depth {
                if depth == limit {
                    return Ok(());
                }
            }
            search_file_rec(
                &path.display().to_string(),
                app,
                depth + 1,
                counter,
                dir_conds,
                limit_depth,
                result,
            )?;
        } else {
            result.push_str(&format!(
                "{}\n",
                // path.file_name().unwrap().to_str().unwrap()
                path.to_str().unwrap()
            ));
        }
    }
    Ok(())
}

/// ワイルドカード文字列 (「*」のみ) を正規表現に変換してコンパイルした Regex を返す
fn compile_wildcard_pattern(pattern: &str) -> Regex {
    // 1. 正規表現特殊文字をエスケープ
    // 2. `*` を `.*` に置換
    let mut regex_str = String::from("^"); // 先頭固定
    for ch in pattern.chars() {
        match ch {
            // `*` は任意文字列にマッチ
            '*' => regex_str.push_str(".*"),
            // それ以外はエスケープして追加
            '.' | '+' | '?' | '^' | '$' | '(' | ')' | '[' | ']' | '{' | '}' | '|' | '\\' => {
                regex_str.push('\\');
                regex_str.push(ch);
            }
            _ => regex_str.push(ch),
        }
    }
    regex_str.push('$'); // 終端固定
    Regex::new(&regex_str).expect("正規表現のコンパイルに失敗")
}

/// ワイルドカードパターンで文字列を判定
fn wildcard_match(pattern: &str, text: &str) -> bool {
    compile_wildcard_pattern(pattern).is_match(text)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![start_long_task])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
