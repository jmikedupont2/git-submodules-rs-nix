// src/lattice_model.rs

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ValueType {
    Bit,
    ThreeValue,
    FiveValue,
    P7(u8),
    P11(u8),
    P13(u8),
    P17(u8),
    P19(u8),
}

impl ValueType {
    pub fn count(&self) -> u8 {
        match self {
            ValueType::Bit => 2,
            ValueType::ThreeValue => 3,
            ValueType::FiveValue => 5,
            ValueType::P7(val) => *val,
            ValueType::P11(val) => *val,
            ValueType::P13(val) => *val,
            ValueType::P17(val) => *val,
            ValueType::P19(val) => *val,
        }
    }

    pub fn zos_sequence() -> Vec<ValueType> {
        vec![
            ValueType::Bit,
            ValueType::ThreeValue,
            ValueType::FiveValue,
            ValueType::P7(7),
            ValueType::P11(11),
            ValueType::P13(13),
            ValueType::P17(17),
            ValueType::P19(19),
        ]
    }
}

pub trait HasValueCount {
    fn value_count() -> u8;
}

impl HasValueCount for bool {
    fn value_count() -> u8 {
        2
    }
}

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
}

#[derive(Debug, Clone)]
pub struct LatticeLayer<T: HasValueCount + std::fmt::Debug> {
    pub value_type: ValueType,
    pub instances: Vec<Instance<T>>,
}

impl<T: HasValueCount + std::fmt::Debug> LatticeLayer<T> {
    pub fn new(value_type: ValueType) -> Self {
        Self { value_type, instances: Vec::new() }
    }

    pub fn add_instance(&mut self, instance: Instance<T>) {
        assert_eq!(T::value_count(), self.value_type.count(),
                   "Instance unit value count must match layer's value type");
        self.instances.push(instance);
    }
}

pub trait LatticeLayerTrait: std::fmt::Debug {
    fn describe_layer(&self);
}

impl<T: HasValueCount + std::fmt::Debug + 'static> LatticeLayerTrait for LatticeLayer<T> {
    fn describe_layer(&self) {
        // In a real scenario, this would call a describe method on LatticeLayer
        // For now, we'll just print a placeholder.
        println!("Layer: {:?}", self.value_type);
    }
}

pub struct Lattice {
    pub name: String,
    pub layers: Vec<Box<dyn LatticeLayerTrait>>,
}

impl Lattice {
    pub fn new(name: &str) -> Self {
        Self { name: name.to_string(), layers: Vec::new() }
    }

    pub fn add_layer<T: HasValueCount + std::fmt::Debug + 'static>(&mut self, layer: LatticeLayer<T>) {
        self.layers.push(Box::new(layer));
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct WordPredicate(pub bool);

impl WordPredicate {
    pub fn new(value: bool) -> Self { WordPredicate(value) }
}

/// Represents a simple classifier based on predicate presence.
pub struct PredicateClassifier {
    target_predicates: Vec<String>,
}

impl PredicateClassifier {
    pub fn new(predicates: Vec<&str>) -> Self {
        Self { target_predicates: predicates.into_iter().map(|s| s.to_lowercase()).collect() }
    }

    /// Extracts WordPredicates from text based on the classifier's target predicates.
    pub fn extract_word_predicates(&self, text: &str) -> Vec<WordPredicate> {
        let lower_text = text.to_lowercase();
        self.target_predicates.iter()
            .map(|p| WordPredicate(lower_text.contains(p)))
            .collect()
    }
}
