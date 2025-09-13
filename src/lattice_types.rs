//! This program defines a conceptual "lattice of types" in Rust,
//! mirroring the abstract mathematical framework. It uses enums, structs,
//! traits, and generics to represent different value types and layers of complexity.

use std::fmt;

// --- 1. ValueType Enum: Represents the 'k' in k-value type ---
// This enum defines the discrete number of values a fundamental unit can have.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ValueType {
    Bit, // 2 values (0, 1)
    ThreeValue, // 3 values (e.g., 0, 1, 2 or Low, Mid, High)
    FiveValue,  // 5 values
    // ... and so on, up to 19-value type based on the Zos Seq primes.
    // For simplicity, we'll explicitly list a few and then generalize.
    PrimeValue(u8), // For other prime values from the Zos Seq
}

impl ValueType {
    pub fn count(&self) -> u8 {
        match self {
            ValueType::Bit => 2,
            ValueType::ThreeValue => 3,
            ValueType::FiveValue => 5,
            ValueType::PrimeValue(p) => *p,
        }
    }

    pub fn zos_sequence() -> Vec<ValueType> {
        vec![
            ValueType::Bit, // 2
            ValueType::ThreeValue, // 3
            ValueType::FiveValue, // 5
            ValueType::PrimeValue(7),
            ValueType::PrimeValue(11),
            ValueType::PrimeValue(13),
            ValueType::PrimeValue(17),
            ValueType::PrimeValue(19),
        ]
    }
}

// --- 2. HasValueCount Trait: Defines common behavior for types with a value count ---
// This trait allows us to work generically with different underlying value types.
pub trait HasValueCount {
    fn value_count() -> u8;
}

// Implement HasValueCount for bool (our 'bit' type)
impl HasValueCount for bool {
    fn value_count() -> u8 {
        2
    }
}

// Define a custom enum for a 3-value type
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ThreeValueUnit {
    Low,
    Mid,
    High,
}

impl HasValueCount for ThreeValueUnit {
    fn value_count() -> u8 {
        3
    }
}

// Define a generic struct for a 'k'-value unit (for primes > 5)
// This is a placeholder; in a real system, this might be a more complex enum or a numeric type.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct PrimeValueUnit(u8);

impl HasValueCount for PrimeValueUnit {
    fn value_count() -> u8 {
        // This would dynamically get the prime value from context or a lookup.
        // For this conceptual model, we'll assume it's set correctly.
        // This is a simplification for demonstration.
        0 // Placeholder, actual value depends on context
    }
}

// --- 3. Instance Struct: Represents an 'instance' of a specific bit/value size ---
// T is the underlying unit type (e.g., bool, ThreeValueUnit).
// This struct conceptually holds a sequence of these units, forming an n-gram.
#[derive(Debug, Clone)]
pub struct Instance<T: HasValueCount + fmt::Debug> {
    pub id: String,
    pub n_gram_size: u8, // The 'n' in n-gram, derived from Zos Seq primes
    pub units: Vec<T>,
}

impl<T: HasValueCount + fmt::Debug> Instance<T> {
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

// --- 4. LatticeLayer Struct: A collection of instances for a specific ValueType ---
#[derive(Debug, Clone)]
pub struct LatticeLayer<T: HasValueCount + fmt::Debug> {
    pub value_type: ValueType,
    pub instances: Vec<Instance<T>>,
}

impl<T: HasValueCount + fmt::Debug> LatticeLayer<T> {
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

// --- 5. Lattice Struct: The top-level structure holding all layers ---
#[derive(Debug, Clone)]
pub struct Lattice {
    pub name: String,
    // Using a Vec of `Box<dyn Any>` or `enum` to hold different generic types
    // is complex. For conceptual clarity, we'll use a simplified approach
    // or focus on describing the structure.
    // In a real system, this would likely involve trait objects or a more complex enum.
    pub layers: Vec<Box<dyn LatticeLayerTrait>>,
}

// A trait to allow storing different generic LatticeLayer types in a Vec.
pub trait LatticeLayerTrait: fmt::Debug {
    fn describe_layer(&self);
}

impl<T: HasValueCount + fmt::Debug + 'static> LatticeLayerTrait for LatticeLayer<T> {
    fn describe_layer(&self) {
        self.describe();
    }
}

impl Lattice {
    pub fn new(name: &str) -> Self {
        Self { name: name.to_string(), layers: Vec::new() }
    }

    pub fn add_layer<T: HasValueCount + fmt::Debug + 'static>(&mut self, layer: LatticeLayer<T>) {
        self.layers.push(Box::new(layer));
    }

    pub fn describe(&self) {
        println!("\n### Lattice Model: {} ###", self.name);
        for layer in &self.layers {
            layer.describe_layer();
        }
    }
}

// --- Main Function: Demonstrates the conceptual usage ---
fn main() {
    println!("Building a conceptual Lattice of Types...");

    let mut my_lattice = Lattice::new("Conceptual Knowledge Lattice");

    // Create Layer 1: Bit-based (2-value type)
    let mut bit_layer = LatticeLayer::<bool>::new(ValueType::Bit);
    bit_layer.add_instance(Instance::new("Model1_BitInstance_A", 2, vec![true, false]));
    bit_layer.add_instance(Instance::new("Model1_BitInstance_B", 3, vec![true, false, true]));
    my_lattice.add_layer(bit_layer);

    // Create Layer 2: Three-value type
    let mut three_value_layer = LatticeLayer::<ThreeValueUnit>::new(ValueType::ThreeValue);
    three_value_layer.add_instance(Instance::new("Model2_ThreeValueInstance_X", 2, vec![ThreeValueUnit::Low, ThreeValueUnit::High]));
    three_value_layer.add_instance(Instance::new("Model2_ThreeValueInstance_Y", 5, vec![ThreeValueUnit::Mid, ThreeValueUnit::Low, ThreeValueUnit::High, ThreeValueUnit::Mid, ThreeValueUnit::Low]));
    my_lattice.add_layer(three_value_layer);

    // Describe the entire lattice
    my_lattice.describe();

    println!("\nThis program conceptually demonstrates the structure of the Lattice Idea Framework in Rust.");
    println!("It shows how different 'k'-value types and n-gram sizes form a hierarchical lattice.");
}
