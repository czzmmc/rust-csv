
use std::prelude::v1::*;
use csv::ByteRecord;
use csv::ErrorKind;
use csv::StringRecord;
use csv::{Writer, WriterBuilder};

    pub fn wtr_as_string(wtr: Writer<Vec<u8>>) -> String {
        String::from_utf8(wtr.into_inner().unwrap()).unwrap()
    }

    
    pub fn one_record() {
        let mut wtr = WriterBuilder::new().from_writer(vec![]);
        wtr.write_record(&["a", "b", "c"]).unwrap();

        assert_eq!(wtr_as_string(wtr), "a,b,c\n");
    }

    
    pub fn one_string_record() {
        let mut wtr = WriterBuilder::new().from_writer(vec![]);
        wtr.write_record(&StringRecord::from(vec!["a", "b", "c"])).unwrap();

        assert_eq!(wtr_as_string(wtr), "a,b,c\n");
    }

    
    pub fn one_byte_record() {
        let mut wtr = WriterBuilder::new().from_writer(vec![]);
        wtr.write_record(&ByteRecord::from(vec!["a", "b", "c"])).unwrap();

        assert_eq!(wtr_as_string(wtr), "a,b,c\n");
    }

    
    pub fn raw_one_byte_record() {
        let mut wtr = WriterBuilder::new().from_writer(vec![]);
        wtr.write_byte_record(&ByteRecord::from(vec!["a", "b", "c"])).unwrap();

        assert_eq!(wtr_as_string(wtr), "a,b,c\n");
    }

    
    pub fn one_empty_record() {
        let mut wtr = WriterBuilder::new().from_writer(vec![]);
        wtr.write_record(&[""]).unwrap();

        assert_eq!(wtr_as_string(wtr), "\"\"\n");
    }

    
    pub fn raw_one_empty_record() {
        let mut wtr = WriterBuilder::new().from_writer(vec![]);
        wtr.write_byte_record(&ByteRecord::from(vec![""])).unwrap();

        assert_eq!(wtr_as_string(wtr), "\"\"\n");
    }

    
    pub fn two_empty_records() {
        let mut wtr = WriterBuilder::new().from_writer(vec![]);
        wtr.write_record(&[""]).unwrap();
        wtr.write_record(&[""]).unwrap();

        assert_eq!(wtr_as_string(wtr), "\"\"\n\"\"\n");
    }

    
    pub fn raw_two_empty_records() {
        let mut wtr = WriterBuilder::new().from_writer(vec![]);
        wtr.write_byte_record(&ByteRecord::from(vec![""])).unwrap();
        wtr.write_byte_record(&ByteRecord::from(vec![""])).unwrap();

        assert_eq!(wtr_as_string(wtr), "\"\"\n\"\"\n");
    }

    
    pub fn unequal_records_bad() {
        let mut wtr = WriterBuilder::new().from_writer(vec![]);
        wtr.write_record(&ByteRecord::from(vec!["a", "b", "c"])).unwrap();
        let err = wtr.write_record(&ByteRecord::from(vec!["a"])).unwrap_err();
        match *err.kind() {
            ErrorKind::UnequalLengths { ref pos, expected_len, len } => {
                assert!(pos.is_none());
                assert_eq!(expected_len, 3);
                assert_eq!(len, 1);
            }
            ref x => {
                panic!("expected UnequalLengths error, but got '{:?}'", x);
            }
        }
    }

    
    pub fn raw_unequal_records_bad() {
        let mut wtr = WriterBuilder::new().from_writer(vec![]);
        wtr.write_byte_record(&ByteRecord::from(vec!["a", "b", "c"])).unwrap();
        let err = wtr.write_byte_record(
            &ByteRecord::from(vec!["a"])).unwrap_err();
        match *err.kind() {
            ErrorKind::UnequalLengths { ref pos, expected_len, len } => {
                assert!(pos.is_none());
                assert_eq!(expected_len, 3);
                assert_eq!(len, 1);
            }
            ref x => {
                panic!("expected UnequalLengths error, but got '{:?}'", x);
            }
        }
    }

    
    pub fn unequal_records_ok() {
        let mut wtr = WriterBuilder::new().flexible(true).from_writer(vec![]);
        wtr.write_record(&ByteRecord::from(vec!["a", "b", "c"])).unwrap();
        wtr.write_record(&ByteRecord::from(vec!["a"])).unwrap();
        assert_eq!(wtr_as_string(wtr), "a,b,c\na\n");
    }

    
    pub fn raw_unequal_records_ok() {
        let mut wtr = WriterBuilder::new().flexible(true).from_writer(vec![]);
        wtr.write_byte_record(&ByteRecord::from(vec!["a", "b", "c"])).unwrap();
        wtr.write_byte_record(&ByteRecord::from(vec!["a"])).unwrap();
        assert_eq!(wtr_as_string(wtr), "a,b,c\na\n");
    }

    
    pub fn serialize_with_headers() {
        #[derive(Serialize)]
        struct Row {
            foo: i32,
            bar: f64,
            baz: bool,
        }

        let mut wtr = WriterBuilder::new().from_writer(vec![]);
        wtr.serialize(Row { foo: 42, bar: 42.5, baz: true }).unwrap();
        assert_eq!(wtr_as_string(wtr), "foo,bar,baz\n42,42.5,true\n");
    }

    
    pub fn serialize_no_headers() {
        #[derive(Serialize)]
        struct Row {
            foo: i32,
            bar: f64,
            baz: bool,
        }

        let mut wtr = WriterBuilder::new()
            .has_headers(false)
            .from_writer(vec![]);
        wtr.serialize(Row { foo: 42, bar: 42.5, baz: true }).unwrap();
        assert_eq!(wtr_as_string(wtr), "42,42.5,true\n");
    }

    
    pub fn serialize_tuple() {
        let mut wtr = WriterBuilder::new().from_writer(vec![]);
        wtr.serialize((true, 1.3, "hi")).unwrap();
        assert_eq!(wtr_as_string(wtr), "true,1.3,hi\n");
    }