# REST API 文档

## 基础信息

- **基础URL**: `http://127.0.0.1:8080`
- **响应格式**: JSON
- **字符编码**: UTF-8

## 响应格式

所有API返回统一的JSON格式：

```json
{
  "code": 0,              // 0表示成功，其他表示错误
  "message": "success",   // 状态消息
  "data": {}              // 响应数据
}
```

### 错误响应示例

```json
{
  "code": 1,
  "message": "File not found",
  "data": null
}
```

## 文件列表 API

### 获取根目录文件列表

```
GET /api/files
```

**响应示例:**
```json
{
  "code": 0,
  "message": "success",
  "data": {
    "items": [
      {
        "name": "folder1",
        "path": "folder1",
        "is_dir": true,
        "size": 0,
        "modified": "2024-01-01 12:00:00",
        "created": "2024-01-01 12:00:00"
      },
      {
        "name": "file.txt",
        "path": "file.txt",
        "is_dir": false,
        "size": 1024,
        "modified": "2024-01-01 12:00:00",
        "created": "2024-01-01 12:00:00"
      }
    ],
    "total": 2
  }
}
```

### 获取指定路径的文件列表

```
GET /api/files/{path}
```

**参数:**
- `path` (string) - 相对路径，例如 `folder1/subfolder`

**响应:** 同上

## 文件预览 API

### 预览文件内容

```
GET /api/preview/{path}
```

**参数:**
- `path` (string) - 文件相对路径

**响应:**
- 返回文件的原始内容
- Content-Type根据文件类型自动设置

**示例:**
```
GET /api/preview/document.txt
```

## 文件下载 API

### 下载文件

```
GET /api/download/{path}
```

**参数:**
- `path` (string) - 文件相对路径

**请求头:**
- `Range` (可选) - 用于断点续传，格式: `bytes=start-end`

**响应头:**
- `Content-Type` - 文件MIME类型
- `Content-Length` - 文件大小
- `Content-Disposition` - 下载文件名
- `Accept-Ranges` - 支持Range请求

**响应状态码:**
- `200` - 完整文件
- `206` - 部分内容（Range请求）

**示例:**
```
GET /api/download/video.mp4
Range: bytes=0-1023
```

**206响应示例:**
```
HTTP/1.1 206 Partial Content
Content-Type: video/mp4
Content-Length: 1024
Content-Range: bytes 0-1023/1048576
Accept-Ranges: bytes
```

## 文件上传 API

### 上传文件到根目录

```
POST /api/upload
```

**请求:**
- Content-Type: `multipart/form-data`
- 参数: `files` (file) - 文件字段，支持多个文件

**响应:**
```json
{
  "code": 0,
  "message": "success",
  "data": {
    "uploaded": ["file1.txt", "file2.txt"]
  }
}
```

### 上传文件到指定目录

```
POST /api/upload/{path}
```

**参数:**
- `path` (string) - 目标目录相对路径

**请求:** 同上

**响应:** 同上

## 文件删除 API

### 删除单个文件

```
DELETE /api/delete/{path}
```

**参数:**
- `path` (string) - 文件相对路径

**响应:**
```json
{
  "code": 0,
  "message": "success",
  "data": null
}
```

## 文件夹删除 API

### 删除文件夹（递归）

```
DELETE /api/delete-dir/{path}
```

**参数:**
- `path` (string) - 文件夹相对路径

**响应:**
```json
{
  "code": 0,
  "message": "success",
  "data": null
}
```

## 创建文件夹 API

### 创建新文件夹

```
POST /api/mkdir
```

**请求体:**
```json
{
  "path": "parent/path",  // 父目录相对路径，可为空表示根目录
  "name": "new_folder"    // 新文件夹名称
}
```

**响应:**
```json
{
  "code": 0,
  "message": "success",
  "data": null
}
```

## 获取文件信息 API

### 获取单个文件的详细信息

```
GET /api/info/{path}
```

**参数:**
- `path` (string) - 文件相对路径

