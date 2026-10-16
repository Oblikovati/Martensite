//! Output seeking preserves state while enabling fixed-width header backpatches.
use martensite_tkernel::storage::{IoStream, state};
use std::io::SeekFrom;
#[path = "output_seek_fakes.rs"]
mod output_seek_fakes;
use output_seek_fakes::OutputSeekFixture;

#[test]
fn output_seek_backpatches_a_fixed_width_header_without_moving_the_payload() {
    let mut fixture = OutputSeekFixture::new(b"", true, None);
    fixture.stream.write(&[0; 8]).write(b"payload");
    assert_eq!(fixture.stream.tellp(), 15);
    fixture.stream.seekp(0).write(&15_u64.to_le_bytes());
    assert_eq!(fixture.stream.tellp(), 8);
    assert_eq!(
        fixture.observations.borrow().cursor.get_ref(),
        b"\x0f\0\0\0\0\0\0\0payload"
    );
    assert!(fixture.stream.good());
    fixture.stream.close();
    assert!(!fixture.stream.is_open());
}

#[test]
fn output_seek_keeps_eof_only_and_moves_without_clearing_it() {
    let mut stream = IoStream::from_bytes(b"12".to_vec());
    let mut value = 0;
    assert!(stream.extract_int(&mut value));
    assert_eq!(stream.rdstate(), state::EOF);
    stream.seekp(0);
    assert_eq!((stream.rdstate(), stream.tellp()), (state::EOF, 0));
    stream.seekg(1);
    assert_eq!((stream.rdstate(), stream.tellp()), (0, 1));
}

#[test]
fn failed_output_states_refuse_seek_without_touching_the_device() {
    for (initial, expected) in [
        (state::FAIL, state::FAIL),
        (state::BAD, state::BAD | state::FAIL),
        (state::EOF | state::FAIL, state::EOF | state::FAIL),
    ] {
        let mut fixture = OutputSeekFixture::new(b"abcdef", true, None);
        fixture.stream.clear(initial);
        fixture.stream.seekp(1);
        assert_eq!(fixture.stream.rdstate(), expected);
        assert!(fixture.observations.borrow().requests.is_empty());
        assert_eq!(fixture.observations.borrow().cursor.position(), 0);
    }
}

#[test]
fn input_only_and_closed_streams_reject_output_positions() {
    let mut fixture = OutputSeekFixture::new(b"abcdef", false, None);
    fixture.stream.seekp(1);
    assert_eq!(fixture.stream.rdstate(), state::FAIL);
    assert!(fixture.observations.borrow().requests.is_empty());
    fixture.stream.clear(0);
    fixture.stream.seekg(1);
    assert_eq!((fixture.stream.rdstate(), fixture.stream.tellg()), (0, 1));
    let mut closed = IoStream::default();
    closed.seekp(0);
    assert_eq!(closed.rdstate(), state::FAIL);
}

#[test]
fn negative_and_device_refused_positions_preserve_bytes_and_position() {
    let mut negative = OutputSeekFixture::new(b"abcdef", true, None);
    negative.stream.seekp(-1);
    assert_eq!(negative.stream.rdstate(), state::FAIL);
    assert!(negative.observations.borrow().requests.is_empty());
    let mut refused = OutputSeekFixture::new(b"abcdef", true, Some(SeekFrom::Start(1)));
    refused.stream.seekp(1);
    assert_eq!(refused.stream.rdstate(), state::FAIL);
    assert_eq!(refused.observations.borrow().requests, [SeekFrom::Start(1)]);
    assert_eq!(refused.observations.borrow().cursor.position(), 0);
    assert_eq!(refused.observations.borrow().cursor.get_ref(), b"abcdef");
}

#[test]
fn output_seek_synchronizes_read_ahead_and_preserves_input_count_and_formatting() {
    let mut fixture = OutputSeekFixture::new(b"abcdef", true, None);
    fixture.stream.read(&mut [0; 2]);
    fixture.stream.set_width(9);
    fixture.stream.set_precision(17);
    fixture.stream.seekp(4).write(b"!");
    assert_eq!(fixture.stream.gcount(), 2);
    assert_eq!(
        (fixture.stream.width(), fixture.stream.precision()),
        (9, 17)
    );
    assert_eq!(
        fixture.observations.borrow().requests,
        [SeekFrom::Current(-4), SeekFrom::Start(4)]
    );
    assert_eq!(fixture.observations.borrow().cursor.get_ref(), b"abcd!f");
}

#[test]
fn output_device_policy_controls_positions_beyond_existing_contents() {
    let mut growing = OutputSeekFixture::new(b"ab", true, None);
    growing.stream.seekp(4).write(b"!");
    assert!(growing.stream.good());
    assert_eq!(growing.observations.borrow().cursor.get_ref(), b"ab\0\0!");
    let mut bounded = OutputSeekFixture::new(b"ab", true, Some(SeekFrom::Start(4)));
    bounded.stream.seekp(4);
    assert_eq!(bounded.stream.rdstate(), state::FAIL);
    assert_eq!(bounded.observations.borrow().cursor.get_ref(), b"ab");
}

#[test]
fn refused_output_seek_keeps_the_previous_logical_read_position() {
    let mut fixture = OutputSeekFixture::new(b"abcdef", true, Some(SeekFrom::Start(4)));
    fixture.stream.read(&mut [0; 2]);
    fixture.stream.seekp(4);
    assert_eq!(
        (fixture.stream.rdstate(), fixture.stream.gcount()),
        (state::FAIL, 2)
    );
    fixture.stream.clear(0);
    let mut next = [0];
    fixture.stream.read(&mut next);
    assert_eq!(next, *b"c");
}
