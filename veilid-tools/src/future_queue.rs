use super::*;
use futures_util::{Stream, StreamExt as _};
use stop_token::future::FutureExt as _;

pub async fn process_batched_future_queue_result<I, C, E, R>(
    future_queue: I,
    batch_size: usize,
    stop_token: StopToken,
    result_callback: C,
) -> Result<(), E>
where
    I: IntoIterator,
    C: Fn(R) -> Result<(), E>,
    <I as std::iter::IntoIterator>::Item: core::future::Future<Output = R>,
{
    let mut buffered_futures =
        futures_util::stream::iter(future_queue).buffer_unordered(batch_size);
    while let Ok(Some(res)) = buffered_futures.next().timeout_at(stop_token.clone()).await {
        result_callback(res)?;
    }

    Ok(())
}

pub async fn process_batched_future_queue_void<I>(
    future_queue: I,
    batch_size: usize,
    stop_token: StopToken,
) where
    I: IntoIterator,
    <I as std::iter::IntoIterator>::Item: core::future::Future<Output = ()>,
{
    let mut buffered_futures =
        futures_util::stream::iter(future_queue).buffer_unordered(batch_size);
    while let Ok(Some(())) = buffered_futures.next().timeout_at(stop_token.clone()).await {}
}

pub async fn process_batched_future_stream_result<S, C, E, R>(
    future_stream: S,
    batch_size: usize,
    stop_token: StopToken,
    result_callback: C,
) -> Result<(), E>
where
    S: Stream,
    C: Fn(R) -> Result<(), E>,
    <S as Stream>::Item: core::future::Future<Output = R>,
{
    let mut buffered_futures = Box::pin(future_stream.buffer_unordered(batch_size));
    while let Ok(Some(res)) = buffered_futures.next().timeout_at(stop_token.clone()).await {
        result_callback(res)?;
    }

    Ok(())
}

pub async fn process_batched_future_stream_void<S>(
    future_stream: S,
    batch_size: usize,
    stop_token: StopToken,
) where
    S: Stream,
    <S as Stream>::Item: core::future::Future<Output = ()>,
{
    let mut buffered_futures = Box::pin(future_stream.buffer_unordered(batch_size));
    while let Ok(Some(())) = buffered_futures.next().timeout_at(stop_token.clone()).await {}
}
