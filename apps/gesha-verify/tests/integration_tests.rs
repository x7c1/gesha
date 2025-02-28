mod v3_0;

use ctor::ctor;
use gesha_core::trace;

#[ctor]
fn init() {
    trace::init();
}
