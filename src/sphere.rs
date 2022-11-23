//! # This Module for sphere Shape
//! 
//! - This module take solid's radius as input parameter
//! - This module gives output volume, surface area, summary


use super::common_formula::Formula;
use std::f32::consts::PI;

// struct definiton
#[derive(Debug)]
pub struct Sphere
{
	/// radius of a Sphere
	pub radius: f32,
}

impl Sphere
{
	/// create new sphere instance
	/// # Examples
	///
	/// Basic usage:
	///
	/// ```
	/// use formula_3d_shape_lib::sphere::Sphere;
	/// let aphere = Sphere::new(4.0);
	/// ```
	pub fn new(radius:f32) -> Sphere
	{ 
		Sphere {
		radius,
		} 
	}
}




impl Formula for Sphere
{
	/// this function return volume area
	/// # Examples
	///
	/// Basic usage:
	///
	/// ```
	/// use formula_3d_shape_lib::sphere::Sphere;
	/// use formula_3d_shape_lib::common_formula::Formula;
	/// let sphere1 = Sphere::new(4.0);
	/// sphere1.get_volume();
	/// ```
	fn get_volume(&self) -> f32
	{
		return 4.0 / 3.0 * PI * self.radius * self.radius * self.radius;
	}

	/// this function return surface area
	/// # Examples
	///
	/// Basic usage:
	///
	/// ```
	/// use formula_3d_shape_lib::sphere::Sphere;
	/// use formula_3d_shape_lib::common_formula::Formula;
	/// let sphere1 = Sphere::new(4.0);
	/// sphere1.get_surface_area();
	/// ```
	fn get_surface_area(&self) -> f32
	{
		return 4.0* PI * self.radius * self.radius;
	}
}

impl Sphere
{
	/// this function gives summary of solid
	/// # Examples
	///
	/// Basic usage:
	///
	/// ```
	/// use formula_3d_shape_lib::sphere::Sphere;
	/// use formula_3d_shape_lib::common_formula::Formula;
	/// let sphere1 = Sphere::new(4.0);
	/// sphere1.summary();
	/// ```
	pub fn summary(&self)
	{
		println!("summary:");
		println!("radius : {}", self.radius);
		println!("volume is {} and surface area is {}", self.get_volume(), self.get_surface_area());
	}
}