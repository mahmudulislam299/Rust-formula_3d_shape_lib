//! # This Module for solid Shape
//! 
//! - This module take solid's length, width and height as input parameter
//! - This module gives output volume, surface area, summary

use super::common_formula::Formula;


// struct definiton
#[derive(Debug)]
pub struct Solid
{
	pub length: f32,
	pub width: f32,
	pub height: f32,
}

impl Solid
{
	/// create new solid instance
	/// # Examples
	///
	/// Basic usage:
	///
	/// ```
	/// use formula_3d_shape_lib::solid::Solid;
	/// use formula_3d_shape_lib::common_formula::Formula;
	/// let solid1 = Solid::new(4.0, 10.0,3.0);
	/// ```
	pub fn new(length:f32, width:f32, height:f32) -> Solid
	{ 
		Solid {
		length,
		height,
		width,
		} 
	}
}



impl Formula for Solid
{
	/// this function return volume
	/// # Examples
	///
	/// Basic usage:
	///
	/// ```
	/// use formula_3d_shape_lib::solid::Solid;
	/// use formula_3d_shape_lib::common_formula::Formula;
	/// let solid1 = Solid::new(4.0, 10.0,3.0);
	/// solid1.get_volume();
	/// ```
	fn get_volume(&self) -> f32
	{
		return self.length * self.width * self.height;
	}

	/// This function return surface area
	/// # Examples
	///
	/// Basic usage:
	///
	/// ```
	/// use formula_3d_shape_lib::solid::Solid;
	/// use formula_3d_shape_lib::common_formula::Formula;
	/// let solid1 = Solid::new(4.0, 10.0,3.0);
	/// solid1.get_surface_area();
	/// ```
	fn get_surface_area(&self) -> f32
	{
		return 2.0*(self.length * self.width + self.width * self.height + self.height * self.length);
	}
}

impl Solid
{
	/// this function gives summary of solid
	/// # Examples
	///
	/// Basic usage:
	///
	/// ```
	/// use formula_3d_shape_lib::solid::Solid;
	/// use formula_3d_shape_lib::common_formula::Formula;
	/// let solid1 = Solid::new(4.0, 10.0,3.0);
	/// solid1.summary();
	/// ```
	pub fn summary(&self)
	{
		println!("summary:");
		println!("length: {}, width: {}, height: {}", self.length, self.width, self.height);
		println!("volume is {} and surface area is {}", self.get_volume(), self.get_surface_area());
	}
}
