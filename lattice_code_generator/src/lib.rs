// This crate provides conceptual code generation utilities for the Lattice Idea Framework.
//! It demonstrates how `syn` and `quote` can be used to programmatically generate Rust code
//! for defining lattice structures, such as enums for value types, and structs for instances and layers.

use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;

/// Generates Rust code for the `ValueType` enum based on a list of prime numbers.
/// This enum represents the 'k' in k-value types (e.g., 2 for Bit, 3 for ThreeValue).
pub fn generate_value_type_enum(primes: &[u8]) -> TokenStream {
    let variants = primes.iter().map(|&p| {
        let name_str = match p {
            2 => "Bit".to_string(),
            3 => "ThreeValue".to_string(),
            5 => "FiveValue".to_string(),
            _ => format!("P{}", p),
        };
        let variant_ident = Ident::new(&name_str, Span::call_site());
        if p == 2 || p == 3 || p == 5 {
            quote! { #variant_ident }
        } else {
            quote! { #variant_ident(u8) }
        }
    });

    let count_matches = primes.iter().map(|&p| {
        let name_str = match p {
            2 => "Bit".to_string(),
            3 => "ThreeValue".to_string(),
            5 => "FiveValue".to_string(),
            _ => format!("P{}", p),
        };
        let variant_ident = Ident::new(&name_str, Span::call_site());
        if p == 2 || p == 3 || p == 5 {
            quote! { ValueType::#variant_ident => #p }
        } else {
            quote! { ValueType::#variant_ident(val) => *val }
        }
    });

    let zos_variants = primes.iter().map(|&p| {
        let name_str = match p {
            2 => "Bit".to_string(),
            3 => "ThreeValue".to_string(),
            5 => "FiveValue".to_string(),
            _ => format!("P{}", p),
        };
        let variant_ident = Ident::new(&name_str, Span::call_site());
        if p == 2 || p == 3 || p == 5 {
            quote! { ValueType::#variant_ident }
        } else {
            quote! { ValueType::#variant_ident(#p) }
        }
    });

    quote! {
        #[derive(Debug, PartialEq, Eq, Clone, Copy)]
        pub enum ValueType {
            #(#variants,)*
        }

        impl ValueType {
            pub fn count(&self) -> u8 {
                match self {
                    #(#count_matches,)*
                }
            }

            pub fn zos_sequence() -> Vec<ValueType> {
                vec![
                    #(#zos_variants,)*
                ]
            }
        }
    }
}

/// Generates Rust code for a generic `Instance` struct.
/// This struct represents an instance of a specific bit/value size (an n-gram).
pub fn generate_instance_struct() -> TokenStream {
    quote! {
        #[derive(Debug, Clone)]
        pub struct Instance<T: HasValueCount + std::fmt::Debug> {
            pub id: String,
            pub n_gram_size: u8,
            pub units: Vec<T>,
        }

        impl<T: HasValueCount + std::fmt::Debug> Instance<T> {
            pub fn new(id: &str, n_gram_size: u8, units: Vec<T>) -> Self {
                assert_eq!(units.len() as u8, n_gram_size, "Number of units must match n_gram_size");
                Self { id: id.to_string(), n_gram_size, units }
            }

            pub fn describe(&self) {
                println!("  Instance ID: {}", self.id);
                println!("    N-gram Size: {}", self.n_gram_size);
                println!("    Underlying Value Type Count: {}", T::value_count());
                println!("    Units: {:?}", self.units);
            }
        }
    }
}

/// Generates Rust code for a generic `LatticeLayer` struct.
/// This struct holds a collection of instances for a specific `ValueType`.
pub fn generate_lattice_layer_struct() -> TokenStream {
    quote! {
        #[derive(Debug, Clone)]
        pub struct LatticeLayer<T: HasValueCount + std::fmt::Debug> {
            pub value_type: ValueType,
            pub instances: Vec<T>,
        }

        impl<T: HasValueCount + std::fmt::Debug> LatticeLayer<T> {
            pub fn new(value_type: ValueType) -> Self {
                Self { value_type, instances: Vec::new() }
            }

            pub fn add_instance(&mut self, instance: Instance<T>) {
                assert_eq!(instance.units[0].value_count(), self.value_type.count(),
                           "Instance unit value count must match layer's value type");
                self.instances.push(instance);
            }

            pub fn describe(&self) {
                println!("\n--- Lattice Layer: {:?} (k={}) ---", self.value_type, self.value_type.count());
                for instance in &self.instances {
                    instance.describe();
                }
            }
        }
    }
}

/// Generates Rust code for the top-level `Lattice` struct.
/// This struct holds all layers of the lattice.
pub fn generate_lattice_struct() -> TokenStream {
    quote! {
        #[derive(Debug, Clone)]
        pub struct Lattice {
            pub name: String,
            pub layers: Vec<Box<dyn LatticeLayerTrait>>,
        }

        pub trait LatticeLayerTrait: std::fmt::Debug {
            fn describe_layer(&self);
        }

        impl<T: HasValueCount + std::fmt::Debug + 'static> LatticeLayerTrait for LatticeLayer<T> {
            fn describe_layer(&self) {
                self.describe();
            }
        }

        impl Lattice {
            pub fn new(name: &str) -> Self {
                Self { name: name.to_string(), layers: Vec::new() }
            }

            pub fn add_layer<T: HasValueCount + std::fmt::Debug + 'static>(&mut self, layer: LatticeLayer<T>) {
                self.layers.push(Box::new(layer));
            }

            pub fn describe(&self) {
                println!("\n### Lattice Model: {} ###", self.name);
                for layer in &self.layers {
                    layer.describe_layer();
                }
            }
        }
    }
}

/// Generates the `HasValueCount` trait.
pub fn generate_has_value_count_trait() -> TokenStream {
    quote! {
        pub trait HasValueCount {
            fn value_count() -> u8;
        }
    }
}

/// Generates implementations of `HasValueCount` for common types.
pub fn generate_has_value_count_impls() -> TokenStream {
    quote! {
        impl HasValueCount for bool {
            fn value_count() -> u8 {
                2
            }
        }

        // Example for a 3-value type (assuming it's defined elsewhere)
        // #[derive(Debug, PartialEq, Eq, Clone, Copy)]
        // pub enum ThreeValueUnit {
        //     Low,
        //     Mid,
        //     High,
        // }
        // impl HasValueCount for ThreeValueUnit {
        //     fn value_count() -> u8 { 3 }
        // }
    }
}

/// Main function to demonstrate code generation.
pub fn main() {
    let primes = vec![2, 3, 5, 7, 11, 13, 17, 19];

    println!("// --- Generated ValueType Enum ---");
    let value_type_enum_code = generate_value_type_enum(&primes);
    println!("{}", value_type_enum_code.to_string());

    println!("\n// --- Generated HasValueCount Trait ---");
    let has_value_count_trait_code = generate_has_value_count_trait();
    println!("{}", has_value_count_trait_code.to_string());

    println!("\n// --- Generated HasValueCount Impls ---");
    let has_value_count_impls_code = generate_has_value_count_impls();
    println!("{}", has_value_count_impls_code.to_string());

    println!("\n// --- Generated Instance Struct ---");
    let instance_struct_code = generate_instance_struct();
    println!("{}", instance_struct_code.to_string());

    println!("\n// --- Generated LatticeLayer Struct ---");
    let lattice_layer_struct_code = generate_lattice_layer_struct();
    println!("{}", lattice_layer_struct_code.to_string());

    println!("\n// --- Generated Lattice Struct ---");
    let lattice_struct_code = generate_lattice_struct();
    println!("{}", lattice_struct_code.to_string());

    println!("\n// This output represents the Rust code that would be generated.");
    println!("// You would typically write this TokenStream to a file or use it in a procedural macro.");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_value_type_enum_basic() {
        let primes = vec![2, 3, 5];
        let generated_code = generate_value_type_enum(&primes).to_string();
        println!("Generated Code (Basic):\n{}", generated_code);
        let _expected_code = r###"#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ValueType {
    Bit,
    ThreeValue,
    FiveValue,
}
impl ValueType {
    pub fn count(&self) -> u8 {
        match self {
            ValueType::Bit => { /* compiler will fill this */ }
            ValueType::ThreeValue => { /* compiler will fill this */ }
            ValueType::FiveValue => { /* compiler will fill this */ }
        }
    }
    pub fn zos_sequence() -> Vec<ValueType> {
        vec![
            ValueType::Bit,
            ValueType::ThreeValue,
            ValueType::FiveValue,
        ]
    }
}"###;
        // Due to `quote!` macro expansion, the exact formatting might vary slightly.
        // We'll check for key components rather than exact string match for now.
        assert!(generated_code.contains("pub enum ValueType {"));
        assert!(generated_code.contains("Bit ,"));
        assert!(generated_code.contains("ThreeValue ,"));
        assert!(generated_code.contains("FiveValue,"));
        assert!(generated_code.contains("pub fn count(&self) -> u8 {"));
        assert!(generated_code.contains("ValueType::Bit => 2"));
        assert!(generated_code.contains("ValueType::ThreeValue => 3"));
        assert!(generated_code.contains("ValueType::FiveValue => 5"));
        assert!(generated_code.contains("pub fn zos_sequence() -> Vec<ValueType> {"));
        assert!(generated_code.contains("ValueType::Bit"));
        assert!(generated_code.contains("ValueType::ThreeValue"));
        assert!(generated_code.contains("ValueType::FiveValue"));
    }

    #[test]
    fn test_generate_value_type_enum_with_prime_value() {
        let primes = vec![2, 7];
        let generated_code = generate_value_type_enum(&primes).to_string();
        println!("Generated Code (Prime Value):\n{}", generated_code);
        assert!(generated_code.contains("pub enum ValueType {"));
        assert!(generated_code.contains("Bit ,"));
        assert!(generated_code.contains("P7(u8),"));
        assert!(generated_code.contains("ValueType::P7(val) => *val"));
        assert!(generated_code.contains("ValueType::P7(7u8)")); // Changed from ValueType::P7(7) to ValueType::P7(7u8)
    }
}