**响应:**
```json
{
  "code": 0,
  "message": "success",
  "data": {
    "name": "file.txt",
    "path": "file.txt",
    "is_dir": false,
    "size": 1024,
    "modified": "2024-01-01 12:00:00",
    "created": "2024-01-01 12:00:00",
    "mime_type": "text/plain"
  }
}
```

## 批量删除 API

### 批量删除文件和文件夹

```
POST /api/batch-delete
```

**请求体:**
```json
{
  "paths": [
    "file1.txt",
    "folder1",
    "file2.txt"
  ]
}
```

**响应:**
```json
{
  "code": 0,
  "message": "success",
  "data": {
    "deleted": ["file1.txt", "folder1"],
    "failed": [
      {
        "path": "file2.txt",
        "error": "Permission denied"
      }
    ]
  }
}
```

## 批量复制 API

### 批量复制文件和文件夹

```
POST /api/batch-copy
```

**请求体:**
```json
{
  "paths": [
    "file1.txt",
    "folder1"
  ],
  "destination": "target/path"  // 目标目录相对路径
}
```

**响应:**
```json
{
  "code": 0,
  "message": "success",
  "data": {
    "copied": ["file1.txt", "folder1"],
    "failed": []
  }
}
```

## 批量移动 API

### 批量移动文件和文件夹

```
POST /api/batch-move
```

**请求体:**
```json
{
  "paths": [
    "file1.txt",
    "folder1"
  ],
  "destination": "target/path"  // 目标目录相对路径
}
```

**响应:**
```json
{
  "code": 0,
  "message": "success",
  "data": {
    "moved": ["file1.txt", "folder1"],
    "failed": []
  }
}
```

## 前端页面 API

### 获取主页面

```
GET /
GET /index.html
```

**响应:** HTML页面

### 获取样式表

```
GET /style.css
```

**响应:** CSS样式表

### 获取应用脚本

```
GET /app.js
```

**响应:** JavaScript应用代码

## 错误代码

| 代码 | 说明 |
|------|------|
| 0 | 成功 |
| 1 | 通用错误 |
| 2 | 文件不存在 |
| 3 | 权限拒绝 |
| 4 | 无效路径 |
| 5 | 无效请求 |

## 使用示例

### JavaScript/Fetch

**获取文件列表**
```javascript
fetch('/api/files')
  .then(res => res.json())
  .then(data => console.log(data.data.items));
```

**上传文件**
```javascript
const formData = new FormData();
formData.append('files', fileInput.files[0]);

fetch('/api/upload', {
  method: 'POST',
  body: formData
})
  .then(res => res.json())
  .then(data => console.log(data.data.uploaded));
```

**删除文件**
```javascript
fetch('/api/delete/file.txt', {
  method: 'DELETE'
})
  .then(res => res.json())
  .then(data => console.log(data));
```

**创建文件夹**
```javascript
fetch('/api/mkdir', {
  method: 'POST',
  headers: { 'Content-Type': 'application/json' },
  body: JSON.stringify({
    path: '',
    name: 'new_folder'
  })
})
  .then(res => res.json())
  .then(data => console.log(data));
```

### cURL

**获取文件列表**
```bash
curl http://127.0.0.1:8080/api/files
```

**上传文件**
```bash
curl -X POST -F "files=@file.txt" http://127.0.0.1:8080/api/upload
```

**删除文件**
```bash
curl -X DELETE http://127.0.0.1:8080/api/delete/file.txt
```

**创建文件夹**
```bash
curl -X POST -H "Content-Type: application/json" \
  -d '{"path":"","name":"new_folder"}' \
  http://127.0.0.1:8080/api/mkdir
```

## 限制

- 单个文件上传最大1GB
- 路径长度限制（操作系统限制）
- 文件名长度限制（操作系统限制）
- 并发请求无特殊限制

## 安全注意事项

- 所有路径都经过验证，防止目录遍历
- 操作限制在配置的基础目录内
- 建议在生产环境使用HTTPS
- 建议配置反向代理进行认证
