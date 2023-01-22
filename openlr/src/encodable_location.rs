use crate::encoding_parameters::EncodingParameters;
use crate::errors::OpenLrErr;
use crate::request_context::RequestContext;
use async_trait::async_trait;

#[async_trait]
pub trait EncodableLocation {
    type Peer;
    async fn encode(
        &self,
        context: &RequestContext<EncodingParameters>,
    ) -> Result<Self::Peer, OpenLrErr>;
}
