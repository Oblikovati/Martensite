//! `XGetProperties`' lines for one label's named data, in the order of categories.
use crate::reports::g17;
use martensite_tkernel::tcollection::ExtendedString;
use martensite_tklcaf::tdata_std::{TDataStdNamedData, TDataStdNamedMap};

fn utf8(text: &ExtendedString) -> String {
    let ascii = martensite_tkernel::tcollection::AsciiString::from(text);
    String::from_utf8_lossy(ascii.as_bytes()).into_owned()
}

/// The lines of a label's named data. Example: `append_named_data(&data, &mut text)`.
pub fn append_named_data(data: &TDataStdNamedData, text: &mut String) {
    if data.has_integers() {
        for (key, value) in data
            .get_integers_container()
            .get()
            .expect("a map")
            .map()
            .items()
        {
            *text += &format!("{} : {value}\n", utf8(key));
        }
    }
    if data.has_reals() {
        for (key, value) in data
            .get_reals_container()
            .get()
            .expect("a map")
            .map()
            .items()
        {
            *text += &format!("{} : {}\n", utf8(key), g17(*value));
        }
    }
    if data.has_strings() {
        for (key, value) in data
            .get_strings_container()
            .get()
            .expect("a map")
            .map()
            .items()
        {
            *text += &format!("{} : {}\n", utf8(key), utf8(value));
        }
    }
    if data.has_bytes() {
        for (key, value) in data
            .get_bytes_container()
            .get()
            .expect("a map")
            .map()
            .items()
        {
            *text += &format!("{} : {value}\n", utf8(key));
        }
    }
    if data.has_arrays_of_integers() {
        for (key, value) in data
            .get_arrays_of_integers_container()
            .get()
            .expect("a map")
            .map()
            .items()
        {
            let words: String = value
                .get()
                .expect("an array")
                .array1()
                .iter()
                .map(|word| format!(" {word}"))
                .collect();
            *text += &format!("{} : {words}\n", utf8(key));
        }
    }
    if data.has_arrays_of_reals() {
        for (key, value) in data
            .get_arrays_of_reals_container()
            .get()
            .expect("a map")
            .map()
            .items()
        {
            let words: String = value
                .get()
                .expect("an array")
                .array1()
                .iter()
                .map(|word| format!(" {}", g17(*word)))
                .collect();
            *text += &format!("{} : {words}\n", utf8(key));
        }
    }
}
