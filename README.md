# HTTP 文件管理服务

一个功能完整的HTTP文件管理系统，使用Rust + Axum框架构建，提供Web界面进行文件操作。所有前端代码完全内置在二进制文件中，无需任何外部资源。

## ✨ 核心特性

### 文件管理
- 📁 **文件浏览** - 列表显示目录下的所有文件和文件夹，与操作系统一致的排序方式
- ⬆️ **文件上传** - 支持单个或多个文件上传，最大支持1GB
- ⬇️ **文件下载** - 直接下载任何文件，支持Range请求用于视频/音频seek
- 🗑️ **文件删除** - 删除单个文件或递归删除文件夹
- 📁 **文件夹操作** - 创建、删除、导航文件夹
- 🔄 **批量操作** - 批量删除、复制、移动文件

### 文件预览
- 🖼️ **图片预览** - jpg, png, gif, svg, webp, bmp等
- 📝 **文本预览** - txt, json, js, html, css, md, ini, pem等
- 🎬 **视频播放** - mp4, webm, ogg, mov, avi, mkv等（流式播放，支持seek）
- 🎵 **音频播放** - mp3, wav, flac, aac, m4a等（流式播放，支持seek）

### 用户界面
- 📱 **响应式设计** - 完美支持桌面、平板、手机等各种设备
- 🔍 **实时搜索** - 快速过滤文件
- 🗂️ **面包屑导航** - 快速导航到任意目录
- 🎨 **现代UI** - 清晰的界面设计和流畅的交互

### 安全特性
- 🔒 **路径验证** - 防止目录遍历攻击
- ✅ **权限检查** - 确保操作在允许的目录范围内
- 📋 **错误处理** - 完善的错误提示和日志

## 🚀 快速开始

