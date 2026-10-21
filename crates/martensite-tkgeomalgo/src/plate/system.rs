//! The blocks of the plate systems: the kernel rows of the pinpoint and linear
//! constraints and the polynomial rows.
use super::{PlatePinpointConstraint, PlatePlate};
use martensite_tkernel::ncollection::array2_failure;
use martensite_tkernel::ncollection::{Array1, Array2};
use martensite_tkernel::standard::Failure;
use martensite_tkmath::gp::Xy;
use martensite_tkmath::math::{Matrix, Vector};

/// A linear constraint read once at solve time: its pinpoint constraints and its
/// coefficients (`ColLength` rows of `RowLength` columns, one-based). `C` is `f64` for
/// the XYZ constraints and `Xyz` for the scalar ones.
pub(super) struct Block<C: 'static> {
    pub(super) ppc: Array1<'static, PlatePinpointConstraint>,
    pub(super) coef: Array2<'static, C>,
}

impl<C: Copy + 'static> Block<C> {
    /// `ColLength`, the number of equations.
    pub(super) fn col_length(&self) -> i32 {
        self.coef.nb_rows() as i32
    }

    /// `RowLength`, the number of pinpoint constraints.
    pub(super) fn row_length(&self) -> i32 {
        self.coef.nb_columns() as i32
    }

    /// `Coeff(row, col)`; Standard_OutOfRange outside the block; `block.coeff(1, 1)?`.
    pub(super) fn coeff(&self, row: i32, col: i32) -> Result<C, Failure> {
        self.coef.value(row, col).copied().map_err(array2_failure)
    }

    pub(super) fn ppc(&self, index: i32) -> PlatePinpointConstraint {
        self.ppc[index]
    }
}

/// The sign `(-1)^(Deru + Derv)` of the adjoint kernel.
fn signe(deru: i32, derv: i32) -> f64 {
    if (deru + derv) % 2 == 1 { -1.0 } else { 1.0 }
}

/// The constraint points and derivation orders of a solve (`points`, `deru`, `derv`).
pub(super) struct Nodes<'a> {
    pub(super) points: &'a [Xy],
    pub(super) deru: &'a [i32],
    pub(super) derv: &'a [i32],
}

impl Nodes<'_> {
    /// `signe_j * SolEm(P_i - P_j, Deru_i + Deru_j, Derv_i + Derv_j)`, the kernel
    /// between nodes `i` and `j`.
    pub(super) fn kernel(&self, plate: &PlatePlate, i: usize, j: usize) -> Result<f64, Failure> {
        let u_order: i32 = self.deru[i] + self.deru[j];
        let v_order: i32 = self.derv[i] + self.derv[j];
        Ok(signe(self.deru[j], self.derv[j])
            * plate.sol_em(self.points[i] - self.points[j], u_order, v_order)?)
    }

    /// `Polm(P_j, u_order, v_order, Deru_j, Derv_j)`, the monomial row entry of node `j`.
    pub(super) fn monomial(
        &self,
        plate: &PlatePlate,
        j: usize,
        u_order: i32,
        v_order: i32,
    ) -> Result<f64, Failure> {
        plate.polm(self.points[j], u_order, v_order, self.deru[j], self.derv[j])
    }
}

/// `(u_order, v_order)` of the monomials `u^u_order v^v_order` with `u_order + v_order < order`, in the native order.
pub(super) fn monomials(order: i32) -> impl Iterator<Item = (i32, i32)> {
    (0..order).flat_map(move |u_order| (0..order - u_order).map(move |v_order| (u_order, v_order)))
}

/// Copies the lower triangle of the `size` block at `(i0, j0)` onto the upper one.
pub(super) fn symmetrize(mat: &mut Matrix<'_>, (i0, j0): (i32, i32), size: i32) {
    for i in 0..size {
        for j in i + 1..size {
            mat[(i0 + i, j0 + j)] = mat[(i0 + j, j0 + i)];
        }
    }
}

/// `fillXYZmatrix`: the block of the pinpoint and linear XYZ constraints at `(i0, j0)`
/// of the system, the kernel between the nodes, the XYZ combinations of kernel rows
/// and the monomial rows, symmetrized.
pub(super) fn fill_xyz_matrix(
    plate: &PlatePlate,
    nodes: &Nodes<'_>,
    xyz: &[Block<f64>],
    mat: &mut Matrix<'_>,
    (i0, j0, ncc1, ncc2): (i32, i32, i32, i32),
) -> Result<(), Failure> {
    for i in 0..ncc1 as usize {
        for j in 0..i {
            mat[(i0 + i as i32, j0 + j as i32)] = nodes.kernel(plate, i, j)?;
        }
    }
    let (mut k, mut kppc): (i32, i32) = (ncc1, ncc1);
    for (index, block) in xyz.iter().enumerate() {
        xyz_rows_against_nodes(plate, nodes, block, mat, (i0 + k, j0, kppc, ncc1))?;
        let (mut k2, mut kppc2): (i32, i32) = (ncc1, ncc1);
        for other in &xyz[..=index] {
            let tmp = kernel_block(
                plate,
                nodes,
                (kppc, block.row_length()),
                (kppc2, other.row_length()),
            )?;
            for irow in 1..=block.col_length() {
                for irow2 in 1..=other.col_length() {
                    for icol in 1..=block.row_length() {
                        for icol2 in 1..=other.row_length() {
                            mat[(i0 + k + irow - 1, j0 + k2 + irow2 - 1)] += block
                                .coeff(irow, icol)?
                                * other.coeff(irow2, icol2)?
                                * tmp[(icol, icol2)];
                        }
                    }
                }
            }
            k2 += other.col_length();
            kppc2 += other.row_length();
        }
        k += block.col_length();
        kppc += block.row_length();
    }
    xyz_monomial_rows(plate, nodes, xyz, mat, (i0 + ncc1 + ncc2, j0, ncc1))?;
    symmetrize(
        mat,
        (i0, j0),
        ncc1 + ncc2 + plate.order * (plate.order + 1) / 2,
    );
    Ok(())
}

