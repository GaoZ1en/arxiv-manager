# ArXiv Manager 功能域重构计划

## 🎯 重构目标

将现有的技术导向架构重构为**功能域导向架构**，提高代码的可维护性、可扩展性和业务表达力。

## 🏗️ 新架构设计

### 功能域分层架构 (Domain-Driven Design)

```
src/
├── main.rs                    # 应用入口
├── lib.rs                     # 库入口
├── 
├── domains/                   # 🎯 功能域核心 (新增)
│   ├── mod.rs
│   ├── paper/                 # 📄 论文管理域
│   │   ├── mod.rs
│   │   ├── models/            # 领域模型
│   │   │   ├── paper.rs       # 论文实体
│   │   │   ├── metadata.rs    # 元数据值对象
│   │   │   └── collection.rs  # 论文集合
│   │   ├── services/          # 领域服务
│   │   │   ├── paper_service.rs       # 论文管理服务
│   │   │   ├── metadata_extractor.rs  # 元数据提取
│   │   │   └── collection_manager.rs  # 集合管理
│   │   ├── repositories/      # 存储库接口
│   │   │   └── paper_repository.rs
│   │   └── events/            # 领域事件
│   │       ├── paper_saved.rs
│   │       ├── paper_updated.rs
│   │       └── paper_deleted.rs
│   │
│   ├── search/                # 🔍 搜索域
│   │   ├── mod.rs
│   │   ├── models/
│   │   │   ├── query.rs       # 搜索查询
│   │   │   ├── filters.rs     # 搜索过滤器
│   │   │   ├── results.rs     # 搜索结果
│   │   │   └── history.rs     # 搜索历史
│   │   ├── services/
│   │   │   ├── search_service.rs      # 搜索服务
│   │   │   ├── arxiv_client.rs        # ArXiv API客户端
│   │   │   ├── query_builder.rs       # 查询构建器
│   │   │   └── result_processor.rs    # 结果处理器
│   │   ├── repositories/
│   │   │   ├── search_history_repository.rs
│   │   │   └── search_cache_repository.rs
│   │   └── events/
│   │       ├── search_executed.rs
│   │       ├── search_completed.rs
│   │       └── search_failed.rs
│   │
│   ├── download/              # ⬇️ 下载域
│   │   ├── mod.rs
│   │   ├── models/
│   │   │   ├── download_task.rs       # 下载任务
│   │   │   ├── download_queue.rs      # 下载队列
│   │   │   ├── download_progress.rs   # 下载进度
│   │   │   └── download_session.rs    # 下载会话
│   │   ├── services/
│   │   │   ├── download_service.rs    # 下载服务
│   │   │   ├── queue_manager.rs       # 队列管理器
│   │   │   ├── progress_tracker.rs    # 进度跟踪器
│   │   │   └── file_manager.rs        # 文件管理器
│   │   ├── repositories/
│   │   │   └── download_repository.rs
│   │   └── events/
│   │       ├── download_started.rs
│   │       ├── download_progress.rs
│   │       ├── download_completed.rs
│   │       └── download_failed.rs
│   │
│   ├── library/               # 📚 库管理域
│   │   ├── mod.rs
│   │   ├── models/
│   │   │   ├── library.rs             # 个人库
│   │   │   ├── category.rs            # 分类
│   │   │   ├── tag.rs                 # 标签
│   │   │   └── reading_list.rs        # 阅读清单
│   │   ├── services/
│   │   │   ├── library_service.rs     # 库管理服务
│   │   │   ├── categorization_service.rs  # 分类服务
│   │   │   ├── tagging_service.rs     # 标签服务
│   │   │   └── recommendation_service.rs  # 推荐服务
│   │   ├── repositories/
│   │   │   ├── library_repository.rs
│   │   │   └── category_repository.rs
│   │   └── events/
│   │       ├── paper_added_to_library.rs
│   │       ├── paper_categorized.rs
│   │       └── paper_tagged.rs
│   │
│   └── user/                  # 👤 用户域
│       ├── mod.rs
│       ├── models/
│       │   ├── user_profile.rs        # 用户配置
│       │   ├── preferences.rs         # 用户偏好
│       │   └── session.rs             # 用户会话
│       ├── services/
│       │   ├── settings_service.rs    # 设置服务
│       │   ├── preference_service.rs  # 偏好服务
│       │   └── profile_service.rs     # 配置服务
│       ├── repositories/
│       │   └── user_repository.rs
│       └── events/
│           ├── settings_changed.rs
│           └── preferences_updated.rs
│
├── application/               # 🚀 应用服务层 (重构)
│   ├── mod.rs
│   ├── use_cases/             # 用例/应用服务
│   │   ├── paper/
│   │   │   ├── save_paper.rs
│   │   │   ├── export_papers.rs
│   │   │   └── import_papers.rs
│   │   ├── search/
│   │   │   ├── execute_search.rs
│   │   │   ├── save_search.rs
│   │   │   └── export_search_results.rs
│   │   ├── download/
│   │   │   ├── start_download.rs
│   │   │   ├── manage_download_queue.rs
│   │   │   └── batch_download.rs
│   │   └── library/
│   │       ├── organize_library.rs
│   │       ├── backup_library.rs
│   │       └── sync_library.rs
│   ├── commands/              # 命令处理器
│   │   ├── paper_commands.rs
│   │   ├── search_commands.rs
│   │   ├── download_commands.rs
│   │   └── library_commands.rs
│   ├── queries/               # 查询处理器
│   │   ├── paper_queries.rs
│   │   ├── search_queries.rs
│   │   ├── download_queries.rs
│   │   └── library_queries.rs
│   └── events/                # 应用事件总线
│       ├── event_bus.rs
│       ├── event_handlers.rs
│       └── event_dispatcher.rs
│
├── infrastructure/            # 🔧 基础设施层 (重构)
│   ├── mod.rs
│   ├── persistence/           # 数据持久化
│   │   ├── mod.rs
│   │   ├── database/
│   │   │   ├── connection.rs
│   │   │   ├── migrations.rs
│   │   │   └── schema.rs
│   │   ├── repositories/      # 存储库实现
│   │   │   ├── sqlite_paper_repository.rs
│   │   │   ├── sqlite_search_repository.rs
│   │   │   ├── sqlite_download_repository.rs
│   │   │   └── sqlite_user_repository.rs
│   │   └── cache/
│   │       ├── memory_cache.rs
│   │       └── file_cache.rs
│   ├── external/              # 外部服务集成
│   │   ├── mod.rs
│   │   ├── arxiv/
│   │   │   ├── api_client.rs
│   │   │   ├── xml_parser.rs
│   │   │   └── rate_limiter.rs
│   │   ├── file_system/
│   │   │   ├── file_manager.rs
│   │   │   └── path_resolver.rs
│   │   └── network/
│   │       ├── http_client.rs
│   │       └── download_manager.rs
│   ├── messaging/             # 消息传递
│   │   ├── mod.rs
│   │   ├── event_store.rs
│   │   └── message_bus.rs
│   └── configuration/         # 配置管理
│       ├── mod.rs
│       ├── app_config.rs
│       └── environment.rs
│
├── presentation/              # 🎨 表现层 (重构UI)
│   ├── mod.rs
│   ├── desktop/               # 桌面GUI
│   │   ├── mod.rs
│   │   ├── app.rs             # 主应用状态
│   │   ├── theme/
│   │   │   ├── mod.rs
│   │   │   ├── gruvbox.rs
│   │   │   ├── colors.rs
│   │   │   └── styles.rs
│   │   ├── components/        # 可复用组件
│   │   │   ├── mod.rs
│   │   │   ├── common/
│   │   │   │   ├── button.rs
│   │   │   │   ├── input.rs
│   │   │   │   └── progress_bar.rs
│   │   │   ├── paper/
│   │   │   │   ├── paper_card.rs
│   │   │   │   ├── paper_list.rs
│   │   │   │   └── paper_details.rs
│   │   │   ├── search/
│   │   │   │   ├── search_bar.rs
│   │   │   │   ├── search_filters.rs
│   │   │   │   └── search_results.rs
│   │   │   └── download/
│   │   │       ├── download_item.rs
│   │   │       ├── download_queue.rs
│   │   │       └── download_progress.rs
│   │   ├── views/             # 页面视图
│   │   │   ├── mod.rs
│   │   │   ├── search_view.rs         # 搜索页面
│   │   │   ├── library_view.rs        # 图书馆页面
│   │   │   ├── downloads_view.rs      # 下载页面
│   │   │   ├── paper_view.rs          # 论文详情页面
│   │   │   └── settings_view.rs       # 设置页面
│   │   ├── layouts/           # 布局组件
│   │   │   ├── mod.rs
│   │   │   ├── main_layout.rs         # 主布局
│   │   │   ├── sidebar.rs             # 侧边栏
│   │   │   └── tab_bar.rs             # 标签栏
│   │   └── messages/          # UI消息
│   │       ├── mod.rs
│   │       ├── ui_messages.rs
│   │       └── view_messages.rs
│   └── shared/                # 跨平台共享
│       ├── mod.rs
│       ├── view_models/       # 视图模型
│       │   ├── paper_view_model.rs
│       │   ├── search_view_model.rs
│       │   ├── download_view_model.rs
│       │   └── library_view_model.rs
│       └── mappers/           # 数据映射器
│           ├── paper_mapper.rs
│           ├── search_mapper.rs
│           └── download_mapper.rs
│
├── shared/                    # 📦 共享模块
│   ├── mod.rs
│   ├── common/
│   │   ├── mod.rs
│   │   ├── types.rs           # 通用类型
│   │   ├── constants.rs       # 常量定义
│   │   └── utils.rs           # 工具函数
│   ├── errors/                # 错误处理
│   │   ├── mod.rs
│   │   ├── domain_errors.rs   # 领域错误
│   │   ├── application_errors.rs  # 应用错误
│   │   └── infrastructure_errors.rs  # 基础设施错误
│   └── validation/            # 验证
│       ├── mod.rs
│       ├── validators.rs
│       └── rules.rs
│
└── tests/                     # 🧪 测试
    ├── unit/                  # 单元测试
    │   ├── domains/
    │   ├── application/
    │   └── infrastructure/
    ├── integration/           # 集成测试
    │   ├── paper_management/
    │   ├── search_functionality/
    │   └── download_system/
    └── e2e/                   # 端到端测试
        ├── user_workflows/
        └── system_scenarios/
```

