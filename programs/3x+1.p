printl "enter number:"
getnum x

label main
	goto do_iteration
	print x
	cmp x 1 end
	print " "
goto main

label do_iteration
	copy x i
	mod i 2
	cmp i 0 even
	ncmp i 0 odd
ret

label even
	div x 2
ret

label odd
	mul x 3
	change x 1
ret

label end
	printl ""
