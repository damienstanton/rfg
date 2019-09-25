//! `main.rs`
//!```no_run
//!fn main() {
//!    let future = hello_world();
//!    assert_eq!("Hello from the future!", block_on(future));
//!   
//!    let mut pool = ThreadPool::new().expect("Error creating thread pool");
//!    pool.run(simple_stream());
//!    pool.run(mapped_stream());
//!    pool.run(comms_between_streams());
//!    pool.run(stream_combinators());
//!}
//!```
use futures::{
    executor::{block_on, ThreadPool},
    stream::{self, StreamExt},
    channel::mpsc::{self, TryRecvError}
};

/// a simple hello world example
///```
///pub async fn hello_world() -> String {
///    String::from("Hello from the future!")
///}
///```
pub async fn hello_world() -> String {
    String::from("Hello from the future!")
}


/// an example of converting streams into futures
///```
///use futures::stream::{self, StreamExt};
///
///pub async fn simple_stream() {
///    let stream = stream::iter(1..=3);
///    let (first, stream) = stream.into_future().await;
///    let (second, _) = stream.into_future().await;
///   
///    // test it
///    assert_eq!(Some(1), first);
///    assert_eq!(Some(2), second);
///}
///```
pub async fn simple_stream() {
    let stream = stream::iter(1..=3);
    let (first, stream) = stream.into_future().await;
    assert_eq!(Some(1), first);
    let (second, _) = stream.into_future().await;
    assert_eq!(Some(2), second);
}

/// an example of taking a stream and maping its values into a concrete collection
///```
///use futures::stream;
///
///pub async fn mapped_stream() {
///    let stream = stream::iter(1..=3);
///    let stream = stream.map(|x| x + 3);
///    
///    // test it
///    assert_eq!(vec![4, 5, 6], stream.collect::<Vec<_>>().await);
///}
///```
pub async fn mapped_stream() {
    let stream = stream::iter(1..=3);
    let stream = stream.map(|x| x + 3);
    
    assert_eq!(vec![4, 5, 6], stream.collect::<Vec<_>>().await);
}


/// an example of communications between streams using the futures impl of mpsc::channel
/// ```
///use futures::stream::{self, StreamExt};
///use futures::mpsc::channel;
///
///
///pub async fn comms_between_streams() -> Result<i32, TryRecvError> {
///    let stream_size = 3;
///    let stream_a = stream::iter(1..=3);
///    let stream_b = stream::iter(4..=6);
///    let (mut tx, mut rx) = mpsc::channel(stream_size);
///    let send_a = tx.try_send(stream_a);
///    let send_b = tx.try_send(stream_b);
///    
///    Ok(rx.try_next())
///
///    if send_a.is_ok() {
///        let rec_a = rx.try_next();
///        assert_eq!(
///            1, rec_a
///            .unwrap() 
///            .unwrap() 
///            .into_future()
///            .await
///            .0
///            .unwrap()
///        );
///    }
///    if send_b.is_ok() {
///        let rec_b = rx.try_next();
///        assert_eq!(
///            4, rec_b
///            .unwrap() 
///            .unwrap() 
///            .into_future()
///            .await
///            .0
///            .unwrap()
///        );
///    }
/// }
/// ```
pub async fn comms_between_streams() {
    let stream_size = 3;
    let stream_a = stream::iter(1..=3);
    let stream_b = stream::iter(4..=6);
    let (mut tx, mut rx) = mpsc::channel(stream_size);
    let send_a = tx.try_send(stream_a);
    let send_b = tx.try_send(stream_b);
    
    if send_a.is_ok() {
        let rec_a = rx.try_next();
        assert_eq!(
            1, rec_a
            .unwrap() 
            .unwrap() 
            .into_future()
            .await
            .0
            .unwrap()
        );
    }
    if send_b.is_ok() {
        let rec_b = rx.try_next();
        assert_eq!(
            4, rec_b
            .unwrap() 
            .unwrap() 
            .into_future()
            .await
            .0
            .unwrap()
        );
    }
}

/// an example of stream combinators, combining two streams into one future-able stream type
///```
///use futures::stream;
///
///pub async fn stream_combinators() {
///    let stream_a = stream::iter(1..=3);
///    let stream_b = stream::iter(4..=6);
///    let mut select = futures::stream::select(stream_a, stream_b);
///    let v1 = select.get_mut().0.into_future().await;
///    assert_eq!(1, v1.0.unwrap());
///    let v2 = select.get_mut().1.into_future().await;
///    assert_eq!(4, v2.0.unwrap());
///}
///```
pub async fn stream_combinators() {
    let stream_a = stream::iter(1..=3);
    let stream_b = stream::iter(4..=6);
    let mut select = futures::stream::select(stream_a, stream_b);
    let v1 = select.get_mut().0.into_future().await;
    assert_eq!(1, v1.0.unwrap());
    let v2 = select.get_mut().1.into_future().await;
    assert_eq!(4, v2.0.unwrap());
}

fn main() {
    let future = hello_world();
    assert_eq!("Hello from the future!", block_on(future));
    
    let mut pool = ThreadPool::new().expect("Error creating thread pool");
    pool.run(simple_stream());
    pool.run(mapped_stream());
    pool.run(comms_between_streams());
    pool.run(stream_combinators());
}
