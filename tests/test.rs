#[cfg(test)]
mod tests {
    use formula_3d_shape_lib::sphere::Sphere;
    use formula_3d_shape_lib::solid::Solid;
    use formula_3d_shape_lib::cylinder::Cylinder;
    use formula_3d_shape_lib::common_formula::Formula;

    #[test]
    fn sphere_test() {
        let sphere1 = Sphere::new(4.0);
        let volume = sphere1.get_volume();
        let surface = sphere1.get_surface_area();
        assert_eq!(volume, 268.08258);
        assert_eq!(surface, 201.06194);
    }


    #[test]
    fn cylinder_test() {
        let cylinder1 = Cylinder::new(4.0, 10.0);
        let volume = cylinder1.get_volume();
        let surface = cylinder1.get_surface_area();
        assert_eq!(volume, 502.65485);
        assert_eq!(surface, 351.8584);
    }


    #[test]
    fn solid1_test() {
        let solid1 = Solid::new(4.0, 10.0, 2.0);
        let volume = solid1.get_volume();
        let surface = solid1.get_surface_area();
        assert_eq!(volume, 80.0);
        assert_eq!(surface, 136.0);
    }
}
