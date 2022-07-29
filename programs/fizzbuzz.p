set x 1

label main
    cmp x 100 end

    change x 1

    copy x a
    mod a 15
    cmp a 0 div_by_15

    copy x a
    mod a 3
    cmp a 0 div_by_5 

    copy x a
    mod a 5
    cmp a 0 div_by_3 

    printl x
goto main

label div_by_15
    printl "FizzBuzz"
goto main

label div_by_3
    printl "Fizz"
goto main

label div_by_5
    printl "Buzz"
goto main

label end
