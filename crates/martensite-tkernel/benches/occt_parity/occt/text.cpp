// SPDX-License-Identifier: GPL-2.0-only
// Twins of ../text.rs.
#include "parity.hxx"

#include <TCollection_AsciiString.hxx>
#include <TCollection_ExtendedString.hxx>
#include <TCollection_HAsciiString.hxx>
#include <TCollection_HExtendedString.hxx>

#include <cstdint>
#include <limits>

namespace
{
using parity::Clock;
using parity::Sample;
using parity::since;

// n / 10 appends of two characters, then one search for a suffix.
Sample ascii_cat_search(long long n)
{
  const long long         appends = n / 10;
  const auto              start   = Clock::now();
  TCollection_AsciiString text;
  for (long long i = 0; i < appends; ++i)
    text.AssignCat("ab");
  text.AssignCat("needle");
  const long long found = text.Search("needle");
  return since(start, appends, found);
}

Sample ascii_binary_search(long long n)
{
  const char bytes[] = {static_cast<char>(255), 0, 'a'};
  TCollection_AsciiString text(1024, 'x');
  text.Insert(513, bytes, 3);
  long long sum = 0;
  const auto start = Clock::now();
  for (long long i = 0; i < n / 10; ++i)
  {
    parity::keep(text);
    sum += text.Search(bytes, 3);
    parity::keep(text);
    sum += text.SearchFromEnd(bytes, 3);
  }
  return since(start, n / 5, sum);
}

uint64_t ascii_text_checksum(const TCollection_AsciiString& text)
{
  uint64_t sum = 0;
  for (int i = 0; i < text.Length(); ++i)
    sum = sum * 31 + static_cast<unsigned char>(text.ToCString()[i]);
  return sum;
}

Sample ascii_format_integer(long long n)
{
  uint64_t sum = 0;
  const auto start = Clock::now();
  for (long long i = 0; i < n; ++i)
  {
    const TCollection_AsciiString text(std::numeric_limits<int>::min() + static_cast<int>(i));
    sum += ascii_text_checksum(text);
  }
  return since(start, n, static_cast<int64_t>(sum));
}

Sample ascii_format_real(long long n)
{
  uint64_t sum = 0;
  const auto start = Clock::now();
  for (long long i = 0; i < n; ++i)
  {
    const TCollection_AsciiString text(static_cast<double>(i) * -0.000000100001);
    sum += ascii_text_checksum(text);
  }
  return since(start, n, static_cast<int64_t>(sum));
}

Sample extended_cat_search(long long n)
{
  const long long appends = n / 10;
  const auto start = Clock::now();
  TCollection_ExtendedString text;
  for (long long i = 0; i < appends; ++i)
    text.AssignCat(u"ab");
  const char16_t needle[] = {78, 0x20ac};
  text.AssignCat(needle, 2);
  return since(start, appends, text.Search(needle, 2));
}

Sample hascii_cat_search(long long n)
{
  const long long appends = n / 10;
  const auto start = Clock::now();
  TCollection_HAsciiString text;
  for (long long i = 0; i < appends; ++i)
    text.AssignCat("ab");
  text.AssignCat("needle");
  return since(start, appends, text.Search("needle"));
}

Sample hextended_from_ascii(long long n)
{
  occ::handle<TCollection_HAsciiString> ascii = new TCollection_HAsciiString("WIDE \xE2\x82\xAC");
  const auto start = Clock::now();
  long long sum = 0;
  for (long long i = 0; i < n; ++i)
  {
    parity::keep(ascii);
    const TCollection_HExtendedString text(ascii);
    sum += text.String().Length() + text.String().Value(text.String().Length());
  }
  return since(start, n, sum);
}
} // namespace

void parity::text(std::vector<Case>& cases)
{
  cases.push_back({"ascii_cat_search", ascii_cat_search});
  cases.push_back({"ascii_borrowed_cat_search", ascii_cat_search});
  cases.push_back({"ascii_binary_search", ascii_binary_search});
  cases.push_back({"ascii_format_integer", ascii_format_integer});
  cases.push_back({"ascii_format_real", ascii_format_real});
  cases.push_back({"extended_cat_search", extended_cat_search});
  cases.push_back({"hascii_cat_search", hascii_cat_search});
  cases.push_back({"hextended_from_ascii", hextended_from_ascii});
}
