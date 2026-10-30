# STATE — VinylMini（Tauri 黑胶迷你播放器）构建与验收（轻档）

- 目标：接手 Linux 沙箱产出的未构建源码，在本机（Windows 11，无管理员权限）完成真实构建 + 按 SPEC 行为逐条测试，交付可双击运行的 exe
- 状态：**已完成**。cargo check 无错误；`npx tauri build` 成功产出 vinylmini.exe（5.3MB）与 NSIS 安装包 VinylMini_1.0.0_x64-setup.exe（1.65MB）；27 项 CDP 行为断言全部通过；首启弹文件夹选择框、安装版双击运行均验证通过（用户现场实际选库播放）。应用已静默安装至 %LOCALAPPDATA%\VinylMini 并保留（用户正在使用），卸载命令 `%LOCALAPPDATA%\VinylMini\uninstall.exe /S`
- 二轮迭代（2026-09-29 晚，用户要求）：改无边框透明窗口——白色圆角卡片即整个窗口（decorations:false + transparent:true + shadow:false），右上角新增最小化/关闭小钮，点击黑胶盘弹出文件夹面板（原左上角文件夹按钮移除），顶栏/碟片区可拖动窗口（data-tauri-drag-region）。6 项定点断言通过（透明背景/点盘开关面板/最小化/还原），用户现场实操验证面板添加文件夹与 ✕ 关闭。安装版已原地升级并恢复用户数据（豆瓣+网易云两歌单、含一条喜欢标记；备份在 %APPDATA%\local.vinyl.mini\state.json.userbak）
- 工具链（本机新增，均免管理员）：Rust stable x86_64-pc-windows-gnu（~/.cargo、~/.rustup）+ winlibs MinGW-w64 16.2.0 msvcrt（~/tools/mingw64，SHA256 已校验）。构建 PATH 需含 `~/.cargo/bin` 与 `~/tools/mingw64/bin`
- 测试资产：`../test/run-cdp-tests.mjs`（CDP 直连 WebView2 的行为断言套件，用法见文件头；T10"逐文件播放"设计仅适合测试音频库，勿对用户真实大库运行）、`../test/verify-frameless.mjs`（无边框 UI 定点验证）、`../test/enum-windows.ps1`（枚举原生对话框）、`../test-music/A|B`（ffmpeg 生成 60s 五格式音频）
- 三轮迭代（2026-09-29 晚，用户反馈"讨厌不切歌"）：改为**每次点讨厌都立即切歌**（去掉原"第二次点=取消讨厌"的 toggle 语义），讨厌计数照加，同一首累计 3 次拉黑；拉黑后权重 0。9 项定点断言通过（`test/verify-dislike.mjs`），回归套件 T8 已按新语义重写。安装版已升级、用户状态已恢复
- 已知瑕疵（不影响使用）：GNU 链接器警告 `.rsrc merge failure: multiple non-default manifests`（图标/DPI 实测正常）
- 下一步（可选）：项目尚未 git init；如走 GitHub Actions 构建路径，先 git init + 推送（.gitignore 需含 src-tauri/target、node_modules）
- 代码作者（用户告知，2026-10-30）：**Claude Sonnet 5.5**（原 Linux 沙箱会话产出）。后续手改保持其风格：极限精简、零注释、en-dash 排版
- 最近更新：2026-10-30（收尾：补记代码作者归属；三轮迭代后无新改动，AI-Common 无待提交沉淀）
