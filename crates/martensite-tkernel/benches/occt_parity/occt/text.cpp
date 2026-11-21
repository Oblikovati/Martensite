// SPDX-License-Identifier: GPL-2.0-only
// Twins of ../text.rs.
#include "parity.hxx"

#include <TCollection_AsciiString.hxx>

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
} // namespace

void parity::text(std::vector<Case>& cases)
{
  cases.push_back({"ascii_cat_search", ascii_cat_search});
}
