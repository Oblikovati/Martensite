//! The entities a builder makes: copious data from its points, a matrix from its position.
use super::IgesConvGeomGeomBuilder;
use crate::iges_data::{iges_array1_set, iges_array2_set, iges_new_array1, iges_new_array2};
use crate::iges_geom::{IgesGeomCopiousData, IgesGeomTransformationMatrix};
use martensite_tkernel::ncollection::{HArray1, HArray2};
use martensite_tkernel::standard::{Failure, Handle, exceptions};

const WHERE: &str = "IgesConvGeomGeomBuilder";

impl IgesConvGeomGeomBuilder {
    /// The coordinates of one recorded point as `datatype` lays them out.
    fn copious_values(&self, rank: usize, datatype: i32) -> Vec<f64> {
        let (point, vector) = (self.points[rank], self.vectors[rank]);
        let mut values = vec![point.x(), point.y()];
        if datatype > 1 {
            values.push(point.z());
        }
        if datatype == 3 {
            values.extend([vector.x(), vector.y(), vector.z()]);
        }
        values
    }
    fn copious_array(&self, datatype: i32) -> Result<Handle<HArray1<f64>>, Failure> {
        let values: Vec<f64> = (0..self.points.len())
            .flat_map(|rank| self.copious_values(rank, datatype))
            .collect();
        let array = iges_new_array1(1, values.len() as i32)?;
        for (rank, value) in (1..).zip(values) {
            iges_array1_set(&array, rank, value, WHERE)?;
        }
        Ok(array)
    }
    /// Makes copious data of the recorded points: `datatype` is 1 with a common Z, the average of
    /// the Z values, 2 with XYZ points, 3 with their vectors. `polyline` codes it as a polyline,
    /// which excludes the datatype 3.
    /// Example: `builder.make_copious_data(2, false)?`.
    ///
    /// # Errors
    /// A domain failure for another datatype, no point, or a polyline of datatype 3.
    pub fn make_copious_data(
        &self,
        datatype: i32,
        polyline: bool,
    ) -> Result<Handle<IgesGeomCopiousData>, Failure> {
        if !(1..=3).contains(&datatype) || self.points.is_empty() || (polyline && datatype == 3) {
            return Err(exceptions::domain_error(&format!(
                "{WHERE}::make_copious_data: datatype {datatype}, polyline {polyline} on {} points; \
                 expected datatype 1 to 3, a point, and no polyline of datatype 3",
                self.points.len()
            )));
        }
        let common_z = if datatype == 1 {
            let sum: f64 = self.points.iter().map(|point| point.z()).sum();
            sum / self.points.len() as f64
        } else {
            0.0
        };
        let entity = IgesGeomCopiousData::new();
        entity.init(datatype, common_z, &self.copious_array(datatype)?)?;
        entity.set_polyline(polyline);
        Ok(Handle::new(entity))
    }
    /// The transformation of the position, also for the identity; `unit` divides its translation.
    /// Example: `builder.make_transformation(1.0)?`.
    ///
    /// # Errors
    /// A failure of the coefficients of the position.
    pub fn make_transformation(
        &self,
        unit: f64,
    ) -> Result<Handle<IgesGeomTransformationMatrix>, Failure> {
        let values: Handle<HArray2<f64>> = iges_new_array2(1, 3, 1, 4)?;
        for row in 1..=3 {
            for column in 1..=4 {
                let coefficient = self.position.value(row, column)?;
                let value = if column == 4 {
                    coefficient / unit
                } else {
                    coefficient
                };
                iges_array2_set(&values, row, column, value, WHERE)?;
            }
        }
        let entity = IgesGeomTransformationMatrix::new();
        entity.init(&values)?;
        if self.position.is_negative() {
            entity.set_form_number(1)?;
        }
        Ok(Handle::new(entity))
    }
}
