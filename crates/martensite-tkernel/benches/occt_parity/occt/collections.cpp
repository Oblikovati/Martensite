// SPDX-License-Identifier: GPL-2.0-only
// Twins of ../collections.rs.
#include "parity.hxx"

#include <NCollection_DataMap.hxx>
#include <NCollection_DynamicArray.hxx>
#include <NCollection_IndexedMap.hxx>
#include <NCollection_List.hxx>
#include <NCollection_Sequence.hxx>

namespace
{
using parity::Clock;
using parity::Sample;
using parity::since;

void fill(NCollection_DataMap<int, int>& map, long long n)
{
  for (int i = 0; i < n; ++i)
    map.Bind(i, 2 * i);
}

Sample datamap_bind(long long n)
{
  const auto                    start = Clock::now();
  NCollection_DataMap<int, int> map;
  fill(map, n);
  return since(start, n, map.Extent());
}

Sample datamap_seek(long long n)
{
  NCollection_DataMap<int, int> map;
  fill(map, n);
  const auto start = Clock::now();
  long long  sum   = 0;
  for (int i = 0; i < n; ++i)
  {
    const int* value = map.Seek(i);
    sum += value != nullptr ? *value : 0;
  }
  return since(start, n, sum);
}

void fill(NCollection_IndexedMap<int>& map, long long n)
{
  for (int i = 0; i < n; ++i)
    map.Add(i);
}

Sample indexedmap_add(long long n)
{
  const auto                  start = Clock::now();
  NCollection_IndexedMap<int> map;
  fill(map, n);
  return since(start, n, map.Extent());
}

Sample indexedmap_find(long long n)
{
  NCollection_IndexedMap<int> map;
  fill(map, n);
  const auto start = Clock::now();
  long long  sum   = 0;
  for (int i = 0; i < n; ++i)
    sum += map.FindIndex(i);
  return since(start, n, sum);
}

void fill(NCollection_Sequence<double>& sequence, long long n)
{
  for (long long i = 0; i < n; ++i)
    sequence.Append(double(i));
}

Sample sequence_append(long long n)
{
  const auto                   start = Clock::now();
  NCollection_Sequence<double> sequence;
  fill(sequence, n);
  return since(start, n, sequence.Length());
}

Sample sequence_iterate(long long n)
{
  NCollection_Sequence<double> sequence;
  fill(sequence, n);
  const auto start = Clock::now();
  double     sum   = 0.0;
  for (NCollection_Sequence<double>::Iterator it(sequence); it.More(); it.Next())
    sum += it.Value();
  return since(start, n, (long long)sum);
}

Sample sequence_value(long long n)
{
  NCollection_Sequence<double> sequence;
  fill(sequence, n);
  const auto start = Clock::now();
  double     sum   = 0.0;
  for (int i = 1; i <= n; ++i)
    sum += sequence.Value(i);
  return since(start, n, (long long)sum);
}

void fill(NCollection_DynamicArray<double>& array, long long n)
{
  for (long long i = 0; i < n; ++i)
    array.Append(double(i));
}

Sample dynarray_append(long long n)
{
  const auto                       start = Clock::now();
  NCollection_DynamicArray<double> array(256);
  fill(array, n);
  return since(start, n, array.Length());
}

Sample dynarray_value(long long n)
{
  NCollection_DynamicArray<double> array(256);
  fill(array, n);
  const auto start = Clock::now();
  double     sum   = 0.0;
  for (int i = 0; i < n; ++i)
    sum += array.Value(i);
  return since(start, n, (long long)sum);
}

Sample list_append_iterate(long long n)
{
  const auto            start = Clock::now();
  NCollection_List<int> list;
  for (int i = 0; i < n; ++i)
    list.Append(i);
  long long sum = 0;
  for (NCollection_List<int>::Iterator it(list); it.More(); it.Next())
    sum += it.Value();
  return since(start, n, sum);
}
Sample sequence_append_explicit_common(long long n)
{
  NCollection_Sequence<double> sequence(NCollection_BaseAllocator::CommonBaseAllocator());
  const auto start = Clock::now();
  fill(sequence, n);
  return since(start, n, sequence.Length());
}

void edit_sequence(NCollection_Sequence<int>& sequence, NCollection_Sequence<int>& tail, int value)
{
  sequence.Append(value);
  sequence.Prepend(7);
  sequence.InsertBefore(3, 8);
  sequence.Exchange(1, 7);
  sequence.Reverse();
  sequence.Split(5, tail);
  sequence.Append(tail);
  sequence.Remove(1, 3);
}

Sample sequence_edit_batch(long long n)
{
  NCollection_Sequence<int> sequence;
  NCollection_Sequence<int> tail;
  for (int i = 1; i <= 4; ++i)
    sequence.Append(i);
  const auto start = Clock::now();
  long long checksum = 0;
  for (long long i = 0; i < n; ++i)
  {
    edit_sequence(sequence, tail, int(i & 7));
    checksum += sequence.First() + sequence.Last();
  }
  return since(start, n, checksum);
}

Sample list_append_explicit_common(long long n)
{
  NCollection_List<int> list(NCollection_BaseAllocator::CommonBaseAllocator());
  const auto start = Clock::now();
  for (int i = 0; i < n; ++i)
    list.Append(i);
  return since(start, n, list.Extent());
}

void edit_list(NCollection_List<int>& list, int value)
{
  list.Append(value);
  list.Prepend(7);
  NCollection_List<int>::Iterator position(list);
  position.Next();
  list.InsertBefore(8, position);
  list.Remove(position);
  list.Reverse();
  list.RemoveFirst();
  list.RemoveFirst();
}

Sample list_edit_batch(long long n)
{
  NCollection_List<int> list;
  for (int i = 1; i <= 4; ++i)
    list.Append(i);
  const auto start = Clock::now();
  long long checksum = 0;
  for (long long i = 0; i < n; ++i)
  {
    edit_list(list, int(i & 7));
    checksum += list.First() + list.Last();
  }
  return since(start, n, checksum);
}

Sample list_cursor_walk(long long n)
{
  NCollection_List<int> list;
  for (int i = 0; i < n; ++i)
    list.Append(i);
  const auto start = Clock::now();
  long long sum = 0;
  for (NCollection_List<int>::Iterator it(list); it.More(); it.Next())
    sum += it.Value();
  return since(start, n, sum);
}
} // namespace

void parity::collections(std::vector<Case>& cases)
{
  cases.insert(cases.end(),
               {{"datamap_bind", datamap_bind},
                {"datamap_seek", datamap_seek},
                {"indexedmap_add", indexedmap_add},
                {"indexedmap_find", indexedmap_find},
                {"sequence_append", sequence_append},
                {"sequence_append_explicit_common", sequence_append_explicit_common},
                {"sequence_edit_batch", sequence_edit_batch},
                {"sequence_cursor_walk", sequence_iterate},
                {"sequence_iterate", sequence_iterate},
                {"sequence_value", sequence_value},
                {"dynarray_append", dynarray_append},
                {"dynarray_value", dynarray_value},
                {"list_append_iterate", list_append_iterate},
                {"list_append_explicit_common", list_append_explicit_common},
                {"list_edit_batch", list_edit_batch},
                {"list_cursor_walk", list_cursor_walk}});
}
