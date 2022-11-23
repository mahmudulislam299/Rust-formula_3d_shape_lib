//! # This Module for cylinder Shape
//! 
//! - This module take cylinder's radius and height as input parameter
//! - This module gives output cylinder's volume, surface area, summary

use super::common_formula::Formula;
use std::f32::consts::PI;



/// This is struct for Cylinder
/// It has two members
/// radius and height
#[derive(Debug)]
pub struct Cylinder
{
	/// radius of a Cylinder
	pub radius: f32,
	/// height of a Cylinder
	pub height: f32,
}

impl Cylinder
{
	/// It will create new Cylinder instance 
	///
	///
	/// # Examples
	///
	/// Basic usage:
	///
	/// ```
	/// use formula_3d_shape_lib::cylinder::Cylinder;
	/// let cylinder1 = Cylinder::new(4.0, 10.0);
	/// ```
	pub fn new(radius:f32, height:f32) -> Cylinder
	{ 
		Cylinder {
		radius,
		height,
		} 
	}
}




impl Formula for Cylinder
{
	/// this function gives the volume of cylinder
  /// # Examples
	///
	/// Basic usage:
	///
	/// ```
	/// use formula_3d_shape_lib::cylinder::Cylinder;
	/// use formula_3d_shape_lib::common_formula::Formula;
	/// let cylinder1 = Cylinder::new(4.0, 10.0);
	/// let vol = cylinder1.get_volume();
	/// ```
	fn get_volume(&self) -> f32
	{
		return PI * self.radius * self.radius * self.height;
	}

	/// This function gives the surface area of cylinder
  /// # Examples
	///
	/// Basic usage:
	///
	/// ```
	/// use formula_3d_shape_lib::cylinder::Cylinder;
	/// use formula_3d_shape_lib::common_formula::Formula;
	/// let cylinder1 = Cylinder::new(4.0, 10.0);
	/// let surface = cylinder1.get_surface_area();
	/// ```
	fn get_surface_area(&self) -> f32
	{
		return 2.0* PI * self.radius * self.height + 2.0 * PI * self.radius * self.radius;
	}
}

impl Cylinder
{
	/// this implementation gives the summary of Cylinder
	/// # Examples
	///
	/// Basic usage:
	///
	/// ```
	/// use formula_3d_shape_lib::cylinder::Cylinder;
	/// use formula_3d_shape_lib::common_formula::Formula;
	/// let cylinder1 = Cylinder::new(4.0, 10.0);
	/// cylinder1.summary();
	/// ```
	pub fn summary(&self)
	{
		println!("summary:");
		println!("radius : {} & height : {}", self.radius, self.height);
		println!("volume is {} and surface area is {}", self.get_volume(), self.get_surface_area());
	}
}
