# Vinyl Mini (Tauri)

已在 Windows 11 上构建并测试通过（2026-09-29）：cargo check 无错误，行为断言全部通过（CDP 直连 WebView2），含 mp3/flac/wav/ogg/m4a 五种格式播放、多文件夹管理、权重（喜欢4/普通1/讨厌0.25/拉黑0）、每次讨厌立即切歌且同一首累计 3 次拉黑、历史回退、state.json 持久化、首次启动自动弹文件夹选择框、启动自动播放、无边框透明圆角窗口（点黑胶盘开文件夹面板，右上角最小化/关闭）。

## 产物
- `src-tauri/target/release/vinylmini.exe`（免安装单文件，双击即用）
- `src-tauri/target/release/bundle/nsis/VinylMini_1.0.0_x64-setup.exe`（NSIS 安装包，装到 %LOCALAPPDATA%\VinylMini，含开始菜单快捷方式与卸载器）

## 重新构建（本机已验证）
本机工具链：Rust stable（x86_64-pc-windows-gnu）+ winlibs MinGW-w64（msvcrt 版，%USERPROFILE%\tools\mingw64）。
```
export PATH="$USERPROFILE/.cargo/bin:$USERPROFILE/tools/mingw64/bin:$PATH"
npm install && npx tauri icon app-icon.png && npx tauri build
```
无管理员权限时用上面的 GNU 工具链；若有管理员权限可改用 MSVC 版 Rust + VS Build Tools（与 .github/workflows/build.yml 的 CI 一致）。

## GitHub 构建（备选）
推到 GitHub → Actions → build-windows → 下载 artifact（内含 exe 与安装包）。

数据：`%APPDATA%\local.vinyl.mini\state.json`（文件夹列表 + 喜欢/讨厌/拉黑标记）

