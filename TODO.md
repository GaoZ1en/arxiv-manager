# ArXiv Manager 代码重构 TODO

## 已完成的重构工作 ✅

### 1. 应用状态模块重构 (已完成)
- ✅ 将 `core/app_state.rs` (763行) 拆分为处理器架构
- ✅ 创建 `core/handlers/` 目录结构
- ✅ 实现模块化的消息处理器

### 2. 数据模型重构 (已完成)
- ✅ 将 `core/models.rs` (635行) 拆分为专门模块
- ✅ 创建 `core/models/` 目录：
  - `paper.rs` - 论文和下载相关模型
  - `search.rs` - 搜索配置模型
  - `ui.rs` - UI和主题模型
  - `settings.rs` - 设置和快捷键模型

### 3. 搜索服务重构 (已完成)
- ✅ 将 `search/services.rs` (310行) 拆分为模块化架构
- ✅ 创建 `search/api/` - API客户端功能
- ✅ 创建 `search/parsers/` - XML解析功能
- ✅ 创建 `search/filters/` - 结果过滤功能
- ✅ 修复所有编译错误

---

## 待完成的重构工作 🚧

### 4. 数据库模块重构 (高优先级)
**目标文件**: `database/mod.rs` (318行)

**计划结构**:
```
src/database/
├── mod.rs (重构后 - 主要导出和连接管理)
├── models/           (已存在)
│   ├── mod.rs
│   ├── download_status.rs
│   └── paper.rs
├── operations/       (已存在)
│   ├── mod.rs
│   ├── create.rs
│   ├── read.rs
│   ├── update.rs
│   └── delete.rs
├── migrations/       (新增)
│   ├── mod.rs
│   └── schema.rs
└── connection/       (新增)
    ├── mod.rs
    ├── pool.rs
    └── config.rs
```

**重构任务**:
- [ ] 将数据库连接管理抽取到 `connection/` 模块
- [ ] 将数据库迁移逻辑移到 `migrations/` 模块
- [ ] 优化 CRUD 操作的组织结构
- [ ] 简化 `mod.rs` 文件，仅保留接口定义

### 5. 设置视图重构 (中优先级)
**目标文件**: `ui/views/settings.rs` (307行)

**计划结构**:
```
src/ui/views/settings/
├── mod.rs (主要设置视图组装)
├── appearance.rs     (主题、语言设置)
├── shortcuts.rs      (快捷键配置)
├── downloads.rs      (下载设置)
├── advanced.rs       (高级设置)
└── components/       (设置相关组件)
    ├── mod.rs
    ├── theme_selector.rs
    ├── language_selector.rs
    └── shortcut_editor.rs
```

**重构任务**:
- [ ] 按功能区域拆分设置界面
- [ ] 创建可复用的设置组件
- [ ] 优化设置状态管理
- [ ] 改进设置验证逻辑

### 6. 主应用模块重构 (中优先级)
**目标文件**: `core/app.rs` 或主应用逻辑

**计划结构**:
```
src/core/
├── app.rs (简化的主应用结构)
├── state/
│   ├── mod.rs
│   ├── ui_state.rs
│   ├── search_state.rs
│   └── download_state.rs
├── handlers/ (已存在，继续完善)
├── models/ (已存在)
└── events/
    ├── mod.rs
    ├── search_events.rs
    ├── download_events.rs
    └── ui_events.rs
```

### 7. UI组件重构 (低优先级)
**目标**: 优化UI组件的组织结构

**计划结构**:
```
src/ui/
├── mod.rs
├── theme/ (主题相关)
│   ├── mod.rs
│   ├── gruvbox.rs
│   ├── colors.rs
│   └── styles.rs
├── components/ (通用组件)
│   ├── mod.rs
│   ├── paper_card.rs
│   ├── search_bar.rs
│   ├── download_progress.rs
│   └── tab_bar.rs
├── views/ (页面视图)
│   ├── mod.rs
│   ├── search/
│   ├── library/
│   ├── downloads/
│   └── settings/ (重构后)
└── layouts/ (布局组件)
    ├── mod.rs
    ├── main_layout.rs
    └── sidebar.rs
```

