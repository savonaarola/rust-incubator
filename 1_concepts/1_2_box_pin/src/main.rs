use std::future::Future;
use std::pin::Pin;
use std::fmt;
use std::rc::Rc;
use std::task::Poll;
use std::time::Instant;

trait SayHi: fmt::Debug {
    fn say_hi(self: Pin<&Self>){
        println!("Hi from {:?}",self)
    }
}
trait MutMeSomehow {
    fn mut_me_somehow(self: Pin<&mut Self>) {
        // Implementation must be meaningful, and
        // obviously call something requiring `&mut self`.
        // The point here is to practice dealing with
        // `Pin<&mut Self>` -> `&mut self` conversion
        // in different contexts, without introducing 
        // any `Unpin` trait bounds.
    }
}
impl<T: Clone> MutMeSomehow for Vec<T>{
    fn mut_me_somehow(self: Pin<&mut Self>) {
        let v= unsafe {self.get_unchecked_mut()};
        if let Some(lst) = v.last(){
            v.push(lst.clone());
        }
    }
}

impl MutMeSomehow for String{
    fn mut_me_somehow(self: Pin<&mut Self>) {
        let s = unsafe {self.get_unchecked_mut()};
        s.push_str(" hello");
    }
}
impl<T: fmt::Debug> SayHi for Box<T>{
    fn say_hi(self: Pin<&Self>){
        println!("Hi from {:?}",self)
    }
}
impl<T: fmt::Debug> SayHi for Rc<T>{
    fn say_hi(self: Pin<&Self>) {
        println!("Hi from {:?}",self)
    }
}
impl<T: fmt::Debug> SayHi for Vec<T>{
    fn say_hi(self: Pin<&Self>) {
        println!("Hi from {:?}",self)
    }
}
impl SayHi for String{
    fn say_hi(self: Pin<&Self>) {
        println!("Hi from {:?}",self)
    }
}
impl SayHi for &[u8]{
    fn say_hi(self: Pin<&Self>) {
        println!("Hi from {:?}",self)
    }
}

#[pin_project::pin_project]
struct MeasurableFuture<Fut> {
    #[pin]
    inner_future: Fut,
    started_at: Option<std::time::Instant>,
}

impl<Fut: Future> Future for MeasurableFuture<Fut>{
    type Output = (Fut::Output, u128);

    fn poll(self: Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> std::task::Poll<Self::Output> {
        let mut this = self.project();
        let start = this.started_at.get_or_insert(Instant::now());
        let inner_poll = this.inner_future.as_mut().poll(cx);
        

        match inner_poll {
            Poll::Pending => Poll::Pending,
            Poll::Ready(output) => {
                let elapsed = start.elapsed();
                Poll::Ready((output, elapsed.as_nanos()))},
        }
    }
}
fn main() {
    let b: Box<i32> = Box::new(5);
    SayHi::say_hi(Pin::new(&b));

    let r: Rc<i32> = Rc::new(10);
    SayHi::say_hi(Pin::new(&r));

    let mut v = Vec::new();
    v.push(1);
    v.push(2);
    SayHi::say_hi(Pin::new(&v));

    let mut s: String = String::from("My string");
    SayHi::say_hi(Pin::new(&s));

    let k: &[u8] = &[1, 2, 3];
    SayHi::say_hi(Pin::new(&k));

    MutMeSomehow::mut_me_somehow(Pin::new(&mut v));
    SayHi::say_hi(Pin::new(&v));

    MutMeSomehow::mut_me_somehow(Pin::new(&mut s));
    SayHi::say_hi(Pin::new(&s));
}
