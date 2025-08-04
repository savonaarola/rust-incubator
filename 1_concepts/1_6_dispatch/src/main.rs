use core::str;
use std::borrow::Cow;
use std::collections::HashMap;
use std::fmt::Debug;

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
struct UserRepository<S> where S: Storage<u64,User>{
    repo: S,
}
impl<S> UserRepository<S> where S: Storage<u64,User>{
    fn new(storage: S) -> Self{
        UserRepository { repo: storage }
    }
    fn add_user(&mut self,user: User){
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

#[derive(Debug)]
struct DynUserRepository{
    repo: Box<dyn Storage<u64,User>>,
}
impl DynUserRepository{
    fn new(storage: Box<dyn Storage<u64,User>>) -> Self{
        DynUserRepository { repo: storage}
    }
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

fn main(){
    let user1 = User{id:0,email: Cow::Owned("stud0@mail.com".to_string()),activated: true};
    let user2 = User{id:1,email: Cow::Owned("vtya112@mail.com".to_string()),activated: false};
    let user3 = User{id:2,email: Cow::Owned("worker4@mail.com".to_string()),activated: true};
    let storage1 = StorageUsers::new();
    let storage2 = StorageUsers::new();
    let mut repository = UserRepository::new(storage1);
    repository.add_user(user1);
    repository.add_user(user2);
    repository.add_user(user3);
    let user2 = repository.remove_user(1).unwrap();
    let mut dyn_repository= DynUserRepository::new(Box::new(storage2));
    dyn_repository.add_user(user2);
    println!("{:?}",repository);
    println!("{:?}",dyn_repository);
}