//! # This Module for common formula for 3D shape
//! 


pub trait Formula
{
	fn get_volume(&self) -> f32;
	fn get_surface_area(&self) -> f32;
}