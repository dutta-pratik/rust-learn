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


---------------------------


Every file and folder makes its own seperate module

Every time when we create a folder, it is requeired to create mod.rs file

/src
	/content
		media.rs
		catalog.rs
	main.rs

root module doesn't have access to media.rs directly. we need to have `mod.rs` file in content folder

In `main.rs` files we can use mod content to use media module or catalog

/src
	/content
		mod.rs
		media.rs
		catalog.rs
	main.rs

In `mod.rs` which we will built in content dir. we can write
`pub mod media;`
`pub mod catalog;`

we can't skip this hirerachy levels. Root module can't directly access this
