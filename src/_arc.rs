use tokio::sync::Mutex;
use std::sync::Arc;

struct AppState {
    request_count: Mutex<u64>,
}

async fn handle_request(state: Arc<AppState>) {
    let mut counter = state.request_count.lock().await;
    *counter += 1;
    println!("Total request: {}", *counter);
}

#[cfg(test)]
mod test_arc {
    use super::AppState;
    use super::handle_request;
    use std::sync::Arc;
    use std::thread;
    use std::time::Duration;

    use tokio::sync::Mutex;
    use tokio::task;

    #[tokio::test]
    async fn test_arc_mutex_increment() {
        let state = Arc::new(AppState {
            request_count: Mutex::new(0 as u64),
        });

        let mut handles = vec![];

        for _ in 0..5 {
            let state = Arc::clone(&state);
            handles.push(task::spawn(async move {
                handle_request(state).await;
            }));
        }

        for h in handles {
            h.await.unwrap();
        }
    }

    #[tokio::test]
    async fn test_arc_multi_thread() {
        let t1 = thread::spawn(|| {
            for i in 1..=3 {
                println!("Thread A: {}", i);
                thread::sleep(Duration::from_millis(500));
            }
        });

        let t2 = thread::spawn(|| {
            for i in 1..=3 {
                println!("Thread B: {}", i);
                thread::sleep(Duration::from_millis(500));
            }
        });

        t1.join().unwrap();
        t2.join().unwrap();
    }
}
