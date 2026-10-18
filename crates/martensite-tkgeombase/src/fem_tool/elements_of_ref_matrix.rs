//! `ElementsOfRefMatrix`: the products of two Hermite-Jacobi basis
//! derivatives, integrated by the criteria into their reference matrices.
use crate::native_failure::{array_failure, failure};
use martensite_tkernel::ncollection::Array1;
use martensite_tkernel::standard::Failure;
use martensite_tkmath::math::{FunctionSet, Vector};
use martensite_tkmath::plib::HermitJacobi;

/// `ElementsOfRefMatrix`: a one-variable function set whose equations are
/// the upper triangle `Pi^(k) * Pj^(k)` (i <= j) of the basis derivative of order k.
///
/// ```
/// use martensite_tkgeombase::fem_tool::ElementsOfRefMatrix;
/// use martensite_tkmath::geom_abs::Shape;
/// use martensite_tkmath::plib::HermitJacobi;
/// let base = HermitJacobi::new(5, Shape::C0)?;
/// let elements = ElementsOfRefMatrix::new(&base, 1)?;
/// assert_eq!(elements.nb_equations(), 21);
/// # Ok::<(), martensite_tkernel::standard::Failure>(())
/// ```
#[derive(Clone, Copy, Debug)]
pub struct ElementsOfRefMatrix {
    base: HermitJacobi,
    der_order: i32,
    nb_equations: i32,
}

impl ElementsOfRefMatrix {
    /// `ElementsOfRefMatrix(TheBase, DerOrder)`: construction failure
    /// for a derivative order outside `0..=3`;
    /// `ElementsOfRefMatrix::new(&base, 2)?`.
    pub fn new(base: &HermitJacobi, der_order: i32) -> Result<Self, Failure> {
        if !(0..=3).contains(&der_order) {
            return Err(failure(
                "Standard_ConstructionError",
                &format!("FEmTool_ElementsOfRefMatrix: DerOrder {der_order}; expected 0..=3"),
            ));
        }
        let degree: i32 = base.work_degree();
        Ok(Self {
            base: *base,
            der_order,
            nb_equations: (degree + 2) * (degree + 1) / 2,
        })
    }

    /// `NbVariables()`: always 1; `elements.nb_variables()`.
    pub fn nb_variables(&self) -> i32 {
        1
    }

    /// `NbEquations()`: `(WorkDegree + 2) * (WorkDegree + 1) / 2`;
    /// `elements.nb_equations()`.
    pub fn nb_equations(&self) -> i32 {
        self.nb_equations
    }

    /// `Value(X, F)`: writes `P0*P0, P0*P1, ..., P1*P1, ...` from `F.Lower()`;
    /// index-range failure when `F` is shorter than NbEquations. Always true
    /// otherwise; `elements.evaluate(&x, &mut f)?`.
    pub fn evaluate(&self, x: &Vector<'_>, f: &mut Vector<'_>) -> Result<bool, Failure> {
        if f.length() < self.nb_equations {
            return Err(Failure::out_of_range(&format!(
                "FEmTool_ElementsOfRefMatrix::Value: F has {} entries; expected >= {}",
                f.length(),
                self.nb_equations
            )));
        }
        let basis: Array1<'static, f64> = self.basis(*x.value(x.lower())?)?;
        let (degree, lower) = (self.base.work_degree(), f.lower());
        let mut ii: i32 = 0;
        for i in 0..=degree {
            for j in i..=degree {
                *f.change_value(lower + ii)? = basis[i] * basis[j];
                ii += 1;
            }
        }
        Ok(true)
    }

    // The basis derivative of order DerOrder at `u` (the lower orders go to scratch
    // arrays: the retained call passes one Aux array for all of them, write-only).
    fn basis(&self, u: f64) -> Result<Array1<'static, f64>, Failure> {
        let degree: i32 = self.base.work_degree();
        let new_row = || {
            Array1::<f64>::with_bounds(0, degree)
                .map_err(|error| array_failure(error, "FEmTool_ElementsOfRefMatrix::Value"))
        };
        let (mut basis, mut a1, mut a2, mut a3) = (new_row()?, new_row()?, new_row()?, new_row()?);
        match self.der_order {
            0 => self.base.d0(u, &mut basis)?,
            1 => self.base.d1(u, &mut a1, &mut basis)?,
            2 => self.base.d2(u, &mut a1, &mut a2, &mut basis)?,
            _ => self.base.d3(u, &mut a1, &mut a2, &mut a3, &mut basis)?,
        }
        Ok(basis)
    }
}

impl FunctionSet for ElementsOfRefMatrix {
    fn variables(&self) -> i32 {
        self.nb_variables()
    }

    fn equations(&self) -> i32 {
        self.nb_equations()
    }

    // math_FunctionSet::Value has no error channel: a short F (the retained
    // index-range failure) reports a failed evaluation.
    fn value(&mut self, point: &Vector<'_>, residual: &mut Vector<'_>) -> bool {
        self.evaluate(point, residual).unwrap_or(false)
    }
}
