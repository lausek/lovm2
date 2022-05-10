(def fib (n)
    (if (or (eq n 0) (eq n 1))
        (ret n)
        (ret (+ (fib (- n 1)) (fib (- n 2))))))

(def main ()
    (print (fib 3) " ")
    (print (fib 5) " ")
    (print (fib 8) " "))
