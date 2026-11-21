//! Collection cases; twins in `occt/collections.cpp`.
use super::{Case, Sample};
use martensite_tkernel::ncollection::{
    BaseAllocator, DataMap, DynamicArray, IndexedMap, List, Sequence,
};
use std::hint::black_box;
use std::time::Instant;

pub const CASES: &[Case] = &[
    Case {
        name: "datamap_bind",
        run: datamap_bind,
    },
    Case {
        name: "datamap_seek",
        run: datamap_seek,
    },
    Case {
        name: "indexedmap_add",
        run: indexedmap_add,
    },
    Case {
        name: "indexedmap_find",
        run: indexedmap_find,
    },
    Case {
        name: "sequence_append",
        run: sequence_append,
    },
    Case {
        name: "sequence_append_explicit_common",
        run: sequence_append_explicit_common,
    },
    Case {
        name: "sequence_edit_batch",
        run: sequence_edit_batch,
    },
    Case {
        name: "sequence_cursor_walk",
        run: sequence_cursor_walk,
    },
    Case {
        name: "sequence_iterate",
        run: sequence_iterate,
    },
    Case {
        name: "sequence_value",
        run: sequence_value,
    },
    Case {
        name: "dynarray_append",
        run: dynarray_append,
    },
    Case {
        name: "dynarray_value",
        run: dynarray_value,
    },
    Case {
        name: "list_append_iterate",
        run: list_append_iterate,
    },
    Case {
        name: "list_append_explicit_common",
        run: list_append_explicit_common,
    },
    Case {
        name: "list_edit_batch",
        run: list_edit_batch,
    },
    Case {
        name: "list_cursor_walk",
        run: list_cursor_walk,
    },
];

fn filled_datamap(n: usize) -> DataMap<i32, i32> {
    let mut map = DataMap::default();
    for i in 0..n as i32 {
        map.bind(i, 2 * i);
    }
    map
}

fn datamap_bind(n: usize) -> Sample {
    let start = Instant::now();
    let map = black_box(filled_datamap(n));
    Sample::since(start, n, map.len() as i64)
}

fn datamap_seek(n: usize) -> Sample {
    let map = filled_datamap(n);
    let start = Instant::now();
    let mut sum = 0i64;
    for i in 0..n as i32 {
        sum += map.seek(black_box(&i)).map_or(0, |value| i64::from(*value));
    }
    Sample::since(start, n, sum)
}

fn filled_indexedmap(n: usize) -> IndexedMap<i32> {
    let mut map = IndexedMap::default();
    for i in 0..n as i32 {
        map.add(i);
    }
    map
}

fn indexedmap_add(n: usize) -> Sample {
    let start = Instant::now();
    let map = black_box(filled_indexedmap(n));
    Sample::since(start, n, map.len() as i64)
}

fn indexedmap_find(n: usize) -> Sample {
    let map = filled_indexedmap(n);
    let start = Instant::now();
    let mut sum = 0i64;
    for i in 0..n as i32 {
        sum += map.find_index(black_box(&i)) as i64;
    }
    Sample::since(start, n, sum)
}

fn filled_sequence(n: usize) -> Sequence<f64> {
    let mut sequence = Sequence::default();
    for i in 0..n {
        sequence.append(i as f64);
    }
    sequence
}

fn sequence_append(n: usize) -> Sample {
    let start = Instant::now();
    let sequence = black_box(filled_sequence(n));
    Sample::since(start, n, sequence.len() as i64)
}

fn sequence_iterate(n: usize) -> Sample {
    let sequence = filled_sequence(n);
    let start = Instant::now();
    let sum: f64 = sequence.iter().sum();
    Sample::since(start, n, black_box(sum) as i64)
}

fn sequence_value(n: usize) -> Sample {
    let sequence = filled_sequence(n);
    let start = Instant::now();
    let mut sum = 0.0;
    for i in 1..=n {
        sum += sequence.value(black_box(i)).map_or(0.0, |value| *value);
    }
    Sample::since(start, n, sum as i64)
}

