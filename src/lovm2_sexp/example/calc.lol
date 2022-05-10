(def main ()
    (let i 0)
    (loop
        (let i (+ i 1))
        (if (gt i 100)
            (break))
        (if (and
                (ne (% i 3) 0)
                (ne (% i 5) 0))
            (continue))
        (print i)
        (print " ")))
