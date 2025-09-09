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

---

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

`super` is a keyword to target parent

eg. from catalog if i want to use media, then in catalog file we can use code like `use` `super::media::Media`

The above code will get the Media struct from media.rs file by getting it from module referrenced in `mod.rs` file

In `main.rs` if we want to import any module, we can use `mod content`, this will import everything written in `mod.rs` file. And to use any thing we can `use` keyword

---

**Result** enum, it has Ok(Value) and Err(Error)

Error is an Object in Rust



---

**Match** variable is used to evaluate value

match function(a, b){

    //code or if we are using Option/Result enum we can evaultate/receive value here

}


---

**Tuple**

`type x = (u8, u8, u8)`

here in tuple(tuple is **()**), it is like struct but we don't have lable, we as a developer need to reemeber what does every placeholder means. on the above eg, u8 is an datatype

we can send empty tuple () whereever we don't have the value to send.

Eg, in Result enum, we have **Ok()**, let's say if we don't have any value to send we can use send empty tuple like **Ok(())**. When we have to  receive value, we can use **Ok(..)** since we don't have any value for **Ok** so we use **..** as placeholder


---

**'a**


---

There are **three** area in **memory**
	**Stack** - Fast, but limited size(generally 2-8 MB)
	**Heap** - Slow, but can grow to store a lot of data
	**Data Segment/Rodata Segment/Static Segment(**these can be interchangebly used) - store literal values that we write into our code

Rodata full form is Read only data

**Common Pattern**
Stack stores metadata about a data structure
Heap stores the actual data
*Corner Case* - If a data structure owns another data structure, the child's metadata will be placed on the heap

---



**String**

**&String**

**&str -** call as String slice


---

**?** operator

we can use it when we can expect Result enum.

**?** will unwrap the **Ok** value and assign it to the binding available

In case of **Err**, **?** will unwrap the error value and directly return and stop code from going to next line

eg.
`let txt = fs::read_file('logs.txt')?;
println!("{}", txt);`

if it successfully reads the file, and if `read_file` function return `Ok('xyz')` from **Result** enum, it will store it in `txt` binding and print it in second line

But in case if we receive error while reading the file, the **Err** will unwrap and will directly return the value present in the `Err(Error:other("Fail to read"));` and will not go to next line to print `txt`
