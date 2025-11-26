use encoding_rs::SHIFT_JIS; // ← Shift‑JIS を取り込む
use encoding_rs_io::DecodeReaderBytesBuilder;
use regex::Regex;
use serde::Deserialize;
use serde::Serialize;
use std::fs;
use std::fs::File;
use std::io;
use std::io::BufReader;
use std::io::Read;
use std::path::Path;
use tauri::{command, AppHandle, Emitter};

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")] // ここでキャメルケースに変換
struct ScanRequest {
    root_path: String,
    expected_depth: u32,
    limit_depth: Option<u32>,
    dir_conds: Vec<DirCond>,
    file_conds: Vec<FileCond>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct BaseCond {
    pattern: String,
    is_exclusion: bool,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
/// ディレクトリ検索条件
struct DirCond {
    #[serde(flatten)]
    base: BaseCond,
    depth: Option<u32>,
}
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
/// ファイル検索条件
struct FileCond {
    #[serde(flatten)]
    base: BaseCond,
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
struct ScanResponse {
    result: String,
    node: Node,
}
#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Node {
    name: String,
    children: Option<Vec<Node>>,
}

#[command]
fn scan_directory(app: AppHandle, req: ScanRequest) -> Result<ScanResponse, String> {
    println!("{}", req.root_path);
    println!("{:?}", req);
    let root = &req.root_path;
    let mut counter: u32 = 0;
    let mut res: ScanResponse = ScanResponse {
        result: String::new(),
        node: Node {
            name: "root".to_string(),
            children: None,
        },
    };
    let node = search_file_rec(root, &app, 0, &mut counter, &req).unwrap();
    match node {
        Some(n) => res.node = n,
        None => {}
    }
    // // 完了通知
    app.emit("progress_done", true).unwrap();

    Ok(res)
}

#[derive(Serialize)]
struct Progress {
    path: String,
    name: String,
    depth: u32,
    counter: u32,
}
/// 再帰的にディレクトリを検索する
fn search_file_rec(
    dir: &str,
    app: &AppHandle,
    depth: u32,
    counter: &mut u32,
    req: &ScanRequest,
) -> Result<Option<Node>, io::Error> {
    *counter += 1;

    let name = Path::new(dir).file_name().unwrap().to_str().unwrap();
    // println!("{:?}", dir_conds);
    let is_accept = req.dir_conds.iter().all(|cond| {
        let is_match = wildcard_match(&cond.base.pattern, name);
        let is_exclusion = cond.base.is_exclusion;
        if is_match {
            match cond.depth {
                Some(d) => !is_exclusion && d == depth,
                None => !is_exclusion,
            }
        } else {
            is_exclusion
        }
    });
    if depth == 1 {
        println!("{}---{}: {}", depth, is_accept, name);
    }
    if !is_accept {
        println!("unmatch! {}", name);
        return Ok(Option::None);
    }
    let mut children: Vec<Node> = Vec::new();
    let mut node: Node = Node {
        name: name.to_string(),
        children: None,
    };
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
            if let Some(limit) = req.limit_depth {
                if depth == limit {
                    return Ok(Option::None);
                }
            }
            let child = search_file_rec(&path.display().to_string(), app, depth + 1, counter, req)?;
            match child {
                Some(n) => children.push(n),
                None => {}
            };
        } else {
            let name = path.file_name().unwrap().to_str().unwrap().to_string();

            let is_accept = req.file_conds.iter().all(|cond: &FileCond| {
                wildcard_match(&cond.base.pattern, &name) != cond.base.is_exclusion
            });
            if is_accept {
                children.push(Node {
                    name,
                    children: None,
                });
            }
        }
    }
    node.children = Some(children);
    Ok(Option::Some(node))
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

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")] // ここでキャメルケースに変換
struct FileRequest {
    file_path: String,
}

#[command]
fn read_file(req: FileRequest) -> String {
    println!("{}", req.file_path);
    let content = match read_file_to_string(req.file_path) {
        Ok(text) => text,
        Err(err) => err.to_string(),
    };
    content
}

fn read_file_to_string<P: AsRef<std::path::Path>>(path: P) -> Result<String, io::Error> {
    let file = File::open(path)?;
    let mut buf_reader = BufReader::new(file);

    // デフォルトでは UTF‑8 が前提です。別エンコーディングなら `DecodeReaderBytesBuilder::new().encoding(Some(Encoding::for_label(b"shift_jis").unwrap()))` で指定できます
    // let mut decoder = DecodeReaderBytesBuilder::new().build(&mut buf_reader);
    let mut decoder = DecodeReaderBytesBuilder::new()
        .encoding(Some(SHIFT_JIS)) // ← ここで SJIS を指定
        .build(&mut buf_reader); // ← デコーダを作成

    let mut s = String::new();
    decoder.read_to_string(&mut s)?;
    Ok(s)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![scan_directory, read_file])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
