use std::io::{Result, Write};


/// An output sink, escaping `<`, `>`, and `&` characters passed thereto.
///
/// # Examples
///
/// ```
/// # use bloguen::util::XmlEscapeWrite;
/// # use std::io::Write;
/// let mut out = vec![];
/// XmlEscapeWrite(&mut out).write_all("hewwo > Бenlo".as_bytes()).unwrap();
/// assert_eq!(out, "hewwo &gt; Бenlo".as_bytes());
/// ```
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct XmlEscapeWrite<Wr1: Write>(pub Wr1);

impl<Wr1: Write> Write for XmlEscapeWrite<Wr1> {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        let mut written = 0;
        let mut last_end = 0;

        for i in 0..buf.len() {
            let repl = match buf[i] {
                b'<' => &b"&lt;"[..],
                b'>' => &b"&gt;"[..],
                b'&' => &b"&amp;"[..],
                _ => continue,
            };

            written += self.0.write(&buf[last_end..i])?;

            self.0.write(repl)?;
            written += 1;

            last_end = i + 1;
        }

        written += self.0.write(&buf[last_end..])?;

        Ok(written)
    }

    #[inline]
    fn flush(&mut self) -> Result<()> {
        self.0.flush()
    }
}
