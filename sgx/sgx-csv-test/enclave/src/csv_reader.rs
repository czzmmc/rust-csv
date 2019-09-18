use std::prelude::v1::*;
use csv::ByteRecord;
use csv::ErrorKind;
use csv::StringRecord;
use csv::{ReaderBuilder, Position};
use std::io;

pub fn b(s: &str) -> &[u8] { s.as_bytes() }
pub fn s(b: &[u8]) -> &str { ::std::str::from_utf8(b).unwrap() }

pub fn newpos(byte: u64, line: u64, record: u64) -> Position {
        let mut p = Position::new();
        p.set_byte(byte).set_line(line).set_record(record);
        p
    }


pub fn read_byte_record() {
        let data = b("foo,\"b,ar\",baz\nabc,mno,xyz");
        let mut rdr = ReaderBuilder::new()
            .has_headers(false)
            .from_reader(data);
        let mut rec = ByteRecord::new();

        assert!(rdr.read_byte_record(&mut rec).unwrap());
        assert_eq!(3, rec.len());
        assert_eq!("foo", s(&rec[0]));
        assert_eq!("b,ar", s(&rec[1]));
        assert_eq!("baz", s(&rec[2]));

        assert!(rdr.read_byte_record(&mut rec).unwrap());
        assert_eq!(3, rec.len());
        assert_eq!("abc", s(&rec[0]));
        assert_eq!("mno", s(&rec[1]));
        assert_eq!("xyz", s(&rec[2]));

        assert!(!rdr.read_byte_record(&mut rec).unwrap());
    }


pub fn read_record_unequal_fails() {
        let data = b("foo\nbar,baz");
        let mut rdr = ReaderBuilder::new()
            .has_headers(false)
            .from_reader(data);
        let mut rec = ByteRecord::new();

        assert!(rdr.read_byte_record(&mut rec).unwrap());
        assert_eq!(1, rec.len());
        assert_eq!("foo", s(&rec[0]));

        match rdr.read_byte_record(&mut rec) {
            Err(err) => {
                match *err.kind() {
                    ErrorKind::UnequalLengths {
                        expected_len: 1,
                        ref pos,
                        len: 2,
                    } => {
                        assert_eq!(pos, &Some(newpos(4, 2, 1)));
                    }
                    ref wrong => panic!("match failed, got {:?}", wrong),
                }
            }
            wrong => panic!("match failed, got {:?}", wrong),
        }
    }


pub fn read_record_unequal_ok() {
        let data = b("foo\nbar,baz");
        let mut rdr = ReaderBuilder::new()
            .has_headers(false)
            .flexible(true)
            .from_reader(data);
        let mut rec = ByteRecord::new();

        assert!(rdr.read_byte_record(&mut rec).unwrap());
        assert_eq!(1, rec.len());
        assert_eq!("foo", s(&rec[0]));

        assert!(rdr.read_byte_record(&mut rec).unwrap());
        assert_eq!(2, rec.len());
        assert_eq!("bar", s(&rec[0]));
        assert_eq!("baz", s(&rec[1]));

        assert!(!rdr.read_byte_record(&mut rec).unwrap());
    }

    // This tests that even if we get a CSV error, we can continue reading
    // if we want.

pub fn read_record_unequal_continue() {
        let data = b("foo\nbar,baz\nquux");
        let mut rdr = ReaderBuilder::new()
            .has_headers(false)
            .from_reader(data);
        let mut rec = ByteRecord::new();

        assert!(rdr.read_byte_record(&mut rec).unwrap());
        assert_eq!(1, rec.len());
        assert_eq!("foo", s(&rec[0]));

        match rdr.read_byte_record(&mut rec) {
            Err(err) => {
                match err.kind() {
                    &ErrorKind::UnequalLengths {
                        expected_len: 1,
                        ref pos,
                        len: 2,
                    } => {
                        assert_eq!(pos, &Some(newpos(4, 2, 1)));
                    }
                    wrong => panic!("match failed, got {:?}", wrong),
                }
            }
            wrong => panic!("match failed, got {:?}", wrong),
        }

        assert!(rdr.read_byte_record(&mut rec).unwrap());
        assert_eq!(1, rec.len());
        assert_eq!("quux", s(&rec[0]));

        assert!(!rdr.read_byte_record(&mut rec).unwrap());
    }


