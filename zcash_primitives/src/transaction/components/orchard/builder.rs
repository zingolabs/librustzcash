use crate::consensus::{self, BlockHeight, NetworkUpgrade};
use orchard::{
    builder::{Builder, Error, InProgress, Unauthorized, Unproven},
    bundle::Bundle,
};

pub struct WithoutOrchard;

pub struct WithOrchard(pub(crate) Option<Builder>);

pub trait MaybeOrchard {
    fn build<V: core::convert::TryFrom<i64>>(
        self,
        rng: impl rand::RngCore,
    ) -> Option<Result<Bundle<InProgress<Unproven, Unauthorized>, V>, Error>>;
}

impl MaybeOrchard for WithOrchard {
    fn build<V: core::convert::TryFrom<i64>>(
        self,
        rng: impl rand::RngCore,
    ) -> Option<Result<Bundle<InProgress<Unproven, Unauthorized>, V>, Error>> {
        self.0.map(|builder| builder.build(rng))
    }
}

impl MaybeOrchard for WithoutOrchard {
    fn build<V: core::convert::TryFrom<i64>>(
        self,
        _: impl rand::RngCore,
    ) -> Option<Result<Bundle<InProgress<Unproven, Unauthorized>, V>, Error>> {
        None
    }
}

impl WithOrchard {
    pub(crate) fn new<P: consensus::Parameters>(
        params: &P,
        target_height: BlockHeight,
        anchor: orchard::tree::Anchor,
    ) -> Self {
        let orchard_builder = if params.is_nu_active(NetworkUpgrade::Nu5, target_height) {
            Some(orchard::builder::Builder::new(
                orchard::bundle::Flags::from_parts(true, true),
                anchor,
            ))
        } else {
            None
        };

        Self(orchard_builder)
    }
}