### 8. 配置管理重构 (低优先级)
**目标**: 统一配置管理

**计划结构**:
```
src/config/
├── mod.rs
├── app_config.rs     (应用配置)
├── user_settings.rs  (用户设置)
├── shortcuts.rs      (快捷键配置)
└── validation.rs     (配置验证)
```

### 9. 错误处理重构 (低优先级)
**目标**: 统一错误处理机制

**计划结构**:
```
src/utils/
├── mod.rs
├── error/
│   ├── mod.rs
│   ├── app_error.rs
│   ├── database_error.rs
│   ├── network_error.rs
│   └── ui_error.rs
└── result.rs (统一Result类型)
```

---

## 代码质量改进 📈

### A. 依赖关系优化
- [ ] 减少模块间的循环依赖
- [ ] 明确定义模块边界和职责
- [ ] 优化导入语句的组织

### B. 测试覆盖率
- [ ] 为新模块添加单元测试
- [ ] 创建集成测试套件
- [ ] 添加性能基准测试

### C. 文档完善
- [ ] 为每个模块添加详细文档
- [ ] 更新架构设计文档
- [ ] 创建开发者指南

### D. 性能优化
- [ ] 优化数据库查询
- [ ] 减少不必要的克隆操作
- [ ] 优化UI渲染性能

---

## 重构指导原则 🎯

### 1. 单一职责原则
- 每个模块应该只有一个改变的理由
- 避免将不相关的功能放在同一个文件中

### 2. 依赖倒置原则
- 高层模块不应该依赖低层模块
- 两者都应该依赖于抽象

### 3. 开闭原则
- 模块应该对扩展开放，对修改封闭
- 使用trait和泛型实现可扩展性

### 4. 接口隔离原则
- 不强迫客户端依赖它们不使用的接口
- 创建小而专一的trait

### 5. 最小知识原则
- 模块应该尽量少地了解其他模块的内部实现
- 通过明确的接口进行交互

---

## 重构流程建议 🔄

### 阶段1: 数据库模块 (1-2天)
1. 创建新的模块结构
2. 迁移现有代码到新模块
3. 更新导入和依赖
4. 测试编译和功能

### 阶段2: 设置视图 (1天)
1. 拆分设置界面组件
2. 创建可复用组件
3. 优化状态管理
4. 测试UI功能

### 阶段3: 其他模块 (1-2天)
1. 按优先级逐个重构
2. 持续测试和验证
3. 优化性能和内存使用

### 阶段4: 质量提升 (持续)
1. 添加测试覆盖
2. 完善文档
3. 性能优化
4. 代码审查

---

## 风险评估与缓解 ⚠️

### 高风险项目
- **数据库模块重构**: 可能影响数据持久化
  - *缓解*: 先备份数据，分步迁移，充分测试

- **UI组件重构**: 可能破坏用户界面
  - *缓解*: 保持功能等价，逐步替换组件

### 中风险项目
- **配置管理**: 可能影响用户设置
  - *缓解*: 保持配置格式兼容，提供迁移路径

### 低风险项目
- **文档和测试**: 主要是添加内容，不影响现有功能

---

## 完成标准 ✨

### 功能完整性
- [ ] 所有现有功能正常工作
- [ ] 新架构提供相同或更好的性能
- [ ] 用户体验保持一致

### 代码质量
- [ ] 编译无警告
- [ ] 所有测试通过
- [ ] 代码覆盖率达到预期

### 维护性
- [ ] 模块职责清晰
- [ ] 依赖关系简单
- [ ] 文档完整准确

---

*最后更新: 2025年6月10日*
*估计完成时间: 4-6天*
