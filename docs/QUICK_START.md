# 快速开始指南

## 30秒快速启动

### Windows
```powershell
.\run.ps1
```

### Linux/Mac
```bash
chmod +x run.sh
./run.sh
```

然后打开浏览器访问: **http://127.0.0.1:8080**

---

## 基本操作

| 操作 | 步骤 |
|------|------|
| **打开文件夹** | 点击文件夹名称 |
| **上传文件** | 点击"上传文件" → 选择文件 |
| **创建文件夹** | 点击"新建文件夹" → 输入名称 |
| **删除文件** | 选中文件 → 点击"删除" |
| **复制文件** | 选中文件 → 点击"复制" |
| **移动文件** | 选中文件 → 点击"移动" → 选择目标 |
| **预览文件** | 点击文件名或"预览"按钮 |
| **搜索文件** | 在搜索框输入文件名 |

---

## 配置修改

编辑 `config.json`:

```json
{
  "dir": "./public",      // 管理的目录
  "host": "127.0.0.1",    // 服务器地址
  "port": "8080"          // 服务器端口
}
```

**常见配置:**
- 修改目录: `"dir": "D:/MyFiles"`
- 允许远程访问: `"host": "0.0.0.0"`
- 修改端口: `"port": "3000"`

---

## 支持的文件预览

✅ **图片**: jpg, png, gif, svg, webp, bmp  
✅ **文本**: txt, json, js, html, css, md, ini, pem  
✅ **视频**: mp4, webm, ogg, mov, avi, mkv  
✅ **音频**: mp3, wav, flac, aac, m4a  

---

## 常见问题

**Q: 如何停止服务?**  
A: 按 `Ctrl+C`

**Q: 如何在后台运行?**  
A: Windows使用任务计划程序，Linux/Mac使用 `nohup ./run.sh &`

**Q: 如何远程访问?**  
A: 修改config.json中的host为 `0.0.0.0`，然后用 `http://服务器IP:端口` 访问

**Q: 支持的最大文件大小?**  
A: 取决于磁盘空间，理论上无限制

**Q: 如何删除文件夹?**  
A: 点击文件夹右侧的"删除"按钮，会递归删除所有内容

---

## 文件图标

| 图标 | 类型 |
|------|------|
| 📁 | 文件夹 |
| 📄 | 文本/PDF |
| 📝 | Word文档 |
| 📊 | Excel表格 |
| 🎬 | PowerPoint |
| 🗜️ | 压缩文件 |
| 🖼️ | 图片 |
| 🎥 | 视频 |
| 🎵 | 音频 |

---

## 快捷键

| 快捷键 | 功能 |
|--------|------|
| Ctrl+A | 全选 |
| Delete | 删除 |
| F5 | 刷新 |

---

## 系统要求

- **操作系统**: Windows, Linux, macOS
- **浏览器**: Chrome 90+, Firefox 88+, Safari 14+, Edge 90+
- **Rust** (仅开发): 1.70+

---

## 文件结构

```
http-server/
├── src/              # 后端代码
├── public/           # 前端文件（内置）
├── docs/             # 文档
├── config.json       # 配置文件
├── README.md         # 完整文档
└── run.ps1/run.sh    # 启动脚本
```

---

## 获取帮助

1. 查看 [README.md](../README.md) - 完整文档
2. 查看 [USAGE.md](USAGE.md) - 详细使用指南
3. 查看 [IMPLEMENTATION.md](IMPLEMENTATION.md) - 技术实现细节
4. 检查浏览器控制台错误信息

---

## 下一步

- 📖 阅读 [README.md](../README.md) 了解更多功能
- 🔧 查看 [USAGE.md](USAGE.md) 学习详细操作
- 💻 查看 [IMPLEMENTATION.md](IMPLEMENTATION.md) 了解技术细节
