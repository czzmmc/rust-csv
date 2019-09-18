// Copyright (C) 2017-2018 Baidu, Inc. All Rights Reserved.
//
// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions
// are met:
//
//  * Redistributions of source code must retain the above copyright
//    notice, this list of conditions and the following disclaimer.
//  * Redistributions in binary form must reproduce the above copyright
//    notice, this list of conditions and the following disclaimer in
//    the documentation and/or other materials provided with the
//    distribution.
//  * Neither the name of Baidu, Inc., nor the names of its
//    contributors may be used to endorse or promote products derived
//    from this software without specific prior written permission.
//
// THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
// "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
// LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
// A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
// OWNER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
// SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT
// LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
// DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
// THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
// (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
// OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.

#![crate_name = "helloworldsampleenclave"]
#![crate_type = "staticlib"]

#![cfg_attr(not(target_env = "sgx"), no_std)]
#![cfg_attr(target_env = "sgx", feature(rustc_private))]

extern crate sgx_types;
#[cfg(not(target_env = "sgx"))]
#[macro_use]
extern crate sgx_tstd as std;
#[macro_use]
extern crate serde_derive;

extern crate sgx_tunittest;

use sgx_types::*;
use std::string::String;
use std::vec::Vec;
use std::io::{self, Write};
use std::slice;
use sgx_tunittest::*;

extern crate csv;

extern crate serde;
mod csv_writer;
mod csv_reader;
mod byte_record;


#[no_mangle]
pub extern "C" fn say_something(some_string: *const u8, some_len: usize) -> sgx_status_t {

    let str_slice = unsafe { slice::from_raw_parts(some_string, some_len) };
    let _ = io::stdout().write(str_slice);

    // A sample &'static string
    let rust_raw_string = "This is a in-Enclave ";
    // An array
    let word:[u8;4] = [82, 117, 115, 116];
    // An vector
    let word_vec:Vec<u8> = vec![32, 115, 116, 114, 105, 110, 103, 33];

    // Construct a string from &'static string
    let mut hello_string = String::from(rust_raw_string);

    // Iterate on word array
    for c in word.iter() {
        hello_string.push(*c as char);
    }

    // Rust style convertion
    hello_string += String::from_utf8(word_vec).expect("Invalid UTF-8")
                                               .as_str();

    // Ocall to normal world for output
    println!("{}", &hello_string);
    rsgx_unit_tests!(
csv_writer::one_record,
csv_writer::one_string_record,
csv_writer::one_byte_record,
csv_writer::raw_one_byte_record,
csv_writer::one_empty_record,
csv_writer::raw_one_empty_record,
csv_writer::two_empty_records,
csv_writer::raw_two_empty_records,
csv_writer::unequal_records_bad,
csv_writer::raw_unequal_records_bad,
csv_writer::unequal_records_ok,
csv_writer::raw_unequal_records_ok,
csv_writer::serialize_with_headers,
csv_writer::serialize_no_headers,
csv_writer::serialize_tuple,
csv_reader::read_byte_record,
csv_reader::read_record_unequal_fails,
csv_reader::read_record_unequal_ok,
csv_reader::read_record_unequal_continue,
csv_reader::read_record_headers,
csv_reader::read_record_headers_invalid_utf8,
csv_reader::read_record_no_headers_before,
csv_reader::read_record_no_headers_after,
csv_reader::seek,
csv_reader::seek_headers_after,
csv_reader::seek_headers_before_after,
csv_reader::seek_headers_no_actual_seek,
csv_reader::positions_no_headers,
csv_reader::positions_headers,
csv_reader::headers_on_empty_data,
csv_reader::no_headers_on_empty_data,
csv_reader::no_headers_on_empty_data_after_headers,
byte_record::record_1,
byte_record::record_2,
byte_record::empty_record,
byte_record::empty_field_1,
byte_record::empty_field_2,
byte_record::empty_surround_1,
byte_record::empty_surround_2,
byte_record::utf8_error_1,
byte_record::utf8_error_2,
byte_record::utf8_error_3,
byte_record::utf8_error_4,
byte_record::utf8_error_5,
byte_record::utf8_error_6,
byte_record::utf8_clear_ok,
byte_record::iter,
byte_record::iter_reverse,
byte_record::iter_forward_and_reverse,

// filetime_test::set_file_times_test,
// filetime_test::set_single_time_test,
// filetime_test::set_symlink_file_times_test,
// filetime_test::set_symlink_file_times_test,
              );

    sgx_status_t::SGX_SUCCESS
}
