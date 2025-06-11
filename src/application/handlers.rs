// 命令和查询处理器 - Handlers
// 实现 CQRS 模式的处理器，分离命令和查询的处理逻辑

use std::sync::Arc;
use async_trait::async_trait;

use crate::application::{
    commands::*, queries::*, 
    // use_cases::{PaperManagementUseCase, PaperQueryUseCase},
    ApplicationResult
};

// Temporarily comment out to isolate use_cases compilation issue
use crate::application::use_cases::{PaperManagementUseCase, PaperQueryUseCase};

/// 命令处理器特征
#[async_trait]
pub trait CommandHandler<TCommand> {
    type Response;
    
    async fn handle(&self, command: TCommand) -> ApplicationResult<Self::Response>;
}

/// 查询处理器特征
#[async_trait]
pub trait QueryHandler<TQuery> {
    type Response;
    
    async fn handle(&self, query: TQuery) -> ApplicationResult<Self::Response>;
}

/// 创建论文命令处理器
pub struct CreatePaperHandler {
    use_case: Arc<PaperManagementUseCase>,
}

impl CreatePaperHandler {
    pub fn new(use_case: Arc<PaperManagementUseCase>) -> Self {
        Self { use_case }
    }
}

#[async_trait]
impl CommandHandler<CreatePaperCommand> for CreatePaperHandler {
    type Response = String;
    
    async fn handle(&self, command: CreatePaperCommand) -> ApplicationResult<Self::Response> {
        self.use_case.create_paper(command).await
    }
}

/// 更新论文元数据命令处理器
pub struct UpdatePaperMetadataHandler {
    use_case: Arc<PaperManagementUseCase>,
}

impl UpdatePaperMetadataHandler {
    pub fn new(use_case: Arc<PaperManagementUseCase>) -> Self {
        Self { use_case }
    }
}

#[async_trait]
impl CommandHandler<UpdatePaperMetadataCommand> for UpdatePaperMetadataHandler {
    type Response = ();
    
    async fn handle(&self, command: UpdatePaperMetadataCommand) -> ApplicationResult<Self::Response> {
        self.use_case.update_paper_metadata(command).await
    }
}

/// 更新阅读状态命令处理器
pub struct UpdateReadingStatusHandler {
    use_case: Arc<PaperManagementUseCase>,
}

impl UpdateReadingStatusHandler {
    pub fn new(use_case: Arc<PaperManagementUseCase>) -> Self {
        Self { use_case }
    }
}

#[async_trait]
impl CommandHandler<UpdateReadingStatusCommand> for UpdateReadingStatusHandler {
    type Response = ();
    
    async fn handle(&self, command: UpdateReadingStatusCommand) -> ApplicationResult<Self::Response> {
        self.use_case.update_reading_status(command).await
    }
}

/// 更新论文标签命令处理器
pub struct UpdatePaperTagsHandler {
    use_case: Arc<PaperManagementUseCase>,
}

impl UpdatePaperTagsHandler {
    pub fn new(use_case: Arc<PaperManagementUseCase>) -> Self {
        Self { use_case }
    }
}

#[async_trait]
impl CommandHandler<UpdatePaperTagsCommand> for UpdatePaperTagsHandler {
    type Response = ();
    
    async fn handle(&self, command: UpdatePaperTagsCommand) -> ApplicationResult<Self::Response> {
        self.use_case.update_paper_tags(command).await
    }
}

/// 更新论文评分命令处理器
pub struct UpdatePaperRatingHandler {
    use_case: Arc<PaperManagementUseCase>,
}

impl UpdatePaperRatingHandler {
    pub fn new(use_case: Arc<PaperManagementUseCase>) -> Self {
        Self { use_case }
    }
}

#[async_trait]
impl CommandHandler<UpdatePaperRatingCommand> for UpdatePaperRatingHandler {
    type Response = ();
    
    async fn handle(&self, command: UpdatePaperRatingCommand) -> ApplicationResult<Self::Response> {
        self.use_case.update_paper_rating(command).await
    }
}

/// 设置收藏状态命令处理器
pub struct SetFavoriteHandler {
    use_case: Arc<PaperManagementUseCase>,
}

