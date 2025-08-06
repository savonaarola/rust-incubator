use std::marker::PhantomData;
use rand::Rng;

#[derive(Debug)]
struct Fact<T>{
    _pht: PhantomData<T>
}
#[derive(Debug)]
struct Type1;
#[derive(Debug)]
struct Type2;

impl<T> Fact<T> where T: HasFacts{
    fn new() -> Self{
        Self { _pht: PhantomData }
    }
    fn fact(&self) -> String{
        let facts = T::get_facts();
        let seed = rand::rng().random_range(0..=facts.len());
        facts[seed].clone()
    }
    
}

trait HasFacts {
    fn get_facts() -> Vec<String>;
}

impl HasFacts for Type1{
    fn get_facts() -> Vec<String> {
        vec!["Type1 is cool".to_string(),"Type1 is awesome".to_string(),"Type1 is unbelivable".to_string(),]
    }
}

impl HasFacts for Type2{
    fn get_facts() -> Vec<String> {
        vec!["Type2 is bad".to_string(),"Type2 is fool".to_string(),"Type2 is disgusting".to_string(),]
    }
}


fn main(){
    let t1: Fact<Type1> = Fact::new();
    let t2: Fact<Type2> = Fact::new();
    println!("{}", t1.fact());
    println!("{}", t2.fact());
}