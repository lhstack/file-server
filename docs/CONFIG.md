# 配置和部署指南

## 配置文件

配置文件位置: `config.json`

### 基础配置

```json
{
  "dir": "./public",      // 文件管理的根目录
  "host": "127.0.0.1",    // 服务器监听地址
  "port": "8080"          // 服务器监听端口
}
```

### 配置说明

#### dir - 文件管理目录
- **类型**: 字符串
- **默认值**: `./public`
- **说明**: 指定文件管理系统可以访问的根目录
- **示例**:
  - Windows: `"D:/MyFiles"` 或 `"C:\\Users\\Username\\Documents"`
  - Linux/Mac: `"/home/user/files"` 或 `"/var/www/files"`

#### host - 服务器地址
- **类型**: 字符串
- **默认值**: `127.0.0.1`
- **说明**: 服务器监听的IP地址
- **常见值**:
  - `127.0.0.1` - 仅本地访问
  - `0.0.0.0` - 允许任何IP访问
  - `192.168.1.100` - 指定IP地址

#### port - 服务器端口
- **类型**: 字符串
- **默认值**: `8080`
- **说明**: 服务器监听的端口号
- **常见值**:
  - `8080` - 默认端口
  - `3000` - 开发端口
  - `80` - HTTP标准端口（需要管理员权限）
  - `443` - HTTPS标准端口（需要管理员权限）

## 常见配置场景

### 1. 本地开发

```json
{
  "dir": "./public",
  "host": "127.0.0.1",
  "port": "8080"
}
```

### 2. 局域网访问

```json
{
  "dir": "./public",
  "host": "0.0.0.0",
  "port": "8080"
}
```

然后使用 `http://192.168.1.100:8080` 访问（替换为实际IP）

### 3. 生产环境

```json
{
  "dir": "/var/www/files",
  "host": "127.0.0.1",
  "port": "8080"
}
```

配合Nginx反向代理使用

### 4. 多个目录

创建多个配置文件，使用不同的端口：

**config1.json:**
```json
{
  "dir": "D:/Files1",
  "host": "127.0.0.1",
  "port": "8080"
}
```

**config2.json:**
```json
{
  "dir": "D:/Files2",
  "host": "127.0.0.1",
  "port": "8081"
}
```

然后分别启动两个实例

## 部署指南

### Windows 部署

#### 1. 编译

```powershell
cargo build --release
```

编译后的可执行文件位于: `target/release/demo.exe`

#### 2. 配置

编辑 `config.json`:
```json
{
  "dir": "D:/MyFiles",
  "host": "0.0.0.0",
  "port": "8080"
}
```

#### 3. 运行

**方式1: 直接运行**
```powershell
.\target\release\demo.exe
```

**方式2: 使用启动脚本**
```powershell
.\run.ps1
```

**方式3: 后台运行**
```powershell
Start-Process -NoNewWindow -FilePath ".\target\release\demo.exe"
```

#### 4. 设置开机自启

使用任务计划程序：
1. 打开"任务计划程序"
2. 创建基本任务
3. 设置触发器为"计算机启动时"
4. 设置操作为运行 `demo.exe`

### Linux 部署

#### 1. 编译

```bash
cargo build --release
```

编译后的可执行文件位于: `target/release/demo`

#### 2. 配置

编辑 `config.json`:
```json
{
  "dir": "/home/user/files",
  "host": "0.0.0.0",
  "port": "8080"
}
```

#### 3. 运行

**方式1: 直接运行**
```bash
./target/release/demo
```

**方式2: 使用启动脚本**
```bash
chmod +x run.sh
./run.sh
```

**方式3: 后台运行**
```bash
nohup ./target/release/demo > server.log 2>&1 &
```

**方式4: 使用systemd**

创建 `/etc/systemd/system/file-manager.service`:
```ini
[Unit]
Description=HTTP File Manager
After=network.target

[Service]
Type=simple
User=www-data
WorkingDirectory=/opt/file-manager
ExecStart=/opt/file-manager/demo
Restart=on-failure
RestartSec=10

[Install]
WantedBy=multi-user.target
```

然后运行：
```bash
sudo systemctl enable file-manager
sudo systemctl start file-manager
```

### macOS 部署

#### 1. 编译

```bash
cargo build --release
```

#### 2. 配置

编辑 `config.json`:
```json
{
  "dir": "/Users/username/files",
  "host": "0.0.0.0",
  "port": "8080"
}
```

#### 3. 运行

**方式1: 直接运行**
```bash
./target/release/demo
```

**方式2: 后台运行**
```bash
nohup ./target/release/demo > server.log 2>&1 &
```

**方式3: 使用launchd**

创建 `~/Library/LaunchAgents/com.filemanager.plist`:
```xml
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>Label</key>
    <string>com.filemanager</string>
    <key>ProgramArguments</key>
    <array>
        <string>/Users/username/file-manager/demo</string>
    </array>
    <key>RunAtLoad</key>
    <true/>
    <key>KeepAlive</key>
    <true/>
</dict>
</plist>
```

然后运行：
```bash
launchctl load ~/Library/LaunchAgents/com.filemanager.plist
```

## Docker 部署

### Dockerfile

```dockerfile
FROM rust:latest as builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/demo /usr/local/bin/
COPY --from=builder /app/config.json /app/
WORKDIR /app
EXPOSE 8080
CMD ["demo"]
```

### 构建和运行

```bash
# 构建镜像
docker build -t file-manager .

# 运行容器
docker run -d \
  -p 8080:8080 \
  -v /path/to/files:/app/files \
  -e CONFIG_DIR=/app/files \
  --name file-manager \
  file-manager
```

### Docker Compose

```yaml
version: '3.8'

services:
  file-manager:
    build: .
    ports:
      - "8080:8080"
    volumes:
      - ./files:/app/files
      - ./config.json:/app/config.json
    environment:
      - RUST_LOG=info
    restart: unless-stopped
```

运行：
```bash
docker-compose up -d
```

## Nginx 反向代理

### 基础配置

```nginx
server {
    listen 80;
    server_name example.com;

    location / {
        proxy_pass http://127.0.0.1:8080;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto $scheme;
    }
}
```

### HTTPS 配置

```nginx
server {
    listen 443 ssl http2;
    server_name example.com;

    ssl_certificate /path/to/cert.pem;
    ssl_certificate_key /path/to/key.pem;

    location / {
        proxy_pass http://127.0.0.1:8080;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto $scheme;
    }
}

# HTTP 重定向到 HTTPS
server {
    listen 80;
    server_name example.com;
    return 301 https://$server_name$request_uri;
}
```

### 带认证的配置

```nginx
server {
    listen 80;
    server_name example.com;

    location / {
        auth_basic "Restricted";
        auth_basic_user_file /etc/nginx/.htpasswd;

        proxy_pass http://127.0.0.1:8080;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
    }
}
```

生成密码文件：
```bash
htpasswd -c /etc/nginx/.htpasswd username
```

## 性能优化

### 1. 增加文件描述符限制

```bash
# Linux
ulimit -n 65536

# 永久设置，编辑 /etc/security/limits.conf
* soft nofile 65536
* hard nofile 65536
```

### 2. 调整TCP参数

```bash
# Linux
sysctl -w net.core.somaxconn=65535
sysctl -w net.ipv4.tcp_max_syn_backlog=65535
```

### 3. 使用CDN加速

配置CDN缓存静态资源（HTML/CSS/JS）

### 4. 启用压缩

在Nginx中启用gzip：
```nginx
gzip on;
gzip_types text/plain text/css application/json application/javascript;
gzip_min_length 1000;
```

## 监控和日志

### 查看日志

**Linux/Mac:**
```bash
# 实时查看日志
tail -f server.log

# 查看最后100行
tail -100 server.log
```

**Windows:**
```powershell
# 查看日志
Get-Content server.log -Tail 100 -Wait
```

### 监控进程

**Linux:**
```bash
# 查看进程
ps aux | grep demo

# 查看资源使用
top -p $(pgrep demo)
```

**Windows:**
```powershell
# 查看进程
Get-Process demo

# 查看资源使用
Get-Process demo | Select-Object Name, CPU, Memory
```

## 故障排除

### 端口被占用

**Linux/Mac:**
```bash
# 查看占用端口的进程
lsof -i :8080

# 杀死进程
kill -9 <PID>
```

**Windows:**
```powershell
# 查看占用端口的进程
netstat -ano | findstr :8080

# 杀死进程
taskkill /PID <PID> /F
```

### 权限问题

**Linux:**
```bash
# 给文件夹赋予权限
chmod 755 /path/to/files

# 给用户赋予权限
chown -R www-data:www-data /path/to/files
```

### 内存泄漏

监控内存使用，如果持续增长：
1. 检查日志中的错误
2. 重启服务
3. 考虑增加系统内存

## 备份和恢复

### 备份配置

```bash
# Linux/Mac
cp config.json config.json.backup

# Windows
Copy-Item config.json config.json.backup
```

### 备份文件

```bash
# Linux/Mac
tar -czf files_backup.tar.gz /path/to/files

# Windows
Compress-Archive -Path "D:\files" -DestinationPath "files_backup.zip"
```

## 安全建议

1. **不要在公网直接暴露** - 使用反向代理和认证
2. **使用HTTPS** - 加密传输数据
3. **限制目录范围** - 只允许访问特定目录
4. **定期备份** - 防止数据丢失
5. **监控日志** - 及时发现异常
6. **更新依赖** - 定期更新Rust和依赖库
