use clap::Parser;
use std::{
    fs::{File, create_dir_all, read_to_string},
    io,
    path::PathBuf,
};

/// DicStructureCreater: 從樹狀文字產生資料夾與空檔
#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Opt {
    /// 結構檔 (txt / md)
    #[arg(value_name = "STRUCTURE_FILE")]
    structure: PathBuf,

    /// 根目錄 (預設 = CWD)
    #[arg(short = 'r', long = "root", default_value = ".")]
    root: PathBuf,

    /// 覆蓋已存在檔案
    #[arg(short = 'O', long = "overwrite")]
    overwrite: bool,

    /// 只列出動作，不實際建立
    #[arg(short = 'd', long = "dry-run")]
    dry_run: bool,
}

fn main() -> io::Result<()> {
    let opt = Opt::parse();
    let content = read_to_string(&opt.structure)?;
    let mut stack: Vec<String> = Vec::new();

    for raw in content.lines() {
        // 0. 忽略空行 & 註解
        let trimmed = raw.trim_end();
        if trimmed.is_empty() || trimmed.trim_start().starts_with('#') {
            continue;
        }

        // 1. 計算前綴：字符數 & 對應 byte offset
        let (mut char_cnt, mut byte_cnt) = (0, 0);
        for (idx, ch) in trimmed.char_indices() {
            if matches!(ch, ' ' | '│' | '├' | '└' | '─') {
                char_cnt += 1;
                byte_cnt = idx + ch.len_utf8();
            } else {
                break;
            }
        }

        // 2. 偵測是否含有 tree 符號，以決定縮排單位
        let prefix_str = &trimmed[..byte_cnt];
        let has_tree_char = prefix_str
            .chars()
            .any(|c| matches!(c, '│' | '├' | '└' | '─'));
        let depth = if has_tree_char {
            char_cnt / 4
        } else {
            char_cnt / 2
        };
        stack.truncate(depth);

        // 3. 去除前綴與 Markdown list 符號
        let mut line = &trimmed[byte_cnt..];
        line = line.trim_start_matches(&[' ', '-', '*'][..]).trim();
        if line.is_empty() {
            continue;
        }

        // 4. 判斷是資料夾 or 檔案
        let is_dir = line.ends_with('/');
        let name = if is_dir {
            &line[..line.len() - 1]
        } else {
            line
        };
        stack.push(name.to_string());

        // 5. 建立相對路徑
        let rel_path = stack.iter().fold(PathBuf::new(), |mut acc, seg| {
            acc.push(seg);
            acc
        });
        let target = opt.root.join(&rel_path);

        // 6. dry-run 模式
        if opt.dry_run {
            println!("{} {}", if is_dir { "📁" } else { "📄" }, target.display());
            continue;
        }

        // 7. 真實執行：建立資料夾或檔案
        if is_dir {
            if !target.exists() {
                create_dir_all(&target)?;
                println!("📂 建立資料夾: {}", target.display());
            }
        } else {
            if target.exists() && !opt.overwrite {
                println!("⏭️  跳過已存在: {}", target.display());
            } else {
                if let Some(parent) = target.parent() {
                    create_dir_all(parent)?;
                }
                File::create(&target)?;
                println!(
                    "📝 {}檔案: {}",
                    if target.exists() && opt.overwrite {
                        "✏️ 覆蓋"
                    } else {
                        "建立"
                    },
                    target.display()
                );
            }
        }
    }

    println!(
        "{}",
        if opt.dry_run {
            "✨ Dry-run 完成"
        } else {
            "✅ 全部完成"
        }
    );
    Ok(())
}
