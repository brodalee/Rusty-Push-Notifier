use repository::repository::Repository;
use service::notification_service::NotificationService;
use service::user_service::UserService;

#[derive(Clone)]
pub struct AppState {
    pub(crate) user_service: UserService,
    pub(crate) notification_service: NotificationService,
}

#[derive(Clone)]
pub struct AppStateChecker {
    pub repository: Repository,
}