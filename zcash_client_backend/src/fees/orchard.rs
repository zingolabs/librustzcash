//! Types related to computation of fees and change related to the Orchard components
//! of a transaction.

use std::convert::Infallible;

use orchard::{BundleActionCountError, BundleProtocol};
use zcash_protocol::value::Zatoshis;

pub(crate) fn action_count_error(err: BundleActionCountError) -> &'static str {
    match err {
        BundleActionCountError::InputCountOverflow => {
            "Requested spend and output counts overflowed."
        }
        BundleActionCountError::SpendsDisabled => "Spends are disabled for this bundle.",
        BundleActionCountError::OutputsDisabled => "Outputs are disabled for this bundle.",
        _ => "The requested Orchard action count violates bundle constraints.",
    }
}

/// A trait that provides a minimized view of Orchard-style bundle configuration
/// suitable for use in fee and change calculation.
pub trait BundleView<NoteRef> {
    /// The type of inputs to the bundle.
    type In: InputView<NoteRef>;
    /// The type of inputs of the bundle.
    type Out: OutputView;

    /// Returns the protocol rules for the bundle.
    fn bundle_type(&self) -> BundleProtocol;
    /// Returns the inputs to the bundle.
    fn inputs(&self) -> &[Self::In];
    /// Returns the outputs of the bundle.
    fn outputs(&self) -> &[Self::Out];
}

impl<'a, NoteRef, In: InputView<NoteRef>, Out: OutputView> BundleView<NoteRef>
    for (BundleProtocol, &'a [In], &'a [Out])
{
    type In = In;
    type Out = Out;

    fn bundle_type(&self) -> BundleProtocol {
        self.0
    }

    fn inputs(&self) -> &[In] {
        self.1
    }

    fn outputs(&self) -> &[Out] {
        self.2
    }
}

/// A [`BundleView`] for the empty legacy Orchard bundle.
pub struct EmptyBundleView;

impl<NoteRef> BundleView<NoteRef> for EmptyBundleView {
    type In = Infallible;
    type Out = Infallible;

    fn bundle_type(&self) -> BundleProtocol {
        BundleProtocol::LegacyOrchard
    }

    fn inputs(&self) -> &[Self::In] {
        &[]
    }

    fn outputs(&self) -> &[Self::Out] {
        &[]
    }
}

/// A trait that provides a minimized view of an Orchard input suitable for use in fee and change
/// calculation.
pub trait InputView<NoteRef> {
    /// An identifier for the input being spent.
    fn note_id(&self) -> &NoteRef;
    /// The value of the input being spent.
    fn value(&self) -> Zatoshis;
}

impl<N> InputView<N> for Infallible {
    fn note_id(&self) -> &N {
        unreachable!()
    }
    fn value(&self) -> Zatoshis {
        unreachable!()
    }
}

/// A trait that provides a minimized view of a Orchard output suitable for use in fee and change
/// calculation.
pub trait OutputView {
    /// The value of the output being produced.
    fn value(&self) -> Zatoshis;
}

impl OutputView for Infallible {
    fn value(&self) -> Zatoshis {
        unreachable!()
    }
}

impl OutputView for Zatoshis {
    fn value(&self) -> Zatoshis {
        *self
    }
}
