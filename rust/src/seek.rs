use std::io::{Read, Seek};
use byteorder::{ReadBytesExt, LE};
use crate::{parse_record, records, McapError, McapResult};

pub struct StreamReader<T> {
    data: T
}

impl<T: Read> StreamReader<T> {
    pub fn new(data: T) -> Self {
        StreamReader {
            data
        }
    }
}

impl <T: Read> Iterator for StreamReader<T> {
    type Item = McapResult<records::Record<'static>>;

    fn next(&mut self) -> Option<Self::Item> {
        read_record_start(self).map(|(op, len)| {
            read_record_from_reader(self, op, len)
        })
    }
}

fn read_record_start<T: Read>(reader: &mut StreamReader<T>) -> Option<(u8, u64)> {
    let op = reader.data.read_u8();
    let len = reader.data.read_u64::<LE>();

    match (op, len) {
        (Ok(op), Ok(len)) => Some((op, len)),
        _ => None,
    }
}

fn read_record_from_reader<'a, T: Read>(reader: &'a mut StreamReader<T>, op: u8, len: u64) -> McapResult<records::Record<'static>> {
    let mut body = vec![0; len as usize];
    reader.data.read_exact(&mut body).map_err(|_| McapError::UnexpectedEof)?;

    let record = parse_record(op, &body);

    record.map(|r| r.into_owned())
}

#[cfg(test)]
mod test {
    use crate::seek::StreamReader;

    #[test]
    fn test_read() {
        let bytes: Vec<u8> = vec![0, 1, 2, 3, 4, 5];
        // let cursor = Cursor::new(bytes.as_slice());
        let cursor = bytes.as_slice();

       let reader = StreamReader::new(cursor);
    }
}