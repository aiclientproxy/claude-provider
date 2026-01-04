# Claude Provider

Claude Provider 是 ProxyCast 的综合性 Anthropic/Claude 插件，支持多种认证方式访问 Claude 模型。

## 支持的认证方式

| 认证方式 | 说明 | 适用场景 |
|---------|------|---------|
| **OAuth** | 标准 OAuth 2.0 + PKCE | Claude.ai 个人账户 |
| **Claude Code** | Claude Code CLI 认证 | 开发者工具 |
| **Console** | Anthropic Console OAuth | 企业/团队账户 |
| **Setup Token** | 只读推理 Token | 最小权限场景 |
| **Bedrock** | AWS Bedrock Claude | AWS 云服务 |
| **CCR** | 第三方中转服务 | 自定义 API 端点 |

## 支持的模型

- `claude-opus-4-20250514` - Claude Opus 4
- `claude-opus-4-5-20251101` - Claude Opus 4.5
- `claude-sonnet-4-20250514` - Claude Sonnet 4
- `claude-sonnet-4-5-20250929` - Claude Sonnet 4.5
- `claude-haiku-3-5-20241022` - Claude Haiku 3.5

## 开发

### 前端开发

```bash
# 安装依赖
npm install

# 开发模式
npm run dev

# 构建
npm run build
```

### 后端开发

```bash
cd src-tauri

# 构建
cargo build --release

# 运行
cargo run -- --help
```

## 项目结构

```
claude-provider/
├── plugin/
│   ├── plugin.json          # 插件元数据
│   ├── config.json          # 默认配置
│   └── dist/                # 前端构建输出
├── src/                     # 前端 React UI
│   ├── index.tsx
│   ├── App.tsx
│   └── components/
├── src-tauri/src/           # 后端 Rust 代码
│   ├── main.rs              # CLI 入口
│   ├── provider.rs          # 核心实现
│   ├── credentials.rs       # 凭证数据结构
│   ├── token_refresh.rs     # Token 刷新
│   └── auth/                # 认证模块
│       ├── oauth.rs
│       ├── bedrock.rs
│       └── ccr.rs
└── package.json
```

## License

MIT
