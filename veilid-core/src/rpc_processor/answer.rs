use super::*;

#[derive(Clone, Debug, Default)]
pub struct Answer<T> {
    /// Hpw long it took to get this answer
    pub _latency: TimestampDuration,
    /// The private route requested to receive the reply
    pub reply_private_route: Option<PublicKey>,
    /// The answer itself
    pub answer: T,
}
impl<T> Answer<T> {
    pub fn new(
        latency: TimestampDuration,
        reply_private_route: Option<PublicKey>,
        answer: T,
    ) -> Self {
        Self {
            _latency: latency,
            reply_private_route,
            answer,
        }
    }
}
