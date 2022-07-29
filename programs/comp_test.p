label main
	rand x
	rand y
	abs x
	abs y
	mod x 100
	mod y 100

	printl ""
	print x
	print " "
	print y
	printl ""

	cmp x y are_the_same
	ncmp x y not_the_same
goto main


label are_the_same
	printl "are the same!"
end

label not_the_same
	printl "are not the same!"
ret