pub fn read_record_headers() {
        let data = b("foo,bar,baz\na,b,c\nd,e,f");
        let mut rdr = ReaderBuilder::new().has_headers(true).from_reader(data);
        let mut rec = StringRecord::new();

        assert!(rdr.read_record(&mut rec).unwrap());
        assert_eq!(3, rec.len());
        assert_eq!("a", &rec[0]);

        assert!(rdr.read_record(&mut rec).unwrap());
        assert_eq!(3, rec.len());
        assert_eq!("d", &rec[0]);

        assert!(!rdr.read_record(&mut rec).unwrap());

        {
            let headers = rdr.byte_headers().unwrap();
            assert_eq!(3, headers.len());
            assert_eq!(b"foo", &headers[0]);
            assert_eq!(b"bar", &headers[1]);
            assert_eq!(b"baz", &headers[2]);
        }
        {
            let headers = rdr.headers().unwrap();
            assert_eq!(3, headers.len());
            assert_eq!("foo", &headers[0]);
            assert_eq!("bar", &headers[1]);
            assert_eq!("baz", &headers[2]);
        }
    }


pub fn read_record_headers_invalid_utf8() {
        let data = &b"foo,b\xFFar,baz\na,b,c\nd,e,f"[..];
        let mut rdr = ReaderBuilder::new().has_headers(true).from_reader(data);
        let mut rec = StringRecord::new();

        assert!(rdr.read_record(&mut rec).unwrap());
        assert_eq!(3, rec.len());
        assert_eq!("a", &rec[0]);

        assert!(rdr.read_record(&mut rec).unwrap());
        assert_eq!(3, rec.len());
        assert_eq!("d", &rec[0]);

        assert!(!rdr.read_record(&mut rec).unwrap());

        // Check that we can read the headers as raw bytes, but that
        // if we read them as strings, we get an appropriate UTF-8 error.
        {
            let headers = rdr.byte_headers().unwrap();
            assert_eq!(3, headers.len());
            assert_eq!(b"foo", &headers[0]);
            assert_eq!(b"b\xFFar", &headers[1]);
            assert_eq!(b"baz", &headers[2]);
        }
        match *rdr.headers().unwrap_err().kind() {
            ErrorKind::Utf8 { pos: Some(ref pos), ref err } => {
                assert_eq!(pos, &newpos(0, 1, 0));
                assert_eq!(err.field(), 1);
                assert_eq!(err.valid_up_to(), 1);
            }
            ref err => panic!("match failed, got {:?}", err),
        }
    }


pub fn read_record_no_headers_before() {
        let data = b("foo,bar,baz\na,b,c\nd,e,f");
        let mut rdr = ReaderBuilder::new()
            .has_headers(false)
            .from_reader(data);
        let mut rec = StringRecord::new();

        {
            let headers = rdr.headers().unwrap();
            assert_eq!(3, headers.len());
            assert_eq!("foo", &headers[0]);
            assert_eq!("bar", &headers[1]);
            assert_eq!("baz", &headers[2]);
        }

        assert!(rdr.read_record(&mut rec).unwrap());
        assert_eq!(3, rec.len());
        assert_eq!("foo", &rec[0]);

        assert!(rdr.read_record(&mut rec).unwrap());
        assert_eq!(3, rec.len());
        assert_eq!("a", &rec[0]);

        assert!(rdr.read_record(&mut rec).unwrap());
        assert_eq!(3, rec.len());
        assert_eq!("d", &rec[0]);

        assert!(!rdr.read_record(&mut rec).unwrap());
    }


pub fn read_record_no_headers_after() {
        let data = b("foo,bar,baz\na,b,c\nd,e,f");
        let mut rdr = ReaderBuilder::new()
            .has_headers(false)
            .from_reader(data);
        let mut rec = StringRecord::new();

        assert!(rdr.read_record(&mut rec).unwrap());
        assert_eq!(3, rec.len());
        assert_eq!("foo", &rec[0]);

        assert!(rdr.read_record(&mut rec).unwrap());
        assert_eq!(3, rec.len());
        assert_eq!("a", &rec[0]);

        assert!(rdr.read_record(&mut rec).unwrap());
        assert_eq!(3, rec.len());
        assert_eq!("d", &rec[0]);

        assert!(!rdr.read_record(&mut rec).unwrap());

        let headers = rdr.headers().unwrap();
        assert_eq!(3, headers.len());
        assert_eq!("foo", &headers[0]);
        assert_eq!("bar", &headers[1]);
        assert_eq!("baz", &headers[2]);
    }


