# 拜 Claude 🪔

> 三炷香催 Claude Code 加速。拜拜即可觸發 Ctrl+C 中斷 + 催促語。

基於 [badclaude](https://github.com/GitFrog1111/badclaude) 的精神，把鞭子換成三炷香，Electron 換成 Tauri (Rust)。

## 功能

- 🪔 系統托盤常駐，點擊圖示上香
- 🪔 三炷香跟隨滑鼠，前後透視晃動 + 左右微擺
- 🪔 上下移動滑鼠三次觸發「拜拜」
- 🪔 拜拜時：木魚音效 + 浮動文字（南無加速菩薩）+ 功德 +1
- 🪔 自動向前一個視窗送出 Ctrl+C → 催促語 → Enter
- 🪔 點擊放下香，香掉落後自動隱藏 overlay

## 前置需求

- [Rust](https://rustup.rs/) (1.70+)
- [Tauri CLI v2](https://v2.tauri.app/start/prerequisites/)

```bash
cargo install tauri-cli --version "^2"
```

macOS 額外需要：在「系統設定 → 隱私權與安全性 → 輔助使用」中允許本 app（用於鍵盤模擬）。

## 開發

```bash
cd baiclaude
cargo tauri dev
```

## 打包

```bash
cargo tauri build
```

產出：
- macOS: `src-tauri/target/release/bundle/dmg/`
- Windows: `src-tauri/target/release/bundle/msi/`

## 專案結構

```
baiclaude/
├── ui/                      # 前端（Tauri webview 載入）
│   └── index.html           # 三炷香 canvas + 煙霧 + 拜拜偵測
├── src-tauri/
│   ├── Cargo.toml           # Rust 依賴
│   ├── tauri.conf.json      # Tauri 設定（透明視窗、tray）
│   ├── capabilities/        # Tauri v2 權限
│   └── src/
│       ├── main.rs          # Tray 圖示、overlay 控制、IPC
│       └── macro_sender.rs  # 跨平台鍵盤注入
├── sounds/                  # （可選）音效檔
└── README.md
```

## 自訂

修改 `ui/index.html` 頂部的 `C` 物件可調整：

| 參數 | 說明 |
|------|------|
| `stickLength` | 香的長度 |
| `tiltFactor` / `tiltMax` | 前後晃動靈敏度與幅度 |
| `swayFactor` / `swayMax` | 左右晃動靈敏度與幅度 |
| `bowAmplitude` / `bowsNeeded` | 觸發拜拜所需的移動幅度與次數 |
| `triggerMessages` | 浮動文字內容 |
| `smokePerFrame` | 煙霧濃度 |

## 致敬

- [badclaude](https://github.com/GitFrog1111/badclaude) — 原版鞭子概念
- 南無加速菩薩 🙏
