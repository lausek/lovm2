use super::*;

use std::ops::*;

/// Default optimizer
pub struct StandardOptimizer;

impl StandardOptimizer {
    pub fn new() -> Self {
        Self
    }
}

impl Optimizer for StandardOptimizer {
    fn postprocess(&mut self, code: &mut Vec<LirElement>) {
        let vp = ValidPath::scan(code);

        for off in (0..code.len()).rev() {
            if !vp.is_valid(off) {
                code.remove(off);
            }
        }
    }

    fn transform(&mut self, code: &mut Vec<LirElement>) {
        while let Some(last) = code.last() {
            let scan_back = self.scan_back_for(last);

            if scan_back == 0 {
                break;
            }

            let l = code.len().saturating_sub(scan_back);
            let view = &mut code[l..];

            match view {
                // if `Not` is in front of a conditional jump, change condition
                // and remove `Not`
                [LirElement::Operation(Operator::Operator1(Operator1::Not)), LirElement::Jump {
                    condition: Some(cond),
                    ..
                }] => {
                    *cond = !*cond;
                    view.swap(0, 1);
                    code.pop();
                }

                // if a constant is pushed before a conditional jump, change condition
                // and remove constant
                [LirElement::PushConstant { value }, LirElement::Jump {
                    condition: cond @ Some(_),
                    ..
                }] => {
                    let bval: bool = value.clone().into();
                    if bval == cond.unwrap() {
                        // always jump
                        *cond = None;
                        view.swap(0, 1);
                        code.pop();
                    } else {
                        // never jump
                        code.pop();
                        code.pop();
                    }
                }

                /*
                // a jump should not target the next instruction
                [LirElement::Jump {
                    label: jlabel,
                    ..
                }, LirElement::Label(tlabel)] if jlabel == tlabel => {
                }
                */
                // if two constants were pushed before an operation, remove all three instructions
                // and push the computed value instead
                [LirElement::PushConstant { value: left }, LirElement::PushConstant { value: right }, LirElement::Operation(Operator::Operator2(op))] =>
                {
                    let (left, right) = (left.clone(), right.clone());
                    let newval = match op {
                        Operator2::Add => left.add(right),
                        Operator2::Sub => left.sub(right),
                        Operator2::Mul => left.mul(right),
                        Operator2::Div => left.div(right),
                        Operator2::Pow => left.pow(right),
                        Operator2::Rem => left.rem(right),
                        Operator2::And => left.bitand(right),
                        Operator2::Or => left.bitor(right),
                        Operator2::XOr => left.bitxor(right),
                        Operator2::Equal => Ok(left.eq(&right).into()),
                        Operator2::NotEqual => Ok(left.ne(&right).into()),
                        Operator2::GreaterEqual => Ok(left.ge(&right).into()),
                        Operator2::GreaterThan => Ok(left.gt(&right).into()),
                        Operator2::LessEqual => Ok(left.le(&right).into()),
                        Operator2::LessThan => Ok(left.lt(&right).into()),
                    }
                    .unwrap();

                    view[0] = LirElement::PushConstant { value: newval };

                    code.pop();
                    code.pop();
                }
                _ => break,
            }
        }
    }

    fn scan_back_for(&self, lir_element: &LirElement) -> usize {
        match lir_element {
            LirElement::Jump { .. } => 2,
            LirElement::Operation(Operator::Operator2(_)) => 3,
            //LirElement::Label(_) => 2,
            _ => 0,
        }
    }
}