impl SetFavoriteHandler {
    pub fn new(use_case: Arc<PaperManagementUseCase>) -> Self {
        Self { use_case }
    }
}

#[async_trait]
impl CommandHandler<SetFavoriteCommand> for SetFavoriteHandler {
    type Response = ();
    
    async fn handle(&self, command: SetFavoriteCommand) -> ApplicationResult<Self::Response> {
        self.use_case.set_favorite(command).await
    }
}

/// 删除论文命令处理器
pub struct DeletePaperHandler {
    use_case: Arc<PaperManagementUseCase>,
}

impl DeletePaperHandler {
    pub fn new(use_case: Arc<PaperManagementUseCase>) -> Self {
        Self { use_case }
    }
}

#[async_trait]
impl CommandHandler<DeletePaperCommand> for DeletePaperHandler {
    type Response = ();
    
    async fn handle(&self, command: DeletePaperCommand) -> ApplicationResult<Self::Response> {
        self.use_case.delete_paper(command).await
    }
}

/// 批量更新阅读状态命令处理器
pub struct BatchUpdateReadingStatusHandler {
    use_case: Arc<PaperManagementUseCase>,
}

impl BatchUpdateReadingStatusHandler {
    pub fn new(use_case: Arc<PaperManagementUseCase>) -> Self {
        Self { use_case }
    }
}

#[async_trait]
impl CommandHandler<BatchUpdateReadingStatusCommand> for BatchUpdateReadingStatusHandler {
    type Response = Vec<String>;
    
    async fn handle(&self, command: BatchUpdateReadingStatusCommand) -> ApplicationResult<Self::Response> {
        self.use_case.batch_update_reading_status(command).await
    }
}

/// 根据ID获取论文查询处理器
pub struct GetPaperByIdHandler {
    use_case: Arc<PaperQueryUseCase>,
}

impl GetPaperByIdHandler {
    pub fn new(use_case: Arc<PaperQueryUseCase>) -> Self {
        Self { use_case }
    }
}

#[async_trait]
impl QueryHandler<GetPaperByIdQuery> for GetPaperByIdHandler {
    type Response = Option<crate::application::dto::PaperResponse>;
    
    async fn handle(&self, query: GetPaperByIdQuery) -> ApplicationResult<Self::Response> {
        self.use_case.get_paper_by_id(query).await
    }
}

/// 搜索论文查询处理器
pub struct SearchPapersHandler {
    use_case: Arc<PaperQueryUseCase>,
}

impl SearchPapersHandler {
    pub fn new(use_case: Arc<PaperQueryUseCase>) -> Self {
        Self { use_case }
    }
}

#[async_trait]
impl QueryHandler<SearchPapersQuery> for SearchPapersHandler {
    type Response = crate::application::dto::PaperListResponse;
    
    async fn handle(&self, query: SearchPapersQuery) -> ApplicationResult<Self::Response> {
        self.use_case.search_papers(query).await
    }
}

/// 获取论文统计查询处理器
pub struct GetPaperStatsHandler {
    use_case: Arc<PaperQueryUseCase>,
}

impl GetPaperStatsHandler {
    pub fn new(use_case: Arc<PaperQueryUseCase>) -> Self {
        Self { use_case }
    }
}

#[async_trait]
impl QueryHandler<GetPaperStatsQuery> for GetPaperStatsHandler {
    type Response = crate::application::dto::PaperStatsResponse;
    
    async fn handle(&self, query: GetPaperStatsQuery) -> ApplicationResult<Self::Response> {
        self.use_case.get_paper_stats(query).await
    }
}

/// 命令总线 - 路由命令到相应的处理器
pub struct CommandBus {
    create_paper_handler: Arc<CreatePaperHandler>,
    update_metadata_handler: Arc<UpdatePaperMetadataHandler>,
    update_reading_status_handler: Arc<UpdateReadingStatusHandler>,
    update_tags_handler: Arc<UpdatePaperTagsHandler>,
    update_rating_handler: Arc<UpdatePaperRatingHandler>,
    set_favorite_handler: Arc<SetFavoriteHandler>,
    delete_paper_handler: Arc<DeletePaperHandler>,
    batch_update_reading_status_handler: Arc<BatchUpdateReadingStatusHandler>,
}

