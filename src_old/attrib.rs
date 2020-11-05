use std::marker::PhantomData;
use dcd::DCD;
use crate::data::DataPtr;
use itertools::Itertools;
use std::hash::{Hash, Hasher};
use crate::names::WithNames;
use crate::objects::{Identifiable, Identity};

pub trait Attribute {}

#[derive(Clone)]
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

pub trait Filter {
    type Entity;
    fn filter(&self, data: DataPtr, element: &Self::Entity) -> bool;
    fn execute(&mut self, data: DataPtr, vector: Vec<Self::Entity>) -> Vec<Self::Entity> {
        vector.into_iter()
            .filter(|e| self.filter(data.clone(), &e))
            .collect()
    }
}

pub mod sort {
    #[derive(Clone, Copy, Hash, Eq, PartialEq, PartialOrd, Ord)]
    pub enum Direction { Descending, Ascending }
    impl Direction {
        pub fn descending(&self) -> bool { match self { Direction::Descending => true, _ => false } }
        pub fn ascending(&self)  -> bool { match self { Direction::Ascending  => true, _ => false } }
    }
    pub struct Sorter<T> { vector: Vec<T>, reverse: bool }
    impl<T> Sorter<T> {
        pub fn from(vector: Vec<T>, direction: Direction) -> Self {
            Sorter { vector, reverse: direction.descending() }
        }
        pub fn new(vector: Vec<T>, reverse: bool) -> Self {
            Sorter { vector, reverse }
        }
        pub fn sort_by_key<F, O>(mut self, f: F) -> Vec<T> where F: FnMut(&T) -> O, O: Ord {
            self.vector.sort_by_key(f);
            if self.reverse { self.vector.reverse() }
            self.vector
        }
        pub fn sort_by<F>(mut self, f: F) -> Vec<T> where F: FnMut(&T, &T) -> std::cmp::Ordering {
            self.vector.sort_by(f);
            if self.reverse { self.vector.reverse() }
            self.vector
        }
    }
}

pub trait Sort<T> {
    fn execute(&mut self, data: DataPtr, vector: Vec<T>, direction: sort::Direction) -> Vec<T>;
}

pub trait Sample<T> {
    fn execute(&mut self, data: DataPtr, vector: Vec<T>) -> Vec<T> {
        self.make_selection(data, vector.into_iter())
    }
    fn make_selection(&mut self, data: DataPtr, iter: impl Iterator<Item=T>) -> Vec<T>;
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

pub trait OptionalAttribute {
    type Entity;
    fn unknown(&self, database: DataPtr, entity: &Self::Entity) -> bool;
}

pub trait ExistentialAttribute {
    type Entity;
    fn exists(&self, database: DataPtr, entity: &Self::Entity) -> bool;
}

pub trait NumericalAttribute {
    type Entity;
    type Number;
    fn calculate(&self, database: DataPtr, entity: &Self::Entity) -> Option<Self::Number>;
}

pub trait CollectionAttribute {
    type Entity;
    type Item;
    fn items(&self, database: DataPtr, entity: &Self::Entity) -> Vec<Self::Item>;
    fn len(&self, database: DataPtr, entity: &Self::Entity) -> usize;
    fn parent_len(&self, database: DataPtr, entity: &Self::Entity) -> usize { self.len(database, entity) }
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

    pub trait StringAttribute {
        type Entity;
        fn extract(&self, database: &dcd::DCD, project_id: &u64, commit_ids: &Vec<u64>) -> String;
    }

}

impl<C,E,T> ExistentialAttribute for C where C: CollectionAttribute<Entity=T,Item=E> {
    type Entity = T;

    fn exists(&self, database: DataPtr, entity: &Self::Entity) -> bool {
        self.len(database, entity) > 0
    }
}

// impl<A,T,N> Sort<N> for A where A: NumericalAttribute<Entity=T,Number=N> {
//     fn execute(&mut self, data: DataPtr, vector: Vec<N>) -> Vec<N> {
//         unimplemented!()
//     }
// }

pub struct ID<I: Identity, A>{ attribute: A, id: PhantomData<I> }

impl<I,A> ID<I,A> where I: Identity {
    pub fn with(attribute: A) -> Self {
        ID { attribute, id: PhantomData }
    }
}

impl<I,A> Attribute for ID<I,A> where I: Identity {}

impl<I,A,T,X> Select<T> for ID<I,A> where A: Select<T, Entity=X>, T: Identifiable<I>, I: Identity {
    type Entity = (I, X);
    fn select(&self, data: DataPtr, entity: T) -> Self::Entity {
        (entity.id(), self.attribute.select(data, entity))
    }
}