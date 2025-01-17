use na::allocator::Allocator;
use na::dimension::DimMin;
use na::dimension::{U2, U4};
use na::{DefaultAllocator, RealField};
use na::{OMatrix, OVector};
use nalgebra as na;

use adskalman::ObservationModelLinear;

// observation model -------

pub struct PositionObservationModel<R: RealField>
where
    DefaultAllocator: Allocator<R, U4, U4>,
    DefaultAllocator: Allocator<R, U2, U4>,
    DefaultAllocator: Allocator<R, U4, U2>,
    DefaultAllocator: Allocator<R, U2, U2>,
    DefaultAllocator: Allocator<R, U4>,
{
    pub observation_matrix: OMatrix<R, U2, U4>,
    pub observation_matrix_transpose: OMatrix<R, U4, U2>,
    pub observation_noise_covariance: OMatrix<R, U2, U2>,
}

impl<R: RealField> PositionObservationModel<R> {
    #[allow(dead_code)]
    pub fn new(var: R) -> Self {
        let one = na::convert(1.0);
        let zero = na::convert(0.0);
        // Create observation model. We only observe the position.
        #[rustfmt::skip]
        let observation_matrix = OMatrix::<R,U2,U4>::new(one, zero, zero, zero,
                                    zero, one, zero, zero);
        #[rustfmt::skip]
        let observation_noise_covariance = OMatrix::<R,U2,U2>::new(var, zero,
                                                zero, var);
        Self {
            observation_matrix,
            observation_matrix_transpose: observation_matrix.transpose(),
            observation_noise_covariance,
        }
    }
}

impl<R: RealField> ObservationModelLinear<R, U4, U2> for PositionObservationModel<R>
where
    DefaultAllocator: Allocator<R, U4, U4>,
    DefaultAllocator: Allocator<R, U2, U4>,
    DefaultAllocator: Allocator<R, U4, U2>,
    DefaultAllocator: Allocator<R, U2, U2>,
    DefaultAllocator: Allocator<R, U4>,
    DefaultAllocator: Allocator<R, U2>,
    DefaultAllocator: Allocator<(usize, usize), U2>,
    U2: DimMin<U2, Output = U2>,
{
    fn observation_matrix(&self) -> &OMatrix<R, U2, U4> {
        &self.observation_matrix
    }
    fn observation_matrix_transpose(&self) -> &OMatrix<R, U4, U2> {
        &self.observation_matrix_transpose
    }
    fn observation_noise_covariance(&self) -> &OMatrix<R, U2, U2> {
        &self.observation_noise_covariance
    }
    fn evaluate(&self, state: &OVector<R, U4>) -> OVector<R, U2> {
        &self.observation_matrix * state
    }
}
