use async_trait::async_trait;

#[async_trait]
pub trait UserViewRepository {
    // TODO what if I do a trait
    async fn save_view(&self, )
    async fn save(&self, match_request: MatchRequest) -> Result<(), MatchRequestDomainError>;
    async fn search_by_criteria(&self, match_request_criteria: MatchRequestCriteria) -> Result<Vec<MatchRequest>, MatchRequestDomainError>;
}
