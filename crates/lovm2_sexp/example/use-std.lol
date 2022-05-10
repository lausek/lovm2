(import regex)

(def main ()
    (import-global collection)
    (import-global string)

    (print "specify your email: ")
    (let mail (input))
    (let re (regex-new-regex "(\S+)@(\S+)"))
    (if (not (regex-is-match re mail))
        (do
            (print "ERROR: not an email")
            (ret)))
    (let groups (regex-captures re mail))
    (let name (to-upper (get groups 1)))
    (let host-parts (split (get groups 2) "."))
    (insert host-parts 1 "com")
    (let host (join host-parts "."))
    (print "name:" name "host:" host))
