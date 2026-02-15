use crate::traits::Domain;

pub trait Registry {
    fn add_to_registry(&self, domain: Box<dyn Domain>);
}
