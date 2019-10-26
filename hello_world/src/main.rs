fn main() {
    // Comments

    // this is an example of a line comment
    // rugular comments which are ignored by the compiler

    // println!("Hello Rust!")

    /*
     * This is another type of comment, a block comment. In general,
     * line comments are the recommended comment style. But
     * block comments are extremely useful for temporarily disabling
     * chunks of code. /* Block comments can be /* nested, */ */
     * so it takes only a few keystrokes to comment out everything
     * in this main() function. /*/*/* Try it yourself! */*/*/
     */

    /*
    Note: The previous column of `*` was entirely for style. There's
    no actual need for it.
    */

    /*
    Note: The previous column of `*` was entirely for style. There's
    no actual need for it.
    */

    // You can manipulate expressions more easily with block comments
    // than with line comments. Try deleting the comment delimiters
    // to change the result:
    let x = 5 + /* 90 + */ 5;
    println!("Is `x` 10 or 100? x = {}", x); // Is `x` 10 or 100? x = 10

    println!("Hello, world!");

    //  format!: 格式化文本
    //  print!: 和format类似，但是文字内容会在控制台打印出来
    //  println!: 和print!类似，但是会从新的一行开始
    //  eprint!: 和format类似，但是文字从标准错误中打印
    //  eprintln!: 和eprint!类似，但是会从新的一行开始

    println!("{} days", 31);

    println!("{0}, this is {1}, {1}, this is {0}", "Rust", "lang");

    println!(
        "{subject}, {verb}, {object}",
        subject = "the quick brown fox",
        verb = "jumps over",
        object = "the lazy dog",
    );
    
    println!("{} of {:b} people know binary, the other half doesn't,", 1, 2);

    println!("{number:>width$}", 
        number = 1,
        width = 10,
    );

    println!("{number:>0width$}",
        number = 1,
        width = 10,
    );


}
