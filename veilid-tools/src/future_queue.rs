use super::*;
use futures_util::StreamExt as _;
use stop_token::future::FutureExt as _;

pub async fn process_batched_future_queue<I, C, F, R>(
    future_queue: I,
    batch_size: usize,
    stop_token: StopToken,
    result_callback: C,
) where
    I: IntoIterator,
    C: Fn(R) -> F,
    F: Future<Output = ()>,
    <I as std::iter::IntoIterator>::Item: core::future::Future<Output = R>,
{
    let mut buffered_futures =
        futures_util::stream::iter(future_queue).buffer_unordered(batch_size);
    while let Ok(Some(res)) = buffered_futures.next().timeout_at(stop_token.clone()).await {
        result_callback(res).await;
    }
}
