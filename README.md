# Problemathic - A quantum secure encryption algorithm based on number systems

## Idea

I got the idea for this project while learning Rust. I was coding a number system convertor and while trying to add as many bases as possible I realised that by including letters I can "convert" words into numbers. After adding special characters, I was able to type whole sentences and convert them into unintelligible strings. By repeating that procces multiple times I got a string which can only be decrtypted and read if you knew exactly from which bases to convert and in what order.

## Usage

### Rust

You are going to need rustup to compile the code into an executable. You can follow Rust's own guide by visiting their official [website](rustup.rs).

### Compiling

You can compile the code by simply running `cargo build` in the terminal while anywhere in the `/problemathic` folder. This should create the `/problemathic/target/debug/problemathic` executable along with other build files.

### Running

You can either run the aforemention executable, or you can run the program with `cargo run ...` while in the `/problemathic` folder.

### Arguements

The program consists of two commands, `problemathic encrypt ...` and `problemathic decrypt ...`. Both commands take the same arguements. You have to provide the program with an input file, an output file and a password file.

The input file should be a .txt file with the information you want to encrypt/decrypt. It must consist of only the characters from the CHARSET variable. If you didn't tweak the program, you are allowed to use all the printable ASCII characters.

The output file should be any .txt file, the contents don't matter since they are going to get deleted.

The password file should be a .txt file with the bases you want to use to encrypt your data. The bases must be lower than the lenght of the CHARSET variable, which is by default 95, and the bases must be seperated by spaces only. This is an example of a valid password file.

`
94 54 68 72 93
`

### Customizing the CHARSET

If you want to use some characters which are not originally in the charset you can add them manually. Just make sure they're UTF-8 encoded and that your terminal supports them. After doing this make sure to run `cargo build` again.
