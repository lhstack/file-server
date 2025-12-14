class FileManager {
    constructor() {
        this.currentPath = '';
        this.selectedFiles = new Set();
        this.isMobile = this.detectMobile();
        this.init();
    }

    detectMobile() {
        return /Android|webOS|iPhone|iPad|iPod|BlackBerry|IEMobile|Opera Mini/i.test(navigator.userAgent) || window.innerWidth <= 768;
    }

    init() {
        this.setupEventListeners();
        this.loadFiles();
    }

    setupEventListeners() {
        document.getElementById('refreshBtn').addEventListener('click', () => this.loadFiles());
        document.getElementById('newFolderBtn').addEventListener('click', () => this.showNewFolderModal());
        document.getElementById('uploadBtn').addEventListener('click', () => document.getElementById('fileInput').click());
        document.getElementById('fileInput').addEventListener('change', (e) => this.handleFileUpload(e));
        document.getElementById('selectAllCheckbox').addEventListener('change', (e) => this.toggleSelectAll(e));
        document.getElementById('deleteBtn').addEventListener('click', () => this.deleteSelected());
        document.getElementById('copyBtn').addEventListener('click', () => this.showCopyModal());
        document.getElementById('moveBtn').addEventListener('click', () => this.showMoveModal());
        document.getElementById('searchInput').addEventListener('input', (e) => this.filterFiles(e.target.value));

        // Modal handlers
        document.querySelectorAll('.modal-close, .modal-cancel').forEach(btn => {
            btn.addEventListener('click', (e) => {
                e.target.closest('.modal').classList.remove('active');
            });
        });

        document.getElementById('confirmNewFolder').addEventListener('click', () => this.createFolder());
        document.getElementById('folderNameInput').addEventListener('keypress', (e) => {
            if (e.key === 'Enter') this.createFolder();
        });

        document.getElementById('confirmTarget').addEventListener('click', () => this.confirmMove());

        // Close modal on background click
        document.querySelectorAll('.modal').forEach(modal => {
            modal.addEventListener('click', (e) => {
                if (e.target === modal) modal.classList.remove('active');
            });
        });
    }

    async loadFiles() {
        try {
            const url = this.currentPath ? `/api/files/${this.currentPath}` : '/api/files';
            const response = await fetch(url);
            
            if (!response.ok) throw new Error('Failed to load files');
            
            const data = await response.json();
            if (data.code !== 0) throw new Error(data.message || 'Failed to load files');
            
            this.renderFileList(data.data.items);
            this.updateBreadcrumb();
            this.selectedFiles.clear();
            this.updateToolbar();
        } catch (error) {
            this.showNotification('加载文件失败: ' + error.message, 'error');
        }
    }

    renderFileList(files) {
        const tbody = document.getElementById('fileListBody');
        tbody.innerHTML = '';

        if (files.length === 0) {
            tbody.innerHTML = '<tr class="loading"><td colspan="5">文件夹为空</td></tr>';
            return;
        }

        files.forEach(file => {
            const row = document.createElement('tr');
            row.dataset.path = file.path;
            row.dataset.isDir = file.is_dir;

            const icon = file.is_dir ? '📁' : this.getFileIcon(file.name);
            const size = file.is_dir ? '-' : this.formatSize(file.size);

            row.innerHTML = `
                <td><input type="checkbox" class="file-item-checkbox" data-path="${file.path}"></td>
                <td><div class="file-item-name"><span class="file-item-icon">${icon}</span><span>${this.escapeHtml(file.name)}</span></div></td>
                <td><span class="file-item-size">${size}</span></td>
                <td><span class="file-item-time">${file.modified}</span></td>
                <td>
                    <div class="file-actions">
                        ${file.is_dir ? `<button class="file-action-btn" onclick="fileManager.enterFolder('${file.path}')">进入</button>` : `<button class="file-action-btn" onclick="fileManager.previewFile('${file.path}', '${file.name}')">预览</button>`}
                        <button class="file-action-btn" onclick="fileManager.downloadFile('${file.path}', '${file.name}')">下载</button>
                    </div>
                </td>
            `;

            const checkbox = row.querySelector('.file-item-checkbox');
            checkbox.addEventListener('change', () => this.toggleFileSelection(file.path));

            if (!this.isMobile) {
                row.querySelector('.file-item-name').addEventListener('click', () => {
                    if (file.is_dir) this.enterFolder(file.path);
                    else this.previewFile(file.path, file.name);
                });
            }

            tbody.appendChild(row);
        });
    }

    toggleFileSelection(path) {
        if (this.selectedFiles.has(path)) {
            this.selectedFiles.delete(path);
        } else {
            this.selectedFiles.add(path);
        }
        this.updateToolbar();
    }

    toggleSelectAll(e) {
        const checkboxes = document.querySelectorAll('.file-item-checkbox');
        checkboxes.forEach(cb => {
            cb.checked = e.target.checked;
            const path = cb.dataset.path;
            if (e.target.checked) {
                this.selectedFiles.add(path);
            } else {
                this.selectedFiles.delete(path);
            }
        });
        this.updateToolbar();
    }

    updateToolbar() {
        const count = this.selectedFiles.size;
        document.getElementById('selectedCount').textContent = count > 0 ? `已选择 ${count} 项` : '';
        document.getElementById('deleteBtn').disabled = count === 0;
        document.getElementById('copyBtn').disabled = count === 0;
        document.getElementById('moveBtn').disabled = count === 0;
        document.getElementById('selectAllCheckbox').checked = count > 0 && count === document.querySelectorAll('.file-item-checkbox').length;
    }

    updateBreadcrumb() {
        const breadcrumb = document.getElementById('breadcrumb');
        breadcrumb.innerHTML = '';

        const parts = this.currentPath ? this.currentPath.split('/').filter(p => p) : [];
        
        const homeBtn = document.createElement('div');
        homeBtn.className = 'breadcrumb-item';
        homeBtn.innerHTML = '<a href="javascript:void(0)">主目录</a>';
        homeBtn.querySelector('a').addEventListener('click', () => this.goToPath(''));
        breadcrumb.appendChild(homeBtn);

        let currentPath = '';
        parts.forEach((part, index) => {
            currentPath += (currentPath ? '/' : '') + part;
            const item = document.createElement('div');
            item.className = 'breadcrumb-item';
            const isLast = index === parts.length - 1;
            if (isLast) {
                item.textContent = part;
            } else {
                item.innerHTML = `<span class="breadcrumb-separator">/</span><a href="javascript:void(0)">${this.escapeHtml(part)}</a>`;
                item.querySelector('a').addEventListener('click', () => this.goToPath(currentPath));
            }
            breadcrumb.appendChild(item);
        });
    }

    enterFolder(path) {
        this.goToPath(path);
    }

    goToPath(path) {
        this.currentPath = path;
        this.loadFiles();
    }

    showNewFolderModal() {
        document.getElementById('folderNameInput').value = '';
        document.getElementById('newFolderModal').classList.add('active');
        document.getElementById('folderNameInput').focus();
    }

    async createFolder() {
        const name = document.getElementById('folderNameInput').value.trim();
        if (!name) {
            this.showNotification('请输入文件夹名称', 'warning');
            return;
        }

        try {
            const response = await fetch('/api/mkdir', {
                method: 'POST',
                headers: { 'Content-Type': 'application/json' },
                body: JSON.stringify({ path: this.currentPath, name })
            });

            const data = await response.json();
            if (data.code !== 0) throw new Error(data.message);

            this.showNotification('文件夹创建成功', 'success');
            document.getElementById('newFolderModal').classList.remove('active');
            this.loadFiles();
        } catch (error) {
            this.showNotification('创建文件夹失败: ' + error.message, 'error');
        }
    }

    async handleFileUpload(e) {
        const files = Array.from(e.target.files);
        if (files.length === 0) return;

        this.showLoadingOverlay();

        try {
            const formData = new FormData();
            files.forEach(file => formData.append('files', file));

            const url = this.currentPath ? `/api/upload/${this.currentPath}` : '/api/upload';
            const response = await fetch(url, {
                method: 'POST',
                body: formData
            });

            const data = await response.json();
            if (data.code !== 0) throw new Error(data.message);

            this.showNotification(`成功上传 ${files.length} 个文件`, 'success');
            this.loadFiles();
        } catch (error) {
            this.showNotification('上传失败: ' + error.message, 'error');
        } finally {
            this.hideLoadingOverlay();
            e.target.value = '';
        }
    }

    async previewFile(path, name) {
        try {
            const ext = name.split('.').pop().toLowerCase();
            const modal = document.getElementById('previewModal');
            const previewBody = document.getElementById('previewBody');
            document.getElementById('previewTitle').textContent = name;

            // Image preview
            if (['jpg', 'jpeg', 'png', 'gif', 'webp', 'bmp'].includes(ext)) {
                previewBody.innerHTML = `<img src="/api/preview/${path}" style="max-width:100%;max-height:600px;">`;
            }
            // Video preview
            else if (['mp4', 'webm', 'ogg', 'mov', 'avi', 'mkv'].includes(ext)) {
                previewBody.innerHTML = `<video controls style="max-width:100%;max-height:600px;"><source src="/api/download/${path}" type="video/${ext}"></video>`;
            }
            // Audio preview
            else if (['mp3', 'wav', 'ogg', 'flac', 'm4a', 'aac'].includes(ext)) {
                previewBody.innerHTML = `<audio controls style="width:100%;"><source src="/api/download/${path}" type="audio/${ext}"></audio>`;
            }
            // Text preview
            else if (['txt', 'md', 'json', 'xml', 'html', 'css', 'js', 'py', 'java', 'cpp', 'c', 'h', 'rs', 'go', 'rb', 'php', 'sh', 'bat', 'ps1', 'ini', 'pem', 'crt', 'key', 'conf', 'config', 'log'].includes(ext)) {
                const response = await fetch(`/api/preview/${path}`);
                const text = await response.text();
                previewBody.innerHTML = `<pre style="max-height:600px;overflow:auto;background:#f5f5f5;padding:15px;border-radius:4px;font-size:12px;"><code>${this.escapeHtml(text)}</code></pre>`;
            }
            // PDF preview
            else if (ext === 'pdf') {
                previewBody.innerHTML = `<iframe src="/api/preview/${path}" style="width:100%;height:600px;border:none;"></iframe>`;
            }
            // Default
            else {
                previewBody.innerHTML = `<p>不支持预览此文件类型</p>`;
            }

            modal.classList.add('active');
        } catch (error) {
            this.showNotification('预览失败: ' + error.message, 'error');
        }
    }

    downloadFile(path, name) {
        const link = document.createElement('a');
        link.href = `/api/download/${path}`;
        link.download = name;
        link.click();
    }

    async deleteSelected() {
        if (this.selectedFiles.size === 0) return;

        if (!confirm(`确定要删除选中的 ${this.selectedFiles.size} 项吗？`)) return;

        this.showLoadingOverlay();

        try {
            const response = await fetch('/api/batch-delete', {
                method: 'POST',
                headers: { 'Content-Type': 'application/json' },
                body: JSON.stringify({ paths: Array.from(this.selectedFiles) })
            });

            const data = await response.json();
            if (data.code !== 0) throw new Error(data.message);

            this.showNotification(`成功删除 ${data.data.deleted.length} 项`, 'success');
            this.loadFiles();
        } catch (error) {
            this.showNotification('删除失败: ' + error.message, 'error');
        } finally {
            this.hideLoadingOverlay();
        }
    }

    showCopyModal() {
        this.showTargetModal('复制到', 'copy');
    }

    showMoveModal() {
        this.showTargetModal('移动到', 'move');
    }

    async showTargetModal(title, action) {
        document.getElementById('selectTargetTitle').textContent = title;
        document.getElementById('confirmTarget').dataset.action = action;

        try {
            const response = await fetch('/api/files');
            const data = await response.json();
            if (data.code !== 0) throw new Error(data.message);

            const folders = this.getAllFolders(data.data.items, '');
            const folderList = document.getElementById('targetFolderList');
            folderList.innerHTML = '';

            folders.forEach(folder => {
                const item = document.createElement('div');
                item.className = 'folder-item';
                item.textContent = folder.path || '主目录';
                item.dataset.path = folder.path;
                item.addEventListener('click', () => {
                    document.querySelectorAll('.folder-item').forEach(f => f.classList.remove('selected'));
                    item.classList.add('selected');
                });
                folderList.appendChild(item);
            });

            document.getElementById('selectTargetModal').classList.add('active');
        } catch (error) {
            this.showNotification('加载文件夹失败: ' + error.message, 'error');
        }
    }

    getAllFolders(items, basePath) {
        let folders = [{ path: basePath }];
        items.forEach(item => {
            if (item.is_dir) {
                const path = basePath ? `${basePath}/${item.name}` : item.name;
                folders.push({ path });
            }
        });
        return folders;
    }

    async confirmMove() {
        const selected = document.querySelector('.folder-item.selected');
        if (!selected) {
            this.showNotification('请选择目标文件夹', 'warning');
            return;
        }

        const destination = selected.dataset.path;
        const action = document.getElementById('confirmTarget').dataset.action;

        this.showLoadingOverlay();

        try {
            const endpoint = action === 'copy' ? '/api/batch-copy' : '/api/batch-move';
            const response = await fetch(endpoint, {
                method: 'POST',
                headers: { 'Content-Type': 'application/json' },
                body: JSON.stringify({
                    paths: Array.from(this.selectedFiles),
                    destination
                })
            });

            const data = await response.json();
            if (data.code !== 0) throw new Error(data.message);

            const successKey = action === 'copy' ? 'copied' : 'moved';
            this.showNotification(`成功${action === 'copy' ? '复制' : '移动'} ${data.data[successKey].length} 项`, 'success');
            document.getElementById('selectTargetModal').classList.remove('active');
            this.loadFiles();
        } catch (error) {
            this.showNotification(`${action === 'copy' ? '复制' : '移动'}失败: ` + error.message, 'error');
        } finally {
            this.hideLoadingOverlay();
        }
    }

    filterFiles(query) {
        const rows = document.querySelectorAll('#fileListBody tr');
        rows.forEach(row => {
            if (row.classList.contains('loading')) return;
            const name = row.querySelector('.file-item-name span:last-child').textContent.toLowerCase();
            row.style.display = name.includes(query.toLowerCase()) ? '' : 'none';
        });
    }

    showLoadingOverlay() {
        document.getElementById('loadingOverlay').classList.add('active');
    }

    hideLoadingOverlay() {
        document.getElementById('loadingOverlay').classList.remove('active');
    }

    showNotification(message, type = 'info') {
        const notification = document.getElementById('notification');
        notification.textContent = message;
        notification.className = `notification active ${type}`;
        setTimeout(() => notification.classList.remove('active'), 3000);
    }

    getFileIcon(name) {
        const ext = name.split('.').pop().toLowerCase();
        const icons = {
            'pdf': '📄', 'doc': '📝', 'docx': '📝', 'xls': '📊', 'xlsx': '📊',
            'ppt': '🎯', 'pptx': '🎯', 'txt': '📋', 'md': '📝',
            'jpg': '🖼️', 'jpeg': '🖼️', 'png': '🖼️', 'gif': '🖼️', 'webp': '🖼️',
            'mp4': '🎬', 'webm': '🎬', 'avi': '🎬', 'mov': '🎬', 'mkv': '🎬',
            'mp3': '🎵', 'wav': '🎵', 'flac': '🎵', 'm4a': '🎵',
            'zip': '📦', 'rar': '📦', '7z': '📦', 'tar': '📦', 'gz': '📦',
            'exe': '⚙️', 'msi': '⚙️', 'app': '⚙️', 'dmg': '⚙️',
            'json': '⚙️', 'xml': '⚙️', 'yaml': '⚙️', 'yml': '⚙️',
            'js': '🔧', 'ts': '🔧', 'py': '🐍', 'java': '☕', 'cpp': '⚙️', 'c': '⚙️',
            'html': '🌐', 'css': '🎨', 'php': '🐘', 'rb': '💎', 'go': '🐹',
            'rs': '🦀', 'sh': '💻', 'bat': '💻', 'ps1': '💻'
        };
        return icons[ext] || '📄';
    }

    formatSize(bytes) {
        if (bytes === 0) return '0 B';
        const k = 1024;
        const sizes = ['B', 'KB', 'MB', 'GB'];
        const i = Math.floor(Math.log(bytes) / Math.log(k));
        return Math.round(bytes / Math.pow(k, i) * 100) / 100 + ' ' + sizes[i];
    }

    escapeHtml(text) {
        const div = document.createElement('div');
        div.textContent = text;
        return div.innerHTML;
    }
}

const fileManager = new FileManager();
