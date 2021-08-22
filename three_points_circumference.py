import sympy
enum T
	A([Char])
	B([Char])
	C([Char])
enum Char
	Space ' '
	h 'h'
	1 '1'
	Letter 'k'
	
impl T
	to_str(x T) ([Char])
		
		match x
			A(b)
				return( Arr::append( [Char::Space] Char("<h1>") b Char("<\h1>") [Char::Space] ) )
			
let spaces [Char::Space]
let line Arr::append(spaces Char("\n"))
let lines [line]
