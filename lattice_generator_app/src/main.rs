use lattice_code_generator::{
    generate_value_type_enum,
    generate_has_value_count_trait,
    generate_has_value_count_impls,
    generate_instance_struct,
    generate_lattice_layer_struct,
    generate_lattice_struct,
};
use std::fs;
use std::io::Write;
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Starting code generation for Lattice structures...");

    let output_dir = Path::new("generated_lattice_code");
    fs::create_dir_all(output_dir)?;

    let primes = vec![2, 3, 5, 7, 11, 13, 17, 19];

    // Generate ValueType enum
    let value_type_enum_code = generate_value_type_enum(&primes);
    write_code_to_file(output_dir, "value_type.rs", &value_type_enum_code.to_string())?;

    // Generate HasValueCount trait
    let has_value_count_trait_code = generate_has_value_count_trait();
    write_code_to_file(output_dir, "has_value_count_trait.rs", &has_value_count_trait_code.to_string())?;

    // Generate HasValueCount impls
    let has_value_count_impls_code = generate_has_value_count_impls();
    write_code_to_file(output_dir, "has_value_count_impls.rs", &has_value_count_impls_code.to_string())?;

    // Generate Instance struct
    let instance_struct_code = generate_instance_struct();
    write_code_to_file(output_dir, "instance_struct.rs", &instance_struct_code.to_string())?;

    // Generate LatticeLayer struct
    let lattice_layer_struct_code = generate_lattice_layer_struct();
    write_code_to_file(output_dir, "lattice_layer_struct.rs", &lattice_layer_struct_code.to_string())?;

    // Generate Lattice struct
    let lattice_struct_code = generate_lattice_struct();
    write_code_to_file(output_dir, "lattice_struct.rs", &lattice_struct_code.to_string())?;

    println!("Code generation complete. Files written to: {:?}", output_dir);

    Ok(())
}

fn write_code_to_file(dir: &Path, filename: &str, code: &str) -> Result<(), Box<dyn std::error::Error>> {
    let file_path = dir.join(filename);
    let mut file = fs::File::create(&file_path)?;
    file.write_all(code.as_bytes())?;
    println!("  Generated: {:?}", file_path);
    Ok(())
}