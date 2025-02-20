use axiom::macros::layout::*;

#[cfg(feature = "boundaries")]
namespace!(pub boundaries);

#[cfg(feature = "interactors")]
namespace!(pub interactors);

#[cfg(feature = "gateways")]
namespace!(pub gateways);

namespace!(pub utils);
