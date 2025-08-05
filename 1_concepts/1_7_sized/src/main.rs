use core::str;
use std::borrow::Cow;
use std::fmt::Debug;
use std::collections::HashMap;
use rand::Rng;

trait Storage<K,V>: Debug{
    fn set(&mut self, key: K, val: V);
    fn get(&self, key: &K) -> Option<&V>;
    fn remove(&mut self, key: &K) -> Option<V>;
}

#[derive(Debug)]
struct User{
    id: u64,
    email: Cow<'static, str>,
    activated: bool,
}

#[derive(Debug)]
struct StorageUsers{
    storage: HashMap<u64,User>,
}

impl StorageUsers{
    fn new() -> Self{
        StorageUsers { storage: HashMap::new() }
    }
}

impl Storage<u64,User> for StorageUsers{
    fn set(&mut self, key: u64, val: User) {
        self.storage.insert(key,val);
    }
    fn get(&self, key: &u64) -> Option<&User> {
        self.storage.get(key)
    }
    fn remove(&mut self, key: &u64) -> Option<User> {
        self.storage.remove(key)
    }
}
#[derive(Debug)]
struct DynUserRepository{
    repo: Box<dyn Storage<u64,User>>,
}
impl DynUserRepository{
    fn new(storage: Box<dyn Storage<u64,User>>) -> Self{
        DynUserRepository { repo: storage}
    }    
}
impl UserRepository for DynUserRepository{

    fn add_user(&mut self, user: User){
        self.repo.set(user.id, user);
    }
    fn get_user(&self, user_id: u64) -> Option<&User>{
        self.repo.get(&user_id)
    }
    fn remove_user(&mut self, user_id: u64) -> Option<User>{
        self.repo.remove(&user_id)
    }
    fn update_user(&mut self, user: User){
        self.repo.set(user.id, user);
    }
}


trait Command{

}
#[derive(Debug)]
struct CreateUser{
    email: String,
    activated: bool,
}
impl CreateUser {
    fn into_user(self, id: u64) -> User{
        User { id: id, email: Cow::Owned(self.email), activated: self.activated }
    }
}
impl Command for CreateUser{}
#[derive(Debug)]
enum UserError{
    UserAlreadyExists,
    InvalidEmail
}
trait CommandHandler<C: Command> {
    type Context: ?Sized;
    type Result;

    fn handle_command(&self, cmd: &C, ctx: &mut Self::Context) -> Self::Result;
}

impl CommandHandler<CreateUser> for User {
    type Context = dyn UserRepository;
    type Result = Result<(), UserError>;
    
    fn handle_command(&self, cmd: &CreateUser, user_repo: &mut Self::Context) -> Self::Result {
        // Here we operate with the `UserRepository`
        // via its trait object `&dyn UserRepository`
        let user_id = rand::rng().random_range(0..=9999);
        
        let new_user = User{
            id: user_id,
            email: Cow::Owned(cmd.email.clone()),
            activated: cmd.activated
        };
        user_repo.add_user(new_user);

        Ok(())
    }
}

trait UserRepository{
    fn add_user(&mut self, user: User);
    fn get_user(&self, user_id: u64) -> Option<&User>;
    fn remove_user(&mut self, user_id: u64) -> Option<User>;
    fn update_user(&mut self, user: User);
}

struct MockUserRepo{
    users: HashMap<u64, User>,
    calls: Vec<String>,
}
impl MockUserRepo{
    fn new() -> Self {
        MockUserRepo {
            users: HashMap::new(),
            calls: Vec::new(),
        }
    }
    
    fn get_calls(&self) -> &Vec<String> {
        &self.calls
    }
    
    fn user_count(&self) -> usize {
        self.users.len()
    }
}

impl UserRepository for MockUserRepo{
    fn add_user(&mut self, user: User) {
        self.calls.push(format!("add_user(id: {})", user.id));
        self.users.insert(user.id, user);
    }
    
    fn get_user(&self, user_id: u64) -> Option<&User> {
        self.users.get(&user_id)
    }
    
    fn remove_user(&mut self, user_id: u64) -> Option<User> {
        self.calls.push(format!("remove_user(id: {})", user_id));
        self.users.remove(&user_id)
    }
    
    fn update_user(&mut self, user: User) {
        self.calls.push(format!("update_user(id: {})", user.id));
        self.users.insert(user.id, user);
    }
}

fn main() {
    let storage = StorageUsers::new();
    let mut dyn_repo = DynUserRepository::new(Box::new(storage));

    let actor = User {
        id: 0,
        email: Cow::Borrowed("admin@example.com"),
        activated: true,
    };

    let cmd = CreateUser {
        email: "user1@example.com".to_string(),
        activated: true,
    };
    match actor.handle_command(&cmd, &mut dyn_repo) {
        Ok(()) => println!("User created successfully"),
        Err(e) => println!("Error: {:?}", e),
    }
    println!("{:?}",dyn_repo);


    let mut mock_repo = MockUserRepo::new();
    
    let cmd3 = CreateUser {
        email: "user3@example.com".to_string(),
        activated: true,
    };
    
    match actor.handle_command(&cmd3, &mut mock_repo) {
        Ok(()) => {
            println!("User created successfully!");
            println!("Mock calls: {:?}", mock_repo.get_calls());
            println!("Users in mock: {}", mock_repo.user_count());
        },
        Err(e) => println!("Error: {:?}", e),
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_user_command_with_mock() {

        let mut mock_repo = MockUserRepo::new();
        let actor = User {
            id: 999,
            email: Cow::Borrowed("actor@example.com"),
            activated: true,
        };
        let cmd = CreateUser {
            email: "newuser@example.com".to_string(),
            activated: false,
        };

        let result = actor.handle_command(&cmd, &mut mock_repo);


        assert!(result.is_ok(), "Command should succeed");
        assert_eq!(mock_repo.user_count(), 1, "Should have one user");
        
        let calls = mock_repo.get_calls();
        assert_eq!(calls.len(), 1);
        assert!(calls[0].starts_with("add_user"));
    }

    #[test]
    fn test_create_user_with_dynamic_repository() {
        let storage = StorageUsers::new();
        let mut dyn_repo = DynUserRepository::new(Box::new(storage));
        let actor = User {
            id: 1,
            email: Cow::Borrowed("test@example.com"),
            activated: true,
        };
        let cmd = CreateUser {
            email: "created@example.com".to_string(),
            activated: true,
        };

        let result = actor.handle_command(&cmd, &mut dyn_repo);

        assert!(result.is_ok());
        
    }

}