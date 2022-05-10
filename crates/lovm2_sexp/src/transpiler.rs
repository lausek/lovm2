use ess::Sexp;
use lovm2::prelude::*;

macro_rules! take_as {
    ($expr:expr, $ty:path) => {
        match $expr {
            $ty(inner, _loc) => Ok(inner),
            _ => Err(format!("expected {}, got {:?}", stringify!($ty), $expr)),
        }
    };
}

pub struct Transpiler;

impl Transpiler {
    pub fn new() -> Self {
        Self
    }

    fn maps_to_operator(&self, name: &str) -> Option<LV2Operator2> {
        match name {
            "+" => Some(LV2Operator2::Add),
            "-" => Some(LV2Operator2::Sub),
            "*" => Some(LV2Operator2::Mul),
            "/" => Some(LV2Operator2::Div),
            "%" => Some(LV2Operator2::Rem),
            "eq" => Some(LV2Operator2::Eq),
            "ne" => Some(LV2Operator2::Ne),
            "ge" => Some(LV2Operator2::Ge),
            "gt" => Some(LV2Operator2::Gt),
            "le" => Some(LV2Operator2::Le),
            "lt" => Some(LV2Operator2::Lt),
            "and" => Some(LV2Operator2::And),
            "or" => Some(LV2Operator2::Or),
            _ => None,
        }
    }

    pub fn build_from_path<T>(&mut self, path: T) -> Result<LV2Module, String>
    where
        T: AsRef<std::path::Path>,
    {
        let source = std::fs::read_to_string(path.as_ref()).map_err(|e| format!("{}", e))?;
        // derive the module name and location from filepath
        let meta: LV2ModuleMeta = path.as_ref().into();
        self.build(meta, source)
    }

    pub fn build<T>(&mut self, meta: LV2ModuleMeta, source: T) -> Result<LV2Module, String>
    where
        T: AsRef<str>,
    {
        let mut builder = LV2ModuleBuilder::with_meta(meta);

        if !source.as_ref().is_empty() {
            let (sexprs, err) = ess::parser::parse(source.as_ref());
            if let Some(err) = err {
                return Err(format!("{:?}", err));
            }

            // build hir
            self.translate(&mut builder, &sexprs)?;
        }

        let module = builder.build().map_err(|e| format!("{:?}", e))?;

        Ok(module)
    }

    fn translate(&mut self, builder: &mut LV2ModuleBuilder, sexprs: &[Sexp]) -> Result<(), String> {
        for sexpr in sexprs.iter() {
            match sexpr {
                Sexp::List(list, _) => {
                    if let Sexp::Sym(name, _) = &list[0] {
                        match name.as_ref() {
                            "def" => self.translate_define(builder, &list)?,
                            "import" => self.translate_toplevel_import(builder, &list)?,
                            _ => return Err(format!("unexpected keyword `{}`", name)),
                        }
                    } else {
                        unimplemented!()
                    }
                }
                _ => panic!("not expected at top-level: {:?}", sexpr),
            }
        }

        Ok(())
    }

    fn translate_toplevel_import(
        &self,
        module: &mut LV2ModuleBuilder,
        list: &[Sexp],
    ) -> Result<(), String> {
        let name = take_as!(&list[1], Sexp::Sym)?;
        module.entry().import(name.as_ref());
        Ok(())
    }

    fn translate_define(&self, module: &mut LV2ModuleBuilder, list: &[Sexp]) -> Result<(), String> {
        let name = take_as!(&list[1], Sexp::Sym)?;
        let arguments = take_as!(&list[2], Sexp::List)?
            .iter()
            .map(|item| take_as!(item, Sexp::Sym).unwrap())
            .map(|item| LV2Variable::from(item.to_string()))
            .collect();

        // TODO: avoid index errors here
        let body = &list[3..];
        let hir = module.add_with_args(name.to_string(), arguments);

        for stmt in body.iter() {
            self.translate_macro(hir.block_mut(), &stmt)?;
        }

        Ok(())
    }

    fn translate_macro(&self, block: &mut LV2Block, ast: &Sexp) -> Result<(), String> {
        let list = take_as!(&ast, Sexp::List)?;
        let name = take_as!(&list[0], Sexp::Sym)?;
        // TODO: avoid index errors here
        let rest = &list[1..];

        match name.as_ref() {
            "break" => block.break_repeat(),
            "continue" => block.continue_repeat(),
            "do" => {
                for step in rest.iter() {
                    self.translate_macro(block, step)?;
                }
            }
            "foreach" => {
                let head = take_as!(&rest[0], Sexp::List)?;
                assert_eq!(2, head.len());

                let collection = self.translate_expr(&head[0])?;
                let item = take_as!(&head[1], Sexp::Sym)?;
                let item = item.to_string();

                let repeat = block.repeat_iterating(LV2Expr::from(collection).to_iter(), item);
                for step in rest[1..].iter() {
                    self.translate_macro(repeat.block_mut(), step)?;
                }
            }
            "if" => {
                let condition = self.translate_expr(&rest[0])?;
                let branch = block.branch();
                self.translate_macro(branch.add_condition(condition), &rest[1])?;
                if rest.len() == 3 {
                    self.translate_macro(branch.default_condition(), &rest[2])?;
                }
            }
            "import" => {
                let name = take_as!(&rest[0], Sexp::Sym)?;
                block.import(name.as_ref());
            }
            "import-global" => {
                let name = take_as!(&rest[0], Sexp::Sym)?;
                block.import_from(name.as_ref());
            }
            "let" => {
                assert_eq!(2, rest.len());
                let name = take_as!(&rest[0], Sexp::Sym)?;
                let name = LV2Variable::from(name.to_string());
                let val = self.translate_expr(&rest[1])?;
                block.local(&name);
                block.assign(name, val);
            }
            "loop" => {
                let repeat = block.repeat();
                for item in rest.iter() {
                    self.translate_macro(repeat.block_mut(), item)?;
                }
            }
            "ret" => {
                assert!(rest.len() <= 1);
                if rest.is_empty() {
                    block.return_nil()
                } else {
                    let val = self.translate_expr(&rest[0])?;
                    block.return_value(val)
                }
            }
            _ => {
                let args = self.to_expr_vec(rest)?;
                block.step(LV2Call::with_args(name.as_ref(), args));
            }
        }

        Ok(())
    }

    fn translate_expr(&self, sexp: &Sexp) -> Result<LV2Expr, String> {
        match sexp {
            Sexp::Sym(name, _) => Ok(LV2Expr::from(LV2Variable::from(name.to_string()))),
            Sexp::Str(s, _) => Ok(LV2Expr::from(s.as_ref())),
            Sexp::Char(c, _) => Ok(LV2Expr::from(format!("{}", c))),
            Sexp::Int(n, _) => Ok(LV2Expr::from(*n)),
            Sexp::Float(n, _) => Ok(LV2Expr::from(*n)),
            Sexp::List(list, _) => self.translate_expr_macro(list),
        }
    }

    fn translate_expr_macro(&self, list: &[Sexp]) -> Result<LV2Expr, String> {
        let name = take_as!(&list[0], Sexp::Sym)?;

        if let Some(op) = self.maps_to_operator(name.as_ref()) {
            let mut rest = self.to_expr_vec(&list[1..])?;
            let mut first = rest.remove(0);

            // automatically turn first operand into float to
            // avoid information loss on integer division
            if op == LV2Operator2::Div {
                first = LV2Expr::to_float(first);
            }

            Ok(first.expand_op(op, rest))
        } else {
            match name.as_ref() {
                "bool" | "float" | "int" | "str" => {
                    assert_eq!(2, list.len());
                    let expr = self.translate_expr(&list[1])?;

                    Ok(match name.as_ref() {
                        "bool" => LV2Expr::to_bool(expr),
                        "float" => LV2Expr::to_float(expr),
                        "int" => LV2Expr::to_integer(expr),
                        "str" => LV2Expr::to_str(expr),
                        _ => unreachable!(),
                    }
                    .into())
                }
                "dict" => {
                    let mut dict = LV2Expr::dict();

                    for tuple in &list[1..] {
                        match tuple {
                            Sexp::List(tuple, _) => {
                                assert_eq!(2, tuple.len());
                                let mut kv = self.to_expr_vec(tuple)?;
                                let (key, value) = (kv.remove(0), kv.remove(0));
                                dict = dict.set(key, value);
                            }
                            _ => return Err("expected key-value tuple".to_string()),
                        }
                    }

                    Ok(dict.into())
                }
                "list" => {
                    let mut ls = LV2Expr::list();
                    let rest = self.to_expr_vec(&list[1..])?;

                    for item in rest {
                        ls = ls.append(item);
                    }

                    Ok(ls.into())
                }
                "not" => {
                    let rest = self.to_expr_vec(&list[1..])?;
                    assert_eq!(1, rest.len());
                    Ok(LV2Expr::not(rest[0].clone()))
                }
                "range" => {
                    let rest = self.to_expr_vec(&list[1..])?;
                    let (from, to): (LV2Expr, LV2Expr) = match rest.as_slice() {
                        [first] => (LV2Value::Nil.into(), first.clone()),
                        [first, second] => (first.clone(), second.clone()),
                        _ => unimplemented!(),
                    };

                    Ok(LV2Expr::iter_ranged(from, to).into())
                }
                _ => {
                    let rest = self.to_expr_vec(&list[1..])?;
                    let call = LV2Call::with_args(name.as_ref(), rest);
                    Ok(LV2Expr::from(call))
                }
            }
        }
    }

    fn to_expr_vec(&self, list: &[Sexp]) -> Result<Vec<LV2Expr>, String> {
        let mut rest = vec![];
        for item in list.iter() {
            rest.push(self.translate_expr(item)?);
        }
        Ok(rest)
    }
}
