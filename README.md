# Muse for Windows

[Muse](https://muse.ai) を Windows で使うための軽量クライアントです。

Muse に公式の Windows アプリはない（Mac アプリのみ）ため、[Tauri](https://tauri.app) と Windows 標準の WebView2 ランタイムで作った最小構成のラッパーです。Chromium や Electron を同梱しないので、アプリ本体は数 MB で、メモリ使用量も軽めです。

## ダウンロード

最新版は [Releases](https://github.com/mushroom1729-xperia/muse-windows-app/releases) から：

- `Muse_<バージョン>_x64-setup.exe` — インストーラー
- `Muse-portable-<バージョン>.exe` — ポータブル版（インストール不要）

Microsoft Edge WebView2 ランタイムが必要です（Windows 11 と、Edge の入った Windows 10 には標準で入っています）。

## ビルド

GitHub Actions（`.github/workflows/build.yml`）でビルドし、インストーラーとポータブル版を下書きリリースに公開します。

## クレジット

- 作者： [mushroom1729-xperia](https://github.com/mushroom1729-xperia)
- AI（Muse）との共同開発
