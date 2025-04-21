![Logo](/docs/logo.png)

# DicStructureCreater

![Crates.io](https://img.shields.io/crates/v/dic-structure-creater)
![License](https://img.shields.io/badge/license-MIT-blue)
![Stars](https://img.shields.io/github/stars/natsuki221/DicStructureCreater)
![Downloads](https://img.shields.io/crates/dr/dic-structure-creater)

一款輕量、跨平台的 CLI 工具，可以自動解析 `.txt`/`.md` 中的樹狀結構，並在指定根目錄下生成對應的資料夾與空檔案。預設不會覆蓋既有檔案，並支援預覽 (`--dry-run`) 及強制覆蓋 (`--overwrite`)。

---

## 功能特色

- 📁 **自動生成**：從各種格式的樹狀描述 (`tree` 或 Markdown list) 產生資料夾結構及檔案。
- 🔒 **安全預設**：預設模式下不會覆蓋已存在檔案，避免意外資料遺失。
- 🔄 **可選覆蓋**：加上 `--overwrite` (`-O`) 強制重寫檔案。
- 👀 **預覽模式**：`--dry-run` (`-d`) 僅列出將執行的動作，但不真正修改檔案系統。
- ⚙️ **跨平台**：使用 Rust 編寫，支援 Windows、macOS、Linux。

---

## 安裝

### 透過 Cargo
```bash
cargo install dic-structure-creater
```

### 下載預編譯二進位
1. 前往 Releases 頁面：
   https://github.com/natsuki221/DicStructureCreater/releases
2. 下載對應作業系統的壓縮包，解壓後將 `dsc`（或 `dsc.exe`）放到環境變數 `PATH` 中。

---

## 使用範例

假設有一份結構檔 `structure.txt`：

```txt
src/
├── lib/
│   └── utils.rs
└── main.rs
```

1. **預覽動作**（不會修改檔案）：
   ```bash
   dsc structure.txt --dry-run
   ```

2. **建立結構**（預設不覆蓋）：
   ```bash
   dsc structure.txt
   ```

3. **覆蓋既有檔案**：
   ```bash
   dsc structure.txt --overwrite
   ```

---

## CLI 選項

| 參數                       | 說明                                                |
|---------------------------|---------------------------------------------------|
| `STRUCTURE_FILE`          | 要解析的文字檔路徑 (必填)                           |
| `-r, --root <path>`       | 指定根目錄 (預設為當前工作目錄)                    |
| `-O, --overwrite`         | 若目標檔案已存在，強制覆蓋                         |
| `-d, --dry-run`           | 僅預覽將要執行的動作，不實際建立檔案與資料夾       |
| `-h, --help`              | 顯示幫助訊息                                       |

---

## 範例結構描述格式

DSC 支援以下兩種常見寫法：

1. **Tree ASCII**：
   ```txt
   src/
   ├── models/
   │   └── user.rs
   └── main.rs
   ```

2. **Markdown List**：
   ```md
   - src
     - models
       - user.rs
     - main.rs
   ```

> **注意**：以 `/` 結尾代表資料夾，否則視為檔案。

---

## 貢獻指南

歡迎任何形式的貢獻：Issues、Pull Requests、功能建議或 Bug 通報。請遵循以下步驟：

1. Fork 本專案
2. 建立分支 (`git checkout -b feature/your-feature`)
3. 實作您的功能或修正
4. 提交 Pull Request

---

## 授權條款

本專案採用 MIT License，詳細請見 [LICENSE](LICENSE)。

---

> 如果有任何問題或建議，歡迎隨時提出！祝開發愉快～✨

