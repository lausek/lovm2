use lovm2_extend::prelude::*;
use lovm2_std_data::{Buffer, Response, Request};

use std::collections::HashMap;

#[lovm2_function]
fn new_request() -> Request { todo!() }

#[lovm2_function]
fn set_header(req: &mut Request, key: String, val: String) { todo!() }

#[lovm2_function]
fn set_method(req: &mut Request, method: String) { todo!() }

#[lovm2_function]
fn exec(req: &Request) -> Response { todo!() }

#[lovm2_function]
fn get_status(res: &Response) -> i64 { todo!() }

#[lovm2_function]
fn get_body_as_string(res: &Response) -> Lovm2Result<String> { todo!() }

#[lovm2_function]
fn get_body_as_buffer(res: &Response) -> Lovm2Result<Buffer> { todo!() }

lovm2_module_init!(net);
