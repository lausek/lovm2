(def fac (x) 
    (if (not (eq x 0))
        (ret (* x (fac (- x 1))))
        (ret 1)))

(def main ()
    (print (fac 1) " ")
    (print (fac 2) " ")
    (print (fac 3) " "))
