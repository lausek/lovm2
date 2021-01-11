use lovm2_extend::prelude::*;

use std::collections::HashMap;

use super::*;

#[lovm2_function]
fn new_request(url: String) -> Request {
    Request {
        url,
        headers: HashMap::new(),
        method: Method::Get,
        body: None,
    }
}

#[lovm2_function]
fn set_header(req: &mut Request, key: String, val: String) {
    req.headers.insert(key, val);
}

#[lovm2_function]
fn set_body(req: &mut Request, mut body: Value) -> Lovm2Result<bool> {
    body.unref_inplace()?;

    if let Value::Str(body) = body {
        req.body = Some(body.as_bytes().to_vec());
        return Ok(true);
    }

    if let Value::Any(any) = body {
        if let Some(buf) = (*any).borrow_mut().0.downcast_mut::<Buffer>() {
            req.body = Some(buf.inner.clone());
        }
        return Ok(true);
    }

    err_not_supported()
}

#[lovm2_function]
fn set_method(req: &mut Request, method: String) -> Lovm2Result<bool> {
    use std::convert::TryFrom;
    req.method = Method::try_from(method)?;
    Ok(true)
}

#[lovm2_function]
fn exec(req: &Request) -> Lovm2Result<Response> {
    use curl::easy::Easy;

    let mut easy = Easy::new();

    easy.url(req.url.as_ref()).unwrap();

    match req.method {
        Method::Get => easy.get(true).unwrap(),
        Method::Post => easy.post(true).unwrap(),
        Method::Put => easy.put(true).unwrap(),
        Method::Delete => unimplemented!(),
    }

    let mut body = vec![];
    let mut rbuf = req.body.as_deref();

    {
        let mut transfer = easy.transfer();

        // write request data
        transfer
            .read_function(|into| {
                if let Some(ref mut rbuf) = rbuf {
                    use std::io::Read;
                    Ok(rbuf.read(into).unwrap())
                } else {
                    Ok(0)
                }
            })
            .unwrap();

        // write response data
        transfer
            .write_function(|data| {
                body.extend_from_slice(data);
                Ok(data.len())
            })
            .unwrap();

        transfer.perform().unwrap();
    }

    let status = easy.response_code().unwrap() as i64;

    Ok(Response { status, body })
}

#[lovm2_function]
fn get_status(res: &Response) -> i64 {
    res.status
}

#[lovm2_function]
fn get_body_as_string(res: &Response) -> Lovm2Result<String> {
    String::from_utf8(res.body.clone()).or_else(|_| err_from_string("response is not valid utf-8"))
}

#[lovm2_function]
fn get_body_as_buffer(res: &Response) -> Lovm2Result<Buffer> {
    Ok(Buffer {
        inner: res.body.clone(),
        ..Buffer::default()
    })
}