### 前置要求
- Rust 1.70+ ([安装](https://rustup.rs/))
- 现代浏览器（Chrome, Firefox, Safari, Edge）

### 启动服务

**手动启动:**
```bash
cargo build --release
./target/release/demo
```

然后打开浏览器访问: **http://127.0.0.1:8080**

## 📖 文档

- [快速开始指南](docs/QUICK_START.md) - 30秒快速上手
- [使用指南](docs/USAGE.md) - 详细的操作说明
- [功能特性](docs/FEATURES.md) - 完整的功能列表
- [API文档](docs/API.md) - REST API详细说明
- [配置指南](docs/CONFIG.md) - 配置和部署
- [技术实现](docs/IMPLEMENTATION.md) - 技术细节和架构

## ⚙️ 配置

编辑 `config.json` 文件进行配置：

```json
{
  "dir": "./public",      // 文件管理的根目录
  "host": "127.0.0.1",    // 服务器地址
  "port": "8080"          // 服务器端口
}
```

**常见配置:**
```json
// 修改管理目录
{"dir": "D:/MyFiles"}

// 允许远程访问
{"host": "0.0.0.0"}

// 修改端口
{"port": "3000"}
```

## 📁 项目结构

```
http-server/
├── src/
│   ├── main.rs          # 主程序入口，路由配置
│   ├── handlers.rs      # API处理器实现
│   ├── models.rs        # 数据模型
│   ├── utils.rs         # 工具函数
│   └── errors.rs        # 错误处理
├── public/
│   ├── index.html       # 前端页面（内置）
│   ├── style.css        # 样式表（内置）
│   └── app.js           # 前端应用（内置）
├── docs/                # 文档目录
│   ├── QUICK_START.md   # 快速开始
│   ├── USAGE.md         # 使用指南
│   ├── FEATURES.md      # 功能特性
│   ├── API.md           # API文档
│   ├── CONFIG.md        # 配置指南
│   └── IMPLEMENTATION.md # 技术实现
├── config.json          # 配置文件
├── Cargo.toml           # Rust依赖
└── README.md            # 本文件
```

## 🎯 基本操作

| 操作 | 说明 |
|------|------|
| **打开文件夹** | 点击文件夹名称进入 |
| **上传文件** | 点击"上传文件"按钮选择文件 |
| **创建文件夹** | 点击"新建文件夹"输入名称 |
| **删除文件** | 选中文件后点击"删除"按钮 |
| **复制文件** | 选中文件后点击"复制"按钮 |
| **移动文件** | 选中文件后点击"移动"按钮 |
| **预览文件** | 点击文件名或"预览"按钮 |
| **搜索文件** | 在搜索框输入文件名 |

## 🔌 API 端点

### 文件操作
- `GET /api/files` - 列表文件（根目录）
- `GET /api/files/{path}` - 列表文件（指定路径）
- `GET /api/preview/{path}` - 预览文件
- `GET /api/download/{path}` - 下载文件
- `POST /api/upload` - 上传文件
- `DELETE /api/delete/{path}` - 删除文件
- `DELETE /api/delete-dir/{path}` - 删除文件夹

### 文件夹操作
- `POST /api/mkdir` - 创建文件夹
- `GET /api/info/{path}` - 获取文件信息

### 批量操作
- `POST /api/batch-delete` - 批量删除
- `POST /api/batch-copy` - 批量复制
- `POST /api/batch-move` - 批量移动

详见 [API文档](docs/API.md)

## 🌐 浏览器兼容性

| 浏览器 | 版本 |
|--------|------|
| Chrome | 90+ |
| Firefox | 88+ |
| Safari | 14+ |
| Edge | 90+ |

## 💻 系统要求

- **操作系统**: Windows, Linux, macOS
- **磁盘空间**: 最小50MB（用于编译和运行）
- **内存**: 最小256MB

## 🔒 安全特性

- ✅ 防止目录遍历攻击
- ✅ 路径验证和权限检查
- ✅ 完善的错误处理
- ✅ 支持HTTPS（通过反向代理）

## 📊 性能指标

- 支持最大1GB单文件上传
- 异步I/O处理并发请求
- 流式文件传输
- 支持Range请求用于视频/音频seek

## 🛠️ 开发

### 编译
```bash
cargo build --release
```

## 📦 部署

### Docker
```dockerfile
FROM rust:latest
WORKDIR /app
COPY . .
RUN cargo build --release
EXPOSE 8080
CMD ["./target/release/demo"]
```

### Nginx反向代理
```nginx
server {
    listen 80;
    server_name example.com;
    
    location / {
        proxy_pass http://127.0.0.1:8080;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
    }
}
```

## ❓ 常见问题

**Q: 如何修改管理的目录？**  
A: 编辑 `config.json` 中的 `dir` 字段。

**Q: 如何在远程机器上访问？**  
A: 修改 `config.json` 中的 `host` 为 `0.0.0.0`，然后用 `http://服务器IP:端口` 访问。

**Q: 支持的最大文件大小？**  
A: 理论上无限制，取决于磁盘空间。

**Q: 如何后台运行？**  
A: Windows使用任务计划程序，Linux/Mac使用 `nohup ./run.sh &`。

**Q: 支持哪些文件类型的预览？**  
A: 图片（jpg, png, gif, svg）、文本（txt, json, js, html, css, md）、视频、音频。

更多问题见 [使用指南](docs/USAGE.md)

## 📝 许可证

MIT License

## 🤝 贡献

欢迎提交Issue和Pull Request！

## 📚 更多资源

- [快速开始指南](docs/QUICK_START.md) - 30秒快速上手
- [详细使用指南](docs/USAGE.md) - 完整的操作说明
- [功能特性列表](docs/FEATURES.md) - 所有功能详解
- [REST API文档](docs/API.md) - API端点说明
- [配置和部署](docs/CONFIG.md) - 部署指南
- [技术实现细节](docs/IMPLEMENTATION.md) - 架构和实现

## 📞 获取帮助

1. 查看 [快速开始指南](docs/QUICK_START.md)
2. 查看 [使用指南](docs/USAGE.md)
3. 查看 [常见问题](docs/USAGE.md#常见问题)
4. 检查浏览器控制台错误信息

---

**开始使用:** 运行 `./run.ps1` (Windows) 或 `./run.sh` (Linux/Mac)，然后访问 http://127.0.0.1:8080
