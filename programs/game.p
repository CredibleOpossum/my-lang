printl "Welcome to to the escape room game"
printl ""

label main_room
    printl " ___________________________"
    printl "/                         2 \"
    printl "|               ._______.   |"
    printl "|               |       |   |"
    printl "|               |       |   |"
    printl "|               |_______|   |"
    printl "|                           |"
    printl "|                           |"
    printl "|        3        ______ 1  |"
    printl "|      <---       |    |    /"
    printl "\_________________|____|___/"
    getnum option
    cmp option 1 table
    cmp option 2 painting
    cmp option 3 room2
goto main_room

label table
    cmp has_key 0 table_with_key
    cmp has_key 1 table_without_key

label table_with_key
    printl " ___________________________"
    printl "/                           \"
    printl "|          __           2   |"
    printl "|         /  \_________     |"
    printl "|        | () ______| |     |"
    printl "|         \__/      |_|     |"
    printl "|                           |"
    printl "|                           |"
    printl "|                  1        |"
    printl "|                 |         |"
    printl "|                 ▼         /"
    printl "\__________________________/"
    getnum option
    cmp option 1 main_room
    cmp option 2 get_key
goto table_with_key

label table_without_key
    printl " ___________________________"
    printl "/                           \"
    printl "|                           |"
    printl "|                           |"
    printl "|                           |"
    printl "|                           |"
    printl "|                           |"
    printl "|                           |"
    printl "|                  1        |"
    printl "|                 |         |"
    printl "|                 ▼         /"
    printl "\__________________________/"
    getnum option
    cmp option 1 main_room

label get_key
    set has_key 1
goto table_without_key

label painting
    printl " ___________________________"
    printl "/                           \"
    printl "|                           |"
    printl "|                           |"
    printl "|     4x - 2                |"
    printl "|     ------  =  1125       |"
    printl "|       10                  |"
    printl "|                           |"
    printl "|                  1        |"
    printl "|                 |         |"
    printl "|                 ▼         /"
    printl "\__________________________/"
    getnum option
    cmp option 1 main_room
goto painting

label room2
    printl " ___________________________"
    printl "/                           \"
    printl "|                           |"
    printl "|                           |"
    printl "|                           |"
    printl "|                 2         |"
    printl "|        |-------|          |"
    printl "|        |       |      1   |"
    printl "|        | o     |   -->    |"
    printl "|        |  '''' |          /"
    printl "\________|_______|_________/"
    getnum option
    cmp option 1 main_room
    cmp option 2 door
goto room2

label door
printl " ___________________________"
printl "/                           \"
printl "|                           |"
printl "|      |                    |"
printl "|     | |                   |"
printl "|     | |   _ _ _           |"
printl "|      |    |2|5|0|5|       |"
printl "|           - - -     1     |"
printl "|                    |      |"
printl "|                    ▼      /"
printl "\__________________________/"
getnum option
cmp option 1 room2
cmp option 2812 win
goto door

label win
    cmp has_key 0 door
    printl "you win!"

