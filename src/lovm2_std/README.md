# lovm2_std

Shared objects with common functionality.

## Functions

```
absolute(path: String) -> Lovm2Result<String>
acos(val: f64) -> f64
all(collection: &Value) -> Lovm2Result<bool>
any(collection: &Value) -> Lovm2Result<bool>
append(collection: &mut Value, value: Value) -> Lovm2Result<()>
argn(vm: &mut Vm) -> Lovm2Result<i64>
asin(val: f64) -> f64
atan(val: f64) -> f64
atan2(val: f64, other: f64) -> f64
basename(path: String) -> Option<String>
call(vm: &mut Vm, function_name: String, mut args: Value) -> Lovm2Result<Value>
captures(regex: &Regex, text: String) -> Option<Value>
ceil(val: f64) -> f64
chr(n: i64) -> Lovm2Result<String>
clamp(val: f64, min: f64, max: f64) -> f64
contains(haystack: &Value, needle: Value) -> Lovm2Result<bool>
cos(val: f64) -> f64
create_file(path: String) -> Lovm2Result<File>
decode(json: String) -> Lovm2Result<Value>
deep_clone(val: Value) -> Value
delete(collection: &mut Value, key: Value) -> Lovm2Result<bool>
e() -> f64
encode(val: Value) -> Lovm2Result<String>
exec(req: &Request) -> Lovm2Result<Response>
exists(path: String) -> bool
filter(vm: &mut Vm, collection: &Value, func_name: String) -> Lovm2Result<Value>
floor(val: f64) -> f64
format(vm: &mut Vm) -> Lovm2Result<String>
get(collection: &Value, key: Value) -> Lovm2Result<Value>
get_body_as_buffer(res: &Response) -> Lovm2Result<Buffer>
get_body_as_string(res: &Response) -> Lovm2Result<String>
get_status(res: &Response) -> i64
get_url(req: &mut Request) -> String
has_data(buffer: &mut Buffer) -> bool
index_of(base: String, pat: String) -> Option<i64>
input	
is_dir(path: String) -> bool
is_match(regex: &Regex, text: String) -> bool
join(base: &Value, sep: String) -> Lovm2Result<String>
len(val: &Value) -> Lovm2Result<i64>
list_dir(path: String) -> Lovm2Result<Value>
log(val: f64, base: f64) -> f64
map(vm: &mut Vm, collection: &Value, func_name: String) -> Lovm2Result<Value>
mkdir(path: String) -> bool
new_buffer() -> Lovm2Result<Buffer>
new_regex(pat: String) -> Lovm2Result<Regex>
new_request(url: String) -> Request
new_response() -> Response
open_file(path: String) -> Lovm2Result<File>
ord(c: String) -> Lovm2Result<i64>
parent(path: String) -> Option<String>
pi() -> f64
pop_vstack(vm: &mut Vm) -> Lovm2Result<Value>
print	
push_vstack(vm: &mut Vm, val: Value) -> ()
read_all(file: &mut File) -> Lovm2Result<String>
read_line(buffer: &mut Buffer) -> Lovm2Result<String>
readn(buffer: &mut Buffer, n: i64) -> Lovm2Result<String>
rename(from: String, to: String) -> Lovm2Result<bool>
replace(base: String, pat: String, repl: String) -> String
rmdir(path: String) -> bool
round(val: f64) -> f64
serve(vm: &mut Vm, host: String, callback: String) -> Lovm2Result<()>
set(collection: &mut Value, key: Value, val: Value) -> Lovm2Result<bool>
set_body(req: &mut Request, mut body: Value) -> Lovm2Result<bool>
set_header(req: &mut Request, key: String, val: String) -> ()
set_method(req: &mut Request, method: String) -> Lovm2Result<bool>
sin(val: f64) -> f64
sort(collection: &Value) -> Lovm2Result<Value>
split(base: String, sep: String) -> Lovm2Result<Value>
sqrt(val: f64) -> f64
tan(val: f64) -> f64
to_lower(base: String) -> String
to_upper(base: String) -> String
trim(base: String) -> String
unlink(path: String) -> bool
write_all(file: &mut File, content: String) -> Lovm2Result<bool>
writes(buffer: &mut Buffer, text: String) -> Lovm2Result<bool>
```
