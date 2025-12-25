struct Foo;
impl Future for Foo {
    type Output = String;
    // // original: fn poll(self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> std::task::Poll<Self::Output> {
    fn poll(self: std::pin::Pin<&mut Self>, _: &mut std::task::Context<'_>) -> std::task::Poll<Self::Output> {
        std::task::Poll::Ready("hello world".to_string())
    }
}

async fn foo() -> String {
    "hello world".to_string()
}

// ini setara interface di golang
trait CounterRepo {
    // Future which:
    // - generic against lifetime 'a
    // - output i32
    type GetFuture<'a>: std::future::Future<Output = i32> where Self:'a;
    // self borrowed as long as 'a
    fn get<'a>(&'a self) -> Self::GetFuture<'a>;
}
struct Counter {
    value: i32,
}
impl CounterRepo for Counter {
    // impl Future = anonym future result from async block
    type GetFuture<'a> = std::pin::Pin<Box<dyn std::future::Future<Output = i32> + Send + 'a>>;

    fn get<'a>(&'a self) -> Self::GetFuture<'a> {
        Box::pin(async move {
            self.value
        })
    }
}

trait UserRepository {
    fn find_by_id<'a>(&'a self, id: i32) -> std::pin::Pin<Box<dyn Future<Output = String> + Send + 'a>>;
}
struct UserRepo;
impl UserRepository for UserRepo {
    fn find_by_id<'a>(&'a self, id: i32) -> std::pin::Pin<Box<dyn Future<Output = String> + Send + 'a>> {
        Box::pin(async move {
            format!("user id {}", id)
        })
    }
}

#[cfg(test)]
mod test_future {
    use crate::_future::Counter;
    use crate::_future::CounterRepo;
    use crate::_future::Foo;
    use crate::_future::UserRepo;
    use crate::_future::UserRepository;
    use crate::_future::foo;

    #[tokio::test]
    async fn test_future() {
        let a = Foo;
        let b = foo();
        println!("{}", a.await);
        println!("{}", b.await);
    }

    #[tokio::test]
    async fn test_future_counter() {
        let counter = Counter {value: 42};
        let result = counter.get().await;
        println!("result: {}", result)
    }

    #[tokio::test]
    async fn test_future_user_repo() {
        let user_repo = UserRepo;
        let result = user_repo.find_by_id(10).await;
        println!("{:?}", result)
    }
}
