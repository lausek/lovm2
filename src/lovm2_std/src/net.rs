use std::collections::HashMap;

use super::*;

#[lovm2_function]
fn new_response() -> Response {
    Response {
        status: 200,
        headers: HashMap::new(),
        body: vec![],
    }
}

#[lovm2_function]
fn new_request(url: String) -> Request {
    Request {
        url,
        headers: HashMap::new(),
        method: Method::Get,
        body: vec![],
    }
}

#[lovm2_function]
fn get_url(req: &mut Request) -> String {
    req.url.clone()
}

#[lovm2_function]
fn set_header(req: &mut Request, key: String, val: String) {
    req.headers.insert(key, val);
}

#[lovm2_function]
fn set_body(req: &mut Request, mut body: LV2Value) -> LV2Result<bool> {
    body.unref_inplace()?;

    if let LV2Value::Str(body) = body {
        req.body = body.as_bytes().to_vec();
        return Ok(true);
    }

    if let LV2Value::Any(any) = body {
        if let Some(buf) = (*any).borrow_mut().0.downcast_mut::<Buffer>() {
            req.body = buf.inner.clone();
        }
        return Ok(true);
    }

    err_not_supported()
}

#[lovm2_function]
fn set_method(req: &mut Request, method: String) -> LV2Result<bool> {
    use std::convert::TryFrom;
    req.method = Method::try_from(method)?;
    Ok(true)
}

#[lovm2_function]
fn serve(vm: &mut LV2Vm, host: String, callback: String) -> LV2Result<()> {
    let server = tiny_http::Server::http(host).unwrap();

    for mut request in server.incoming_requests() {
        let mut parsed_request = Request {
            url: request.url().to_string(),
            headers: HashMap::new(),
            method: match request.method() {
                tiny_http::Method::Delete => Method::Delete,
                tiny_http::Method::Get => Method::Get,
                tiny_http::Method::Post => Method::Post,
                tiny_http::Method::Put => Method::Put,
                method => err_method_not_supported(&format!("{:?}", method))?,
            },
            body: vec![],
        };

        for tiny_http::Header { field, value } in request.headers() {
            parsed_request
                .headers
                .insert(field.to_string(), value.to_string());
        }

        request
            .as_reader()
            .read_to_end(&mut parsed_request.body)
            .unwrap();

        let (status_code, content_type, body) = {
            let response = vm.call(callback.clone(), &[parsed_request.into()])?;
            let status_code = response.get(&0.into())?.as_integer_inner()?;
            let content_type = response.get(&1.into())?.as_str_inner()?;
            let body = response.get(&2.into())?.as_str_inner()?;
            (status_code, content_type, body)
        };

        let response = tiny_http::Response::from_string(body)
            .with_status_code(status_code as u16)
            .with_header(
                tiny_http::Header::from_bytes(&b"Content-Type"[..], content_type.into_bytes())
                    .unwrap(),
            );

        request.respond(response).unwrap();
    }

    Ok(())
}

#[lovm2_function]
fn exec(req: &Request) -> LV2Result<Response> {
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
    let mut rbuf = req.body.as_slice();

    {
        let mut transfer = easy.transfer();

        // write request data
        transfer
            .read_function(|into| {
                use std::io::Read;
                Ok(rbuf.read(into).unwrap())
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

    Ok(Response {
        status,
        body,
        headers: HashMap::new(),
    })
}

#[lovm2_function]
fn get_status(res: &Response) -> i64 {
    res.status
}

#[lovm2_function]
fn get_body_as_string(res: &Response) -> LV2Result<String> {
    String::from_utf8(res.body.clone()).or_else(|_| err_from_string("response is not valid utf-8"))
}

#[lovm2_function]
fn get_body_as_buffer(res: &Response) -> LV2Result<Buffer> {
    Ok(Buffer {
        inner: res.body.clone(),
        ..Buffer::default()
    })
}