pub fn seek() {
        let data = b("foo,bar,baz\na,b,c\nd,e,f\ng,h,i");
        let mut rdr = ReaderBuilder::new()
            .from_reader(io::Cursor::new(data));
        rdr.seek(newpos(18, 3, 2)).unwrap();

        let mut rec = StringRecord::new();

        assert_eq!(18, rdr.position().byte());
        assert!(rdr.read_record(&mut rec).unwrap());
        assert_eq!(3, rec.len());
        assert_eq!("d", &rec[0]);

        assert_eq!(24, rdr.position().byte());
        assert_eq!(4, rdr.position().line());
        assert_eq!(3, rdr.position().record());
        assert!(rdr.read_record(&mut rec).unwrap());
        assert_eq!(3, rec.len());
        assert_eq!("g", &rec[0]);

        assert!(!rdr.read_record(&mut rec).unwrap());
    }

    // Test that we can read headers after seeking even if the headers weren't
    // explicit read before seeking.

pub fn seek_headers_after() {
        let data = b("foo,bar,baz\na,b,c\nd,e,f\ng,h,i");
        let mut rdr = ReaderBuilder::new()
            .from_reader(io::Cursor::new(data));
        rdr.seek(newpos(18, 3, 2)).unwrap();
        assert_eq!(rdr.headers().unwrap(), vec!["foo", "bar", "baz"]);
    }

    // Test that we can read headers after seeking if the headers were read
    // before seeking.

pub fn seek_headers_before_after() {
        let data = b("foo,bar,baz\na,b,c\nd,e,f\ng,h,i");
        let mut rdr = ReaderBuilder::new()
            .from_reader(io::Cursor::new(data));
        let headers = rdr.headers().unwrap().clone();
        rdr.seek(newpos(18, 3, 2)).unwrap();
        assert_eq!(&headers, rdr.headers().unwrap());
    }

    // Test that even if we didn't read headers before seeking, if we seek to
    // the current byte offset, then no seeking is done and therefore we can
    // still read headers after seeking.

pub fn seek_headers_no_actual_seek() {
        let data = b("foo,bar,baz\na,b,c\nd,e,f\ng,h,i");
        let mut rdr = ReaderBuilder::new()
            .from_reader(io::Cursor::new(data));
        rdr.seek(Position::new()).unwrap();
        assert_eq!("foo", &rdr.headers().unwrap()[0]);
    }

    // Test that position info is reported correctly in absence of headers.

pub fn positions_no_headers() {
        let mut rdr = ReaderBuilder::new()
            .has_headers(false)
            .from_reader("a,b,c\nx,y,z".as_bytes())
            .into_records();

        let pos = rdr.next().unwrap().unwrap().position().unwrap().clone();
        assert_eq!(pos.byte(), 0);
        assert_eq!(pos.line(), 1);
        assert_eq!(pos.record(), 0);

        let pos = rdr.next().unwrap().unwrap().position().unwrap().clone();
        assert_eq!(pos.byte(), 6);
        assert_eq!(pos.line(), 2);
        assert_eq!(pos.record(), 1);
    }

    // Test that position info is reported correctly with headers.

pub fn positions_headers() {
        let mut rdr = ReaderBuilder::new()
            .has_headers(true)
            .from_reader("a,b,c\nx,y,z".as_bytes())
            .into_records();

        let pos = rdr.next().unwrap().unwrap().position().unwrap().clone();
        assert_eq!(pos.byte(), 6);
        assert_eq!(pos.line(), 2);
        assert_eq!(pos.record(), 1);
    }

    // Test that reading headers on empty data yields an empty record.

pub fn headers_on_empty_data() {
        let mut rdr = ReaderBuilder::new().from_reader("".as_bytes());
        let r = rdr.byte_headers().unwrap();
        assert_eq!(r.len(), 0);
    }

    // Test that reading the first record on empty data works.

pub fn no_headers_on_empty_data() {
        let mut rdr = ReaderBuilder::new()
            .has_headers(false)
            .from_reader("".as_bytes());
        assert_eq!(rdr.records().count(), 0);
    }

    // Test that reading the first record on empty data works, even if
    // we've tried to read headers before hand.

pub fn no_headers_on_empty_data_after_headers() {
        let mut rdr = ReaderBuilder::new()
            .has_headers(false)
            .from_reader("".as_bytes());
        assert_eq!(rdr.headers().unwrap().len(), 0);
        assert_eq!(rdr.records().count(), 0);
    }