## 🔄 迁移策略

### 阶段1: 核心域提取 (1-2天)
1. **Paper Domain 提取**
   - 从 `core/models/paper.rs` 提取论文实体
   - 创建 PaperService 和 PaperRepository 接口
   - 实现基础的论文管理用例

2. **Search Domain 独立**
   - 从 `search/` 模块提取搜索逻辑
   - 创建 SearchService 和相关查询对象
   - 分离 ArXiv API 客户端到基础设施层

### 阶段2: 应用服务层建立 (1天)
1. **Use Cases 创建**
   - 提取现有 handlers 中的业务逻辑到用例
   - 实现 CQRS 模式（命令和查询分离）
   - 建立事件总线机制

2. **基础设施接口定义**
   - 定义存储库接口
   - 创建外部服务接口
   - 实现依赖注入容器

### 阶段3: UI层重构 (1-2天)
1. **组件化重构**
   - 按功能域组织 UI 组件
   - 实现视图模型模式
   - 创建可复用的设计系统

2. **状态管理优化**
   - 使用单向数据流
   - 实现响应式状态更新
   - 优化渲染性能

### 阶段4: 基础设施实现 (1天)
1. **存储库实现**
   - 实现具体的数据访问层
   - 优化数据库查询
   - 添加缓存机制

2. **外部服务集成**
   - 重构 ArXiv API 集成
   - 实现文件系统操作
   - 添加网络错误处理

## 🎯 预期收益

### 代码质量提升
- ✅ **关注点分离**: 业务逻辑与技术实现完全分离
- ✅ **可测试性**: 每个域都可以独立测试
- ✅ **可维护性**: 功能域边界清晰，修改影响范围小
- ✅ **可扩展性**: 新功能可以作为新域添加

### 开发效率提升
- ✅ **团队协作**: 不同开发者可以并行开发不同域
- ✅ **代码复用**: 共享组件和服务可以跨域使用
- ✅ **错误定位**: 问题可以快速定位到特定域
- ✅ **文档生成**: 域模型即文档

### 系统架构优化
- ✅ **性能优化**: 每个域可以独立优化
- ✅ **缓存策略**: 按域设计缓存策略
- ✅ **错误处理**: 分层错误处理机制
- ✅ **日志管理**: 按域组织日志和监控

## 📊 具体实施计划

### Step 1: Paper Domain 提取
**目标**: 将论文相关功能独立为一个完整的域

**当前文件**:
- `src/core/models/paper.rs` → `src/domains/paper/models/paper.rs`
- `src/core/handlers/paper_handler.rs` → `src/domains/paper/services/paper_service.rs`

**新增文件**:
- `src/domains/paper/repositories/paper_repository.rs`
- `src/domains/paper/events/paper_events.rs`
- `src/application/use_cases/paper/save_paper.rs`

### Step 2: Search Domain 独立
**目标**: 创建独立的搜索域，与 ArXiv API 集成

**当前文件**:
- `src/search/` → `src/domains/search/`
- `src/core/arxiv_api.rs` → `src/infrastructure/external/arxiv/api_client.rs`

**新增文件**:
- `src/domains/search/services/search_service.rs`
- `src/application/use_cases/search/execute_search.rs`
- `src/infrastructure/external/arxiv/rate_limiter.rs`

### Step 3: UI层组件化
**目标**: 按功能域重新组织UI组件

**当前文件**:
- `src/ui/views/search.rs` → `src/presentation/desktop/views/search_view.rs`
- `src/ui/components/paper_card.rs` → `src/presentation/desktop/components/paper/paper_card.rs`

**新增文件**:
- `src/presentation/shared/view_models/search_view_model.rs`
- `src/presentation/desktop/components/search/search_filters.rs`

## 🚀 开始实施

这个重构计划将大大提高代码的可维护性和可扩展性。每个功能域都有明确的职责边界，使得团队可以更高效地协作开发。

准备好开始实施吗？我建议从 **Paper Domain** 开始，因为它是整个应用的核心实体。
