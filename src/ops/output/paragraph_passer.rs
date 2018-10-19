use jetscii::ByteSubstring;
use std::io::{self, Write};


lazy_static! {
    static ref OPEN: ByteSubstring<'static> = ByteSubstring::new(b"<p");
    static ref CLOSE: ByteSubstring<'static> = ByteSubstring::new(b"</p>");
}

const CLOSE_LEN: usize = 4;


/// Cannot be templated on `W: Write` because it overflows the recursion limit when used in `format_output()`.
pub struct ParagraphPasser<'w> {
    out: &'w mut Write,
    paras_left: usize,
    depth: usize,
    has_ended: bool,
}

impl<'w> ParagraphPasser<'w> {
    pub fn new(into: &'w mut Write, count: usize) -> ParagraphPasser<'w> {
        ParagraphPasser {
            out: into,
            paras_left: count,
            depth: 0,
            has_ended: false,
        }
    }

    fn close_para(&mut self, buf: &mut &[u8], past_end_idx: usize) -> io::Result<()> {
        self.out.write_all(&buf[..past_end_idx])?;
        *buf = &buf[past_end_idx..];

        if self.depth == 1 {
            self.paras_left -= 1;
        }
        self.depth -= 1;

        Ok(())
    }
}

impl<'w> Write for ParagraphPasser<'w> {
    fn write(&mut self, mut buf: &[u8]) -> io::Result<usize> {
        let mut full_len = buf.len();

        if !self.has_ended {
            if buf.ends_with(b"</p") {
                full_len -= 3;
            } else if buf.ends_with(b"</") || buf.ends_with(b"<p") {
                full_len -= 2;
            } else if buf.ends_with(b"<") {
                full_len -= 1;
            }
            buf = &buf[..full_len];

            let mut open_idx = OPEN.find(buf);
            let mut close_idx = CLOSE.find(buf);

            while !self.has_ended && !buf.is_empty() {
                match (open_idx.take(), close_idx.take(), self.paras_left == 0) {
                    (Some(oi), Some(_), true) |
                    (Some(oi), None, true) => {
                        self.out.write_all(&buf[..oi])?;
                        self.has_ended = true;
                    }
                    (Some(oi), Some(ci), false) => {
                        if oi < ci {
                            self.depth += 1;
                        }

                        let past_end = ci + CLOSE_LEN;
                        self.close_para(&mut buf, past_end)?;

                        if oi < ci {
                            open_idx = OPEN.find(buf);
                        } else {
                            open_idx = Some(oi - past_end);
                        }
                        close_idx = CLOSE.find(buf);
                    }
                    (Some(_), None, false) => {
                        self.out.write_all(buf)?;
                        self.depth += 1;

                        buf = &buf[..0];
                    }
                    (None, Some(ci), _) => {
                        let past_end = ci + CLOSE_LEN;
                        self.close_para(&mut buf, past_end)?;

                        close_idx = CLOSE.find(buf);
                    }
                    (None, None, _) => {
                        self.out.write_all(buf)?;
                        buf = &buf[..0];
                    }
                }
            }
        }

        return Ok(full_len);
    }

    fn flush(&mut self) -> io::Result<()> {
        self.out.flush()
    }
}