impl CommandBus {
    pub fn new(
        create_paper_handler: Arc<CreatePaperHandler>,
        update_metadata_handler: Arc<UpdatePaperMetadataHandler>,
        update_reading_status_handler: Arc<UpdateReadingStatusHandler>,
        update_tags_handler: Arc<UpdatePaperTagsHandler>,
        update_rating_handler: Arc<UpdatePaperRatingHandler>,
        set_favorite_handler: Arc<SetFavoriteHandler>,
        delete_paper_handler: Arc<DeletePaperHandler>,
        batch_update_reading_status_handler: Arc<BatchUpdateReadingStatusHandler>,
    ) -> Self {
        Self {
            create_paper_handler,
            update_metadata_handler,
            update_reading_status_handler,
            update_tags_handler,
            update_rating_handler,
            set_favorite_handler,
            delete_paper_handler,
            batch_update_reading_status_handler,
        }
    }
    
    /// 发送创建论文命令
    pub async fn send_create_paper(&self, command: CreatePaperCommand) -> ApplicationResult<String> {
        self.create_paper_handler.handle(command).await
    }
    
    /// 发送更新元数据命令
    pub async fn send_update_metadata(&self, command: UpdatePaperMetadataCommand) -> ApplicationResult<()> {
        self.update_metadata_handler.handle(command).await
    }
    
    /// 发送更新阅读状态命令
    pub async fn send_update_reading_status(&self, command: UpdateReadingStatusCommand) -> ApplicationResult<()> {
        self.update_reading_status_handler.handle(command).await
    }
    
    /// 发送更新标签命令
    pub async fn send_update_tags(&self, command: UpdatePaperTagsCommand) -> ApplicationResult<()> {
        self.update_tags_handler.handle(command).await
    }
    
    /// 发送更新评分命令
    pub async fn send_update_rating(&self, command: UpdatePaperRatingCommand) -> ApplicationResult<()> {
        self.update_rating_handler.handle(command).await
    }
    
    /// 发送设置收藏命令
    pub async fn send_set_favorite(&self, command: SetFavoriteCommand) -> ApplicationResult<()> {
        self.set_favorite_handler.handle(command).await
    }
    
    /// 发送删除论文命令
    pub async fn send_delete_paper(&self, command: DeletePaperCommand) -> ApplicationResult<()> {
        self.delete_paper_handler.handle(command).await
    }
    
    /// 发送批量更新阅读状态命令
    pub async fn send_batch_update_reading_status(&self, command: BatchUpdateReadingStatusCommand) -> ApplicationResult<Vec<String>> {
        self.batch_update_reading_status_handler.handle(command).await
    }
}

/// 查询总线 - 路由查询到相应的处理器
pub struct QueryBus {
    get_paper_by_id_handler: Arc<GetPaperByIdHandler>,
    search_papers_handler: Arc<SearchPapersHandler>,
    get_stats_handler: Arc<GetPaperStatsHandler>,
}

impl QueryBus {
    pub fn new(
        get_paper_by_id_handler: Arc<GetPaperByIdHandler>,
        search_papers_handler: Arc<SearchPapersHandler>,
        get_stats_handler: Arc<GetPaperStatsHandler>,
    ) -> Self {
        Self {
            get_paper_by_id_handler,
            search_papers_handler,
            get_stats_handler,
        }
    }
    
    /// 发送根据ID获取论文查询
    pub async fn send_get_paper_by_id(&self, query: GetPaperByIdQuery) -> ApplicationResult<Option<crate::application::dto::PaperResponse>> {
        self.get_paper_by_id_handler.handle(query).await
    }
    
    /// 发送搜索论文查询
    pub async fn send_search_papers(&self, query: SearchPapersQuery) -> ApplicationResult<crate::application::dto::PaperListResponse> {
        self.search_papers_handler.handle(query).await
    }
    
    /// 发送获取统计查询
    pub async fn send_get_stats(&self, query: GetPaperStatsQuery) -> ApplicationResult<crate::application::dto::PaperStatsResponse> {
        self.get_stats_handler.handle(query).await
    }
}

