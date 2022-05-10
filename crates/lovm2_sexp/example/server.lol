(def main ()
    (serve "localhost:8080" "callback"))

(def get-starships ()
    (let req (new_request "https://swapi.dev/api/starships/"))
    (let res (exec req))
    (let body (get_body_as_string res))
    (ret (decode body)))

(def parse-get-parameters (url)
    (let args (dict ("passengers" 0)))

    (if (not (contains url "?"))
        (ret args))

    (let parts (split url "?"))
    (let params (get parts 1))

    (foreach ((split params "&") kv-pair)
        (let parts (split kv-pair "="))
        (let key (trim (get parts 0)))
        (let val (trim (get parts 1)))

        (if (eq val "")
            (continue))

        (if (eq key "passengers")
            (let val (int val)))

        (set args key val))

    (ret args))

(def callback (request)
    (let args (parse-get-parameters (get_url request)))
    (let min-passengers (get args "passengers"))

    (let starships (get-starships))
    (let html-table "<table><tr><th>Name</th><th>Passengers</th></tr>")

    (foreach ((get starships "results") starship)
        (let passengers (replace (get starship "passengers") "," ""))

        (if (eq passengers "n/a")
            (continue))

        (let passengers (int passengers))
        (if (lt passengers min-passengers)
            (continue))

        (let line (format "<tr><td>{}</td><td>{}</td></tr>" (get starship "name") passengers))
        (let html-table (+ html-table line)))

    (let html-table (+ html-table "</table>"))

    (let template (read_all (open_file "example/server-template.html")))
    (let page (format template html-table))

    (ret (list 200 "text/html" page)))
