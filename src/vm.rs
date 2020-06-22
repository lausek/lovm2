use crate::frame::Frame;
use crate::value::RuValue;

struct Vm {
    lstack: Vec<Frame>,
    vstack: Vec<RuValue>,
}