/// The kernel values of the nodes of a block against the pinpoint nodes `0..ncc1`,
/// combined by the block rows.
fn xyz_rows_against_nodes(
    plate: &PlatePlate,
    nodes: &Nodes<'_>,
    block: &Block<f64>,
    mat: &mut Matrix<'_>,
    (row0, j0, kppc, ncc1): (i32, i32, i32, i32),
) -> Result<(), Failure> {
    for a_j in 0..ncc1 {
        let vmat: Vector<'static> =
            node_kernels(plate, nodes, kppc, block.row_length(), a_j as usize)?;
        for irow in 1..=block.col_length() {
            for icol in 1..=block.row_length() {
                mat[(row0 + irow - 1, j0 + a_j)] += block.coeff(irow, icol)? * vmat[icol];
            }
        }
    }
    Ok(())
}

/// `vmat(ippc) = kernel(kppc + ippc - 1, j)` for the `count` nodes of a block.
pub(super) fn node_kernels(
    plate: &PlatePlate,
    nodes: &Nodes<'_>,
    kppc: i32,
    count: i32,
    j: usize,
) -> Result<Vector<'static>, Failure> {
    let mut vmat: Vector<'static> = Vector::with_bounds(1, count)?;
    for ippc in 1..=count {
        vmat[ippc] = nodes.kernel(plate, (kppc + ippc - 1) as usize, j)?;
    }
    Ok(vmat)
}

/// `tmpmat(ippc, ippc2)`: the kernel between the nodes of two blocks.
pub(super) fn kernel_block(
    plate: &PlatePlate,
    nodes: &Nodes<'_>,
    (kppc, count): (i32, i32),
    (kppc2, count2): (i32, i32),
) -> Result<Matrix<'static>, Failure> {
    let mut tmp: Matrix<'static> = Matrix::with_bounds(1, count, 1, count2)
        .map_err(martensite_tkg2d::geom2d::errors::array_bounds)?;
    for ippc in 1..=count {
        for ippc2 in 1..=count2 {
            let (i, j) = ((kppc + ippc - 1) as usize, (kppc2 + ippc2 - 1) as usize);
            tmp[(ippc, ippc2)] = nodes.kernel(plate, i, j)?;
        }
    }
    Ok(tmp)
}

/// `vmat(ippc) = Polm(node, u_order, v_order)` for the `count` nodes of a block.
pub(super) fn node_monomials(
    plate: &PlatePlate,
    nodes: &Nodes<'_>,
    (kppc, count): (i32, i32),
    (u_order, v_order): (i32, i32),
) -> Result<Vector<'static>, Failure> {
    let mut vmat: Vector<'static> = Vector::with_bounds(1, count)?;
    for ippc in 1..=count {
        vmat[ippc] = nodes.monomial(plate, (kppc + ippc - 1) as usize, u_order, v_order)?;
    }
    Ok(vmat)
}

/// The monomial rows of the pinpoint nodes and of the XYZ blocks.
fn xyz_monomial_rows(
    plate: &PlatePlate,
    nodes: &Nodes<'_>,
    xyz: &[Block<f64>],
    mat: &mut Matrix<'_>,
    (first_row, j0, ncc1): (i32, i32, i32),
) -> Result<(), Failure> {
    for (i, (u_order, v_order)) in (first_row..).zip(monomials(plate.order)) {
        for a_j in 0..ncc1 {
            mat[(i, j0 + a_j)] = nodes.monomial(plate, a_j as usize, u_order, v_order)?;
        }
        let (mut k2, mut kppc2): (i32, i32) = (ncc1, ncc1);
        for block in xyz {
            let vmat = node_monomials(
                plate,
                nodes,
                (kppc2, block.row_length()),
                (u_order, v_order),
            )?;
            for irow2 in 1..=block.col_length() {
                for icol2 in 1..=block.row_length() {
                    mat[(i, j0 + k2 + irow2 - 1)] += block.coeff(irow2, icol2)? * vmat[icol2];
                }
            }
            k2 += block.col_length();
            kppc2 += block.row_length();
        }
    }
    Ok(())
}
