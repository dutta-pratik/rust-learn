Rust doesn't have null, nul, undefined

Instead, we get a built-in enum called '**Option**'

Has two variants - 'Some' and 'None'

If you want to work with Option you have to use pattern matching(the 'if let' thing) or a match statement

Forces you to handle the case in which you have a value and the case in which you don't

enum Option{
	Some(value),
	None 
}

`Some` and `None` can also be used to indicate the presence or absence of some value in a struct field.
