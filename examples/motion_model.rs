use nalgebra as na;
use na::core::MatrixN;
use na::core::dimension::{U2, U4};
use na::allocator::Allocator;
use na::{DefaultAllocator, RealField};

use adskalman::TransitionModelLinearNoControl;

// motion model -------

#[allow(dead_code)]
pub(crate) struct ConstantVelocity2DModel<R>
    where
        R: RealField,
        DefaultAllocator: Allocator<R, U4, U4>,
        DefaultAllocator: Allocator<R, U4>,
{
    pub(crate) transition_model: MatrixN<R,U4>,
    pub(crate) transition_model_transpose: MatrixN<R,U4>,
    pub(crate) transition_noise_covariance: MatrixN<R,U4>,
}

impl<R> ConstantVelocity2DModel<R>
    where
        R: RealField,
{
    #[allow(dead_code)]
    pub(crate) fn new(dt: R, noise_scale: R) -> Self {
        let one = na::convert(1.0);
        let zero = na::convert(0.0);
        // Create transition model. 2D position and 2D velocity.
        let transition_model = MatrixN::<R,U4>::new(one, zero,  dt, zero,
                            zero, one, zero,  dt,
                            zero, zero, one, zero,
                            zero, zero, zero, one);

        // This form is after N. Shimkin's lecture notes in
        // Estimation and Identification in Dynamical Systems
        // http://webee.technion.ac.il/people/shimkin/Estimation09/ch8_target.pdf
        // See also eq. 43 on pg. 13 of
        // http://www.robots.ox.ac.uk/~ian/Teaching/Estimation/LectureNotes2.pdf

        let t33 = dt*dt*dt/na::convert(3.0);
        let t22 = dt*dt/na::convert(2.0);
        let transition_noise_covariance = MatrixN::<R,U4>::new(t33, zero, t22, zero,
                                        zero, t33, zero, t22,
                                        t22, zero, dt, zero,
                                        zero, t22, zero, dt)*noise_scale;
        Self {
            transition_model,
            transition_model_transpose: transition_model.transpose(),
            transition_noise_covariance,
        }
    }
}

impl<R> TransitionModelLinearNoControl<R, U4> for ConstantVelocity2DModel<R>
    where
        R: RealField,
        DefaultAllocator: Allocator<R, U4, U4>,
        DefaultAllocator: Allocator<R, U2, U4>,
        DefaultAllocator: Allocator<R, U4, U2>,
        DefaultAllocator: Allocator<R, U2, U2>,
        DefaultAllocator: Allocator<R, U4>,
{
    fn transition_model(&self) -> &MatrixN<R,U4> {
        &self.transition_model
    }
    fn transition_model_transpose(&self) -> &MatrixN<R,U4> {
        &self.transition_model_transpose
    }
    fn transition_noise_covariance(&self) -> &MatrixN<R,U4> {
        &self.transition_noise_covariance
    }
}

#[allow(dead_code)]
fn main() {
    // TODO: can this .rs file be compiled just as a lib?
}
