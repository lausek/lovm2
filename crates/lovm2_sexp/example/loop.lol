(def main ()
    (let i 0)
    (loop
        (let i (+ i 1))
        (print i)
        (if (eq i 10)
            (break))))