/// 应用服务 - 统一的应用层入口
pub struct ApplicationService {
    command_bus: Arc<CommandBus>,
    query_bus: Arc<QueryBus>,
}

impl ApplicationService {
    pub fn new(command_bus: Arc<CommandBus>, query_bus: Arc<QueryBus>) -> Self {
        Self {
            command_bus,
            query_bus,
        }
    }
    
    /// 获取命令总线
    pub fn commands(&self) -> &CommandBus {
        &self.command_bus
    }
    
    /// 获取查询总线
    pub fn queries(&self) -> &QueryBus {
        &self.query_bus
    }
}

/// 处理器工厂 - 创建和配置所有处理器
pub struct HandlerFactory;

impl HandlerFactory {
    /// 构建命令总线
    pub fn build_command_bus(paper_use_case: Arc<PaperManagementUseCase>) -> Arc<CommandBus> {
        let create_paper_handler = Arc::new(CreatePaperHandler::new(paper_use_case.clone()));
        let update_metadata_handler = Arc::new(UpdatePaperMetadataHandler::new(paper_use_case.clone()));
        let update_reading_status_handler = Arc::new(UpdateReadingStatusHandler::new(paper_use_case.clone()));
        let update_tags_handler = Arc::new(UpdatePaperTagsHandler::new(paper_use_case.clone()));
        let update_rating_handler = Arc::new(UpdatePaperRatingHandler::new(paper_use_case.clone()));
        let set_favorite_handler = Arc::new(SetFavoriteHandler::new(paper_use_case.clone()));
        let delete_paper_handler = Arc::new(DeletePaperHandler::new(paper_use_case.clone()));
        let batch_update_reading_status_handler = Arc::new(BatchUpdateReadingStatusHandler::new(paper_use_case));
        
        Arc::new(CommandBus::new(
            create_paper_handler,
            update_metadata_handler,
            update_reading_status_handler,
            update_tags_handler,
            update_rating_handler,
            set_favorite_handler,
            delete_paper_handler,
            batch_update_reading_status_handler,
        ))
    }
    
    /// 构建查询总线
    pub fn build_query_bus(query_use_case: Arc<PaperQueryUseCase>) -> Arc<QueryBus> {
        let get_paper_by_id_handler = Arc::new(GetPaperByIdHandler::new(query_use_case.clone()));
        let search_papers_handler = Arc::new(SearchPapersHandler::new(query_use_case.clone()));
        let get_stats_handler = Arc::new(GetPaperStatsHandler::new(query_use_case));
        
        Arc::new(QueryBus::new(
            get_paper_by_id_handler,
            search_papers_handler,
            get_stats_handler,
        ))
    }
    
    /// 构建完整的应用服务
    pub fn build_application_service(
        paper_use_case: Arc<PaperManagementUseCase>,
        query_use_case: Arc<PaperQueryUseCase>,
    ) -> Arc<ApplicationService> {
        let command_bus = Self::build_command_bus(paper_use_case);
        let query_bus = Self::build_query_bus(query_use_case);
        
        Arc::new(ApplicationService::new(command_bus, query_bus))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    // Mock implementations for testing
    
    struct MockPaperManagementUseCase;
    
    impl MockPaperManagementUseCase {
        #[allow(dead_code)]
        async fn create_paper(&self, _command: CreatePaperCommand) -> ApplicationResult<String> {
            Ok("test-paper-id".to_string())
        }
        
        #[allow(dead_code)]
        async fn update_reading_status(&self, _command: UpdateReadingStatusCommand) -> ApplicationResult<()> {
            Ok(())
        }
    }
    
    #[tokio::test]
    async fn test_handler_factory() {
        // 这里需要实际的用例实现来进行测试
        // 由于依赖于具体的存储库实现，这里只测试工厂方法的结构
        
        // 确保工厂方法能够编译
        // let use_case = Arc::new(MockPaperManagementUseCase);
        // let command_bus = HandlerFactory::build_command_bus(use_case);
        // assert!(command_bus.is_ok());
    }
}
