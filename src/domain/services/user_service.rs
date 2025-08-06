#[derive(Clone)]
pub struct UserService<T: UserRepository> {
    user_repo: T,
}

impl<T: UserRepository> UserService<T> {
    pub fn new(user_repo: T) -> Self {
        UserService
    }
}
