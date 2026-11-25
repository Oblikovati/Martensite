// SPDX-License-Identifier: GPL-2.0-only
//! Common local constant axial/radial-rate helix law.

#[derive(Clone, Copy, Debug)]
pub(crate) struct LocalHelix {
    pub opening_radius: f64,
    pub radial_rate: f64,
    pub axial_rate: f64,
    pub positive: bool,
}

impl LocalHelix {
    pub(crate) fn radius(self, parameter: f64) -> f64 {
        self.opening_radius + self.radial_rate * parameter
    }

    pub(crate) fn point(self, parameter: f64, cosine: f64, sine: f64) -> [f64; 3] {
        let radius = self.radius(parameter);
        [
            radius * cosine,
            self.sense(radius * sine),
            self.axial_rate * parameter,
        ]
    }

    pub(crate) fn first(self, parameter: f64, cosine: f64, sine: f64) -> [f64; 3] {
        let radius = self.radius(parameter);
        [
            self.radial_rate * cosine - radius * sine,
            self.sense(self.radial_rate * sine + radius * cosine),
            self.axial_rate,
        ]
    }

    pub(crate) fn second(self, parameter: f64, cosine: f64, sine: f64) -> [f64; 3] {
        let radius = self.radius(parameter);
        let twice = 2. * self.radial_rate;
        // #614. This checked law differentiates D1. The audited legacy adaptor
        // keeps its historical negative Y cross-term for parity replay.
        [
            -radius * cosine - twice * sine,
            self.sense(-radius * sine + twice * cosine),
            0.,
        ]
    }

    pub(crate) fn sense(self, value: f64) -> f64 {
        if self.positive { value } else { -value }
    }
}
