use std::fs::File;
use std::io::Write;

fn say_hello(out: &mut dyn Write) -> std::io::Result<()> {
    out.write_all(b"Hello world\n")?;
    out.flush()
}

fn say_hello2<W>(out: &mut W) -> std::io::Result<()>
    where W: Write
{
    out.write_all(b"Hello world\n")?;
    out.flush()
}

fn main() {
    let mut local_file = File::create("hello.txt").unwrap();
    say_hello(&mut local_file).unwrap();

    let mut bytes = vec![];
    say_hello2(&mut bytes).unwrap();
    assert_eq!(bytes, b"Hello world\n");
}
