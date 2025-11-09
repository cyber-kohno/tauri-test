import { invoke } from "@tauri-apps/api/core";

/**
 * greet コマンド呼び出し
 */
export async function greet(name: string): Promise<string> {
  return invoke<string>("greet", { name });
}

/**
 * ファイル解析コマンド
 */
export async function analyzeFiles(
  paths: string[],
): Promise<{ total: number; success: number }> {
  return invoke("analyze_files", { paths });
}
