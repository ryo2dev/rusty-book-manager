use async_trait::async_trait;

#[async_trait] // 1
pub trait HealthCheckRepository: Send + Sync {
    // 2
    async fn check_db(&self) -> bool; // 3
}