fn filled_dynarray(n: usize) -> DynamicArray<f64> {
    let mut array = DynamicArray::with_increment(256);
    for i in 0..n {
        array.append(i as f64);
    }
    array
}

fn dynarray_append(n: usize) -> Sample {
    let start = Instant::now();
    let array = black_box(filled_dynarray(n));
    Sample::since(start, n, array.len() as i64)
}

fn dynarray_value(n: usize) -> Sample {
    let array = filled_dynarray(n);
    let start = Instant::now();
    let mut sum = 0.0;
    for i in 0..n {
        sum += array.value(black_box(i)).map_or(0.0, |value| *value);
    }
    Sample::since(start, n, sum as i64)
}

fn list_append_iterate(n: usize) -> Sample {
    let start = Instant::now();
    let mut list = List::default();
    for i in 0..n as i32 {
        list.append(i);
    }
    let sum: i64 = list.iter().map(|value| i64::from(*value)).sum();
    Sample::since(start, n, black_box(sum))
}

fn sequence_append_explicit_common(n: usize) -> Sample {
    let mut sequence = Sequence::with_allocator(BaseAllocator::common());
    let start = Instant::now();
    for i in 0..n {
        sequence.append(i as f64);
    }
    Sample::since(start, n, black_box(sequence.len()) as i64)
}

fn sequence_edit_batch(n: usize) -> Sample {
    let mut sequence: Sequence<i32> = [1, 2, 3, 4].into_iter().collect();
    let mut tail = Sequence::default();
    let start = Instant::now();
    let mut checksum = 0;
    for i in 0..n {
        edit_sequence(&mut sequence, &mut tail, black_box((i & 7) as i32));
        checksum += i64::from(sequence.first().copied().unwrap());
        checksum += i64::from(sequence.last().copied().unwrap());
    }
    Sample::since(start, n, black_box(checksum))
}

fn edit_sequence(sequence: &mut Sequence<i32>, tail: &mut Sequence<i32>, value: i32) {
    sequence.append(value);
    sequence.prepend(7);
    sequence.insert_before(3, 8).unwrap();
    sequence.exchange(1, 7).unwrap();
    sequence.reverse();
    sequence.split(5, tail).unwrap();
    sequence.append_sequence(tail);
    sequence.remove_range(1, 3).unwrap();
}

fn sequence_cursor_walk(n: usize) -> Sample {
    let sequence = filled_sequence(n);
    let start = Instant::now();
    let mut cursor = sequence.cursor();
    let mut sum = 0.0;
    while let Some(item) = cursor.value(&sequence) {
        sum += item;
        cursor.next(&sequence);
    }
    Sample::since(start, n, black_box(sum) as i64)
}

fn list_append_explicit_common(n: usize) -> Sample {
    let mut list = List::with_allocator(BaseAllocator::common());
    let start = Instant::now();
    for i in 0..n as i32 {
        list.append(i);
    }
    Sample::since(start, n, black_box(list.len()) as i64)
}

fn list_edit_batch(n: usize) -> Sample {
    let mut list: List<i32> = [1, 2, 3, 4].into_iter().collect();
    let start = Instant::now();
    let mut checksum = 0;
    for i in 0..n {
        edit_list(&mut list, black_box((i & 7) as i32));
        checksum += i64::from(list.first().copied().unwrap());
        checksum += i64::from(list.last().copied().unwrap());
    }
    Sample::since(start, n, black_box(checksum))
}

fn edit_list(list: &mut List<i32>, value: i32) {
    list.append(value);
    list.prepend(7);
    let mut position = list.cursor();
    position.next(list);
    list.insert_before_at(8, &mut position).unwrap();
    list.remove(&mut position).unwrap();
    list.reverse();
    list.remove_first().unwrap();
    list.remove_first().unwrap();
}

fn list_cursor_walk(n: usize) -> Sample {
    let list: List<i32> = (0..n as i32).collect();
    let start = Instant::now();
    let mut cursor = list.cursor();
    let mut sum = 0;
    while let Some(item) = cursor.value(&list) {
        sum += i64::from(*item);
        cursor.next(&list);
    }
    Sample::since(start, n, black_box(sum))
}
