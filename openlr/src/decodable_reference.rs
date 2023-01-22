use crate::decoding_parameters::DecodingParameters;
use crate::errors::OpenLrErr;
use crate::request_context::RequestContext;
use async_trait::async_trait;

#[async_trait]
pub trait DecodableReference {
    type Peer;
    async fn decode(
        &self,
        context: &RequestContext<DecodingParameters>,
    ) -> Result<Self::Peer, OpenLrErr>;
}
