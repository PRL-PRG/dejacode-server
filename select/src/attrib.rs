use std::marker::PhantomData;
use dcd::DCD;
use crate::data::DataPtr;
use itertools::Itertools;
use std::hash::{Hash, Hasher};
use crate::names::WithNames;

pub trait Attribute {}

pub struct AttributeValue<A: Attribute, T> {
    pub value: T,
    attribute_type: PhantomData<A>,
}

impl<A, T> Hash for AttributeValue<A, T> where A: Attribute, T: Hash {
    fn hash<H: Hasher>(&self, state: &mut H) { self.value.hash(state) }
}

impl<A, T> PartialEq for AttributeValue<A, T> where A: Attribute, T: Eq {
    fn eq(&self, other: &Self) -> bool { self.value.eq(&other.value) }
}

impl<A, T> Eq for AttributeValue<A, T> where A: Attribute, T: Eq {}

impl<A, T> AttributeValue<A, T> where A: Attribute {
    pub fn new(_attribute: &A, value: T) -> AttributeValue<A, T> {
        AttributeValue { value, attribute_type: PhantomData }
    }
}

pub trait LoadFilter {
    fn filter(&self, database: &DCD, /*key: &Self::Key,*/ project_id: &dcd::ProjectId, commit_ids: &Vec<dcd::CommitId>) -> bool;
    fn clone_box(&self) -> Box<dyn LoadFilter>;
}

pub trait Group<T> {
    type Key;
    fn select(&self, data: DataPtr, element: &T) -> Self::Key;
    fn execute(&mut self, data: DataPtr, vector: Vec<T>) -> Vec<(Self::Key, Vec<T>)> where <Self as Group<T>>::Key: Hash + Eq {
        vector.into_iter()
            .map(|e| (self.select(data.clone(), &e), e))
            .into_group_map()
            .into_iter()
            .collect()
    }
}

pub trait Filter<T> {
    fn filter(&self, data: DataPtr, element: &T) -> bool;
    fn execute(&mut self, data: DataPtr, vector: Vec<T>) -> Vec<T> {
        vector.into_iter()
            .filter(|e| self.filter(data.clone(), &e))
            .collect()
    }
}

pub trait Sort<T> {
    fn execute(&mut self, data: DataPtr, vector: Vec<T>) -> Vec<T>;
}

pub trait Sample<T> {
    fn execute(&mut self, data: DataPtr, vector: Vec<T>) -> Vec<T>;
}

pub trait Select<T>: WithNames {
    type Entity; // TODO rename
    fn select(&self, data: DataPtr, project: T) -> Self::Entity;
    fn execute(&mut self, data: DataPtr, vector: Vec<T>) -> Vec<Self::Entity> {
        vector.into_iter()
            .map(|e| self.select(data.clone(), e))
            .collect()
    }
}

pub trait NumericalAttribute {
    type Entity;
    fn calculate(&self, database: DataPtr, entity: &Self::Entity) -> usize;
}

pub trait CollectionAttribute {
    type Entity;
    type Item;
    fn calculate(&self, database: DataPtr, entity: &Self::Entity) -> Vec<Self::Item>;
}

pub trait StringAttribute {
    type Entity;
    fn extract(&self, database: DataPtr, entity: &Self::Entity) -> String;
}

pub mod raw {
    pub trait NumericalAttribute {
        type Entity;
        fn calculate(&self, database: &dcd::DCD, project_id: &u64, commit_ids: &Vec<u64>) -> usize;
    }

    pub trait CollectionAttribute {
        type Entity;
        //fn calculate(&self, database: &dcd::DCD, project_id: &u64, commit_ids: &Vec<u64>) -> usize;
    }

    pub trait StringAttribute {
        type Entity;
        fn extract(&self, database: &dcd::DCD, project_id: &u64, commit_ids: &Vec<u64>) -> String;
    }

}