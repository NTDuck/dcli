use layout::namespace_mod;

namespace_mod!(pub ids);
namespace_mod!(pub tasks);

pub use ids::*;
pub use tasks::*;

// Test code, delete later!
#[derive(ddd::ValueObject)]
struct MyObject {
    s: String,
    i: usize,
    b: bool,
}
