use std::prelude::v1::*;
use csv::ByteRecord;
use csv::StringRecord;


use std::str;



   pub fn b(s: &str) -> &[u8] { s.as_bytes() }

 
   pub fn record_1() {
        let mut rec = ByteRecord::new();
        rec.push_field(b"foo");

        assert_eq!(rec.len(), 1);
        assert_eq!(rec.get(0), Some(b("foo")));
        assert_eq!(rec.get(1), None);
        assert_eq!(rec.get(2), None);
    }

 
   pub fn record_2() {
        let mut rec = ByteRecord::new();
        rec.push_field(b"foo");
        rec.push_field(b"quux");

        assert_eq!(rec.len(), 2);
        assert_eq!(rec.get(0), Some(b("foo")));
        assert_eq!(rec.get(1), Some(b("quux")));
        assert_eq!(rec.get(2), None);
        assert_eq!(rec.get(3), None);
    }

 
   pub fn empty_record() {
        let rec = ByteRecord::new();

        assert_eq!(rec.len(), 0);
        assert_eq!(rec.get(0), None);
        assert_eq!(rec.get(1), None);
    }

 
   pub fn empty_field_1() {
        let mut rec = ByteRecord::new();
        rec.push_field(b"");

        assert_eq!(rec.len(), 1);
        assert_eq!(rec.get(0), Some(b("")));
        assert_eq!(rec.get(1), None);
        assert_eq!(rec.get(2), None);
    }

 
   pub fn empty_field_2() {
        let mut rec = ByteRecord::new();
        rec.push_field(b"");
        rec.push_field(b"");

        assert_eq!(rec.len(), 2);
        assert_eq!(rec.get(0), Some(b("")));
        assert_eq!(rec.get(1), Some(b("")));
        assert_eq!(rec.get(2), None);
        assert_eq!(rec.get(3), None);
    }

 
   pub fn empty_surround_1() {
        let mut rec = ByteRecord::new();
        rec.push_field(b"foo");
        rec.push_field(b"");
        rec.push_field(b"quux");

        assert_eq!(rec.len(), 3);
        assert_eq!(rec.get(0), Some(b("foo")));
        assert_eq!(rec.get(1), Some(b("")));
        assert_eq!(rec.get(2), Some(b("quux")));
        assert_eq!(rec.get(3), None);
        assert_eq!(rec.get(4), None);
    }

 
   pub fn empty_surround_2() {
        let mut rec = ByteRecord::new();
        rec.push_field(b"foo");
        rec.push_field(b"");
        rec.push_field(b"quux");
        rec.push_field(b"");

        assert_eq!(rec.len(), 4);
        assert_eq!(rec.get(0), Some(b("foo")));
        assert_eq!(rec.get(1), Some(b("")));
        assert_eq!(rec.get(2), Some(b("quux")));
        assert_eq!(rec.get(3), Some(b("")));
        assert_eq!(rec.get(4), None);
        assert_eq!(rec.get(5), None);
    }

 
   pub fn utf8_error_1() {
        let mut rec = ByteRecord::new();
        rec.push_field(b"foo");
        rec.push_field(b"b\xFFar");

        let err = StringRecord::from_byte_record(rec).unwrap_err();
        assert_eq!(err.utf8_error().field(), 1);
        assert_eq!(err.utf8_error().valid_up_to(), 1);
    }

 
   pub fn utf8_error_2() {
        let mut rec = ByteRecord::new();
        rec.push_field(b"\xFF");

        let err = StringRecord::from_byte_record(rec).unwrap_err();
        assert_eq!(err.utf8_error().field(), 0);
        assert_eq!(err.utf8_error().valid_up_to(), 0);
    }

 
   pub fn utf8_error_3() {
        let mut rec = ByteRecord::new();
        rec.push_field(b"a\xFF");

        let err = StringRecord::from_byte_record(rec).unwrap_err();
        assert_eq!(err.utf8_error().field(), 0);
        assert_eq!(err.utf8_error().valid_up_to(), 1);
    }

 
   pub fn utf8_error_4() {
        let mut rec = ByteRecord::new();
        rec.push_field(b"a");
        rec.push_field(b"b");
        rec.push_field(b"c");
        rec.push_field(b"d");
        rec.push_field(b"xyz\xFF");

        let err = StringRecord::from_byte_record(rec).unwrap_err();
        assert_eq!(err.utf8_error().field(), 4);
        assert_eq!(err.utf8_error().valid_up_to(), 3);
    }

 
   pub fn utf8_error_5() {
        let mut rec = ByteRecord::new();
        rec.push_field(b"a");
        rec.push_field(b"b");
        rec.push_field(b"c");
        rec.push_field(b"d");
        rec.push_field(b"\xFFxyz");

        let err = StringRecord::from_byte_record(rec).unwrap_err();
        assert_eq!(err.utf8_error().field(), 4);
        assert_eq!(err.utf8_error().valid_up_to(), 0);
    }

    // This tests a tricky case where a single field on its own isn't valid
    // UTF-8, but the concatenation of all fields is.
 
   pub fn utf8_error_6() {
        let mut rec = ByteRecord::new();
        rec.push_field(b"a\xc9");
        rec.push_field(b"\x91b");

        let err = StringRecord::from_byte_record(rec).unwrap_err();
        assert_eq!(err.utf8_error().field(), 0);
        assert_eq!(err.utf8_error().valid_up_to(), 1);
    }

    // This tests that we can always clear a `ByteRecord` and get a guaranteed
    // successful conversion to UTF-8. This permits reusing the allocation.
 
   pub fn utf8_clear_ok() {
        let mut rec = ByteRecord::new();
        rec.push_field(b"\xFF");
        assert!(StringRecord::from_byte_record(rec).is_err());

        let mut rec = ByteRecord::new();
        rec.push_field(b"\xFF");
        rec.clear();
        assert!(StringRecord::from_byte_record(rec).is_ok());
    }

 
   pub fn iter() {
        let data = vec!["foo", "bar", "baz", "quux", "wat"];
        let rec = ByteRecord::from(&*data);
        let got: Vec<&str> = rec.iter()
            .map(|x| ::std::str::from_utf8(x).unwrap())
            .collect();
        assert_eq!(data, got);
    }

 
   pub fn iter_reverse() {
        let mut data = vec!["foo", "bar", "baz", "quux", "wat"];
        let rec = ByteRecord::from(&*data);
        let got: Vec<&str> = rec.iter()
            .rev()
            .map(|x| ::std::str::from_utf8(x).unwrap())
            .collect();
        data.reverse();
        assert_eq!(data, got);
    }

 
   pub fn iter_forward_and_reverse() {
        let data = vec!["foo", "bar", "baz", "quux", "wat"];
        let rec = ByteRecord::from(data);
        let mut it = rec.iter();

        assert_eq!(it.next_back(), Some(b("wat")));
        assert_eq!(it.next(), Some(b("foo")));
        assert_eq!(it.next(), Some(b("bar")));
        assert_eq!(it.next_back(), Some(b("quux")));
        assert_eq!(it.next(), Some(b("baz")));
        assert_eq!(it.next_back(), None);
        assert_eq!(it.next(), None);
    }