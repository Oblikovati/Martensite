// SPDX-License-Identifier: GPL-2.0-only
use super::*;
use std::iter::Peekable;
use std::slice::Iter;
pub(super) struct CommonEndpoints<'a, 'v> {
    first: Peekable<Iter<'a, AxisEndpoint<'v>>>,
    second: Peekable<Iter<'a, AxisEndpoint<'v>>>,
}
pub(super) fn common<'a, 'v>(
    view: &'a ObservedIncidences<'v>,
    axes: [usize; 2],
) -> CommonEndpoints<'a, 'v> {
    let first = view.axes.get(axes[0]).map_or(&[][..], Vec::as_slice);
    let second = view.axes.get(axes[1]).map_or(&[][..], Vec::as_slice);
    CommonEndpoints {
        first: first.iter().peekable(),
        second: second.iter().peekable(),
    }
}
impl<'a, 'v> Iterator for CommonEndpoints<'a, 'v> {
    type Item = (&'a AxisEndpoint<'v>, &'a AxisEndpoint<'v>);
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let first = self.first.peek()?;
            let second = self.second.peek()?;
            match first.endpoint.cmp(&second.endpoint) {
                std::cmp::Ordering::Less => {
                    self.first.next();
                }
                std::cmp::Ordering::Greater => {
                    self.second.next();
                }
                std::cmp::Ordering::Equal => return self.first.next().zip(self.second.next()),
            }
        }
    }
}
