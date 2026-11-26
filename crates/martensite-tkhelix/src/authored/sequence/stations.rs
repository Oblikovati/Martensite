// SPDX-License-Identifier: GPL-2.0-only
//! Exact turns and rise accumulate once before literal native stations are chosen.
use super::{
    FlatEndHelixRequest, HelixEndZone, HelixPitchRow, HelixSequenceError as Failure,
    HelixSequenceSpan, HelixSpanSource as Source,
};
use crate::authored::{HelixQuantity, law::positive};
use martensite_tkmath::predicates::roots::DyadicBound as Exact;
use std::{collections::BTreeSet, f64::consts::TAU};

#[derive(Clone, Copy, Debug)]
pub(super) enum Form {
    Flat,
    Constant,
    Accelerating,
    Decelerating,
}

#[derive(Clone, Debug)]
pub(super) struct Station {
    pub turns: Exact,
    pub height: Exact,
    pub parameter: f64,
    pub elevation: f64,
}

impl Station {
    fn opening() -> Result<Self, Failure> {
        let zero = exact(0., Source::Natural)?;
        Ok(Self {
            turns: zero.clone(),
            height: zero,
            parameter: 0.,
            elevation: 0.,
        })
    }

    fn next(&self, piece: &Piece) -> Result<Self, Failure> {
        let turns = self.turns.sum(&piece.width);
        let height = self.height.sum(&piece.rise);
        let parameter = TAU * representable(&turns, piece.source)?;
        let elevation = representable(&height, piece.source)?;
        if !parameter.is_finite()
            || parameter <= self.parameter
            || (!matches!(piece.form, Form::Flat) && elevation <= self.elevation)
        {
            return Err(Failure::UnrepresentableSpan(piece.source));
        }
        Ok(Self {
            turns,
            height,
            parameter,
            elevation,
        })
    }
}

#[derive(Debug)]
pub(super) struct Span {
    pub source: Source,
    pub form: Form,
    pub stations: [Station; 2],
    pub natural_rate: f64,
    pub natural_parameter_width: f64,
    pub rise: f64,
}

impl Span {
    pub(super) fn summary(&self) -> HelixSequenceSpan {
        HelixSequenceSpan {
            source: self.source,
            parameters: self.stations.each_ref().map(|station| station.parameter),
            elevations: self.stations.each_ref().map(|station| station.elevation),
        }
    }
}

struct Piece {
    source: Source,
    form: Form,
    width: Exact,
    rise: Exact,
    pitch: f64,
}

impl Piece {
    fn flat(width: f64, source: Source) -> Result<Self, Failure> {
        Self::literal(width, 0., 0, source, Form::Flat)
    }

    fn literal(
        width: f64,
        pitch: f64,
        halvings: u8,
        source: Source,
        form: Form,
    ) -> Result<Self, Failure> {
        Ok(Self {
            source,
            form,
            width: exact(width, source)?,
            rise: product(pitch, width, halvings, source)?,
            pitch,
        })
    }

    fn span(&self, first: Station) -> Result<Span, Failure> {
        let last = first.next(self)?;
        let rise = representable(&self.rise, self.source)?;
        let natural_parameter_width = TAU * representable(&self.width, self.source)?;
        let natural_rate = self.pitch / TAU;
        if !natural_parameter_width.is_finite()
            || natural_parameter_width <= 0.
            || (!matches!(self.form, Form::Flat) && (natural_rate <= 0. || rise <= 0.))
        {
            return Err(Failure::UnrepresentableSpan(self.source));
        }
        Ok(Span {
            source: self.source,
            form: self.form,
            stations: [first, last],
            natural_rate,
            natural_parameter_width,
            rise,
        })
    }
}

fn announce(pieces: impl IntoIterator<Item = Piece>) -> Result<Box<[Span]>, Failure> {
    let mut station = Station::opening()?;
    let mut spans = Vec::new();
    for piece in pieces {
        let span = piece.span(station)?;
        station = span.stations[1].clone();
        spans.push(span);
    }
    Ok(spans.into_boxed_slice())
}

pub(super) fn table(rows: &[HelixPitchRow]) -> Result<Box<[Span]>, Failure> {
    if !(1..=128).contains(&rows.len()) {
        return Err(Failure::RowCount(rows.len()));
    }
    let mut ids = BTreeSet::new();
    let mut pieces = Vec::with_capacity(rows.len());
    for &HelixPitchRow { id, pitch, turns } in rows {
        if !ids.insert(id) {
            return Err(Failure::DuplicateRow(id));
        }
        positive(pitch, HelixQuantity::Pitch)?;
        positive(turns, HelixQuantity::Turns)?;
        pieces.push(Piece::literal(
            turns,
            pitch,
            0,
            Source::Row(id),
            Form::Constant,
        )?);
    }
    announce(pieces)
}

pub(super) fn flat(request: FlatEndHelixRequest) -> Result<Box<[Span]>, Failure> {
    let FlatEndHelixRequest {
        turns,
        pitch,
        start,
        end,
        ..
    } = request;
    positive(turns, HelixQuantity::Turns)?;
    positive(pitch, HelixQuantity::Pitch)?;
    let [start, end] = [zones(start, pitch, true)?, zones(end, pitch, false)?];
    let natural = unreserved(turns, pitch, start.iter().chain(&end))?;
    announce(start.into_iter().chain(natural).chain(end))
}

fn zones(zone: Option<HelixEndZone>, pitch: f64, start: bool) -> Result<Vec<Piece>, Failure> {
    let Some(zone) = zone else {
        return Ok(Vec::new());
    };
    if !(0. ..1.).contains(&zone.flat_turns)
        || !zone.transition_turns.is_finite()
        || zone.transition_turns <= 0.
    {
        return Err(Failure::EndZone { start, zone });
    }
    let ramp = if start {
        Source::StartTransition
    } else {
        Source::EndTransition
    };
    let form = if start {
        Form::Accelerating
    } else {
        Form::Decelerating
    };
    let transition = Piece::literal(zone.transition_turns, pitch, 1, ramp, form)?;
    if zone.flat_turns == 0. {
        return Ok(vec![transition]);
    }
    let flat = Piece::flat(
        zone.flat_turns,
        if start {
            Source::StartFlat
        } else {
            Source::EndFlat
        },
    )?;
    Ok(if start {
        vec![flat, transition]
    } else {
        vec![transition, flat]
    })
}

fn unreserved<'a>(
    turns: f64,
    pitch: f64,
    zones: impl Iterator<Item = &'a Piece>,
) -> Result<Option<Piece>, Failure> {
    let source = Source::Natural;
    let mut width = exact(turns, source)?;
    let mut rise = product(pitch, turns, 0, source)?;
    for piece in zones {
        width = width.difference(&piece.width);
        // Natural rise accounts for the full reserved angle before ramps reduce it.
        rise = rise.difference(&natural_rise(piece, pitch)?);
    }
    match width
        .compare_f64(0.)
        .map_err(|_| Failure::UnrepresentableSpan(source))?
    {
        std::cmp::Ordering::Less => Err(Failure::OverlappingEndZones),
        std::cmp::Ordering::Equal => Ok(None),
        std::cmp::Ordering::Greater => Ok(Some(Piece {
            source,
            form: Form::Constant,
            width,
            rise,
            pitch,
        })),
    }
}

fn natural_rise(piece: &Piece, pitch: f64) -> Result<Exact, Failure> {
    let width = representable(&piece.width, piece.source)?;
    product(pitch, width, 0, piece.source)
}

fn exact(value: f64, source: Source) -> Result<Exact, Failure> {
    Exact::from_f64(value).map_err(|_| Failure::UnrepresentableSpan(source))
}

fn product(pitch: f64, width: f64, halvings: u8, source: Source) -> Result<Exact, Failure> {
    Exact::product_of_f64(pitch, width, halvings).map_err(|_| Failure::UnrepresentableSpan(source))
}

fn representable(value: &Exact, source: Source) -> Result<f64, Failure> {
    value
        .nearest_binary64()
        .ok_or(Failure::UnrepresentableSpan(source))
}
