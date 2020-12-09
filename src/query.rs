use crate::attrib::*;
use crate::objects;
use crate::objects::Duration;
use crate::iterators::ItemWithData;

macro_rules! impl_attribute_definition {
    [$object:ty, $attribute:ident] => {
        #[derive(Eq, PartialEq, Copy, Clone, Hash)] pub struct $attribute;
        impl Attribute for $attribute { type Object = $object; }
    };
    [$object:ty, $attribute:ident] => {
        #[derive(Eq, PartialEq, Copy, Clone, Hash)] pub struct $attribute;
        impl Attribute for $attribute { type Object = $object; }
    }
}

macro_rules! impl_attribute_getter {
    [! $object:ty, $attribute:ident] => {
        impl<'a> Getter<'a> for $attribute {
            type IntoItem = Self::Object;
            fn get(&self, object: &ItemWithData<'a, Self::Object>) -> Self::IntoItem {
                object.item.clone()
            }
        }
        impl<'a> OptionGetter<'a> for $attribute {
            type IntoItem = Self::Object;
            fn get_opt(&self, object: &ItemWithData<'a, Self::Object>) -> Option<Self::IntoItem> {
                Some(object.item.clone())
            }
        }
    };
    [! $object:ty, $attribute:ident, $small_type:ty, $getter:ident] => {
        impl<'a> Getter<'a> for $attribute {
            type IntoItem = $small_type;
            fn get(&self, object: &ItemWithData<'a, Self::Object>) -> Self::IntoItem {
                object.$getter()
            }
        }
        impl<'a> OptionGetter<'a> for $attribute {
            type IntoItem = $small_type;
            fn get_opt(&self, object: &ItemWithData<'a, Self::Object>) -> Option<Self::IntoItem> {
                Some(object.$getter())
            }
        }
    };
    [? $object:ty, $attribute:ident, $small_type:ty, $getter:ident] => {
        impl<'a> Getter<'a> for $attribute {
            type IntoItem = Option<$small_type>;
            fn get(&self, object: &ItemWithData<'a, Self::Object>) -> Self::IntoItem {
                object.$getter()
            }
        }
        impl<'a> OptionGetter<'a> for $attribute {
            type IntoItem = $small_type;
            fn get_opt(&self, object: &ItemWithData<'a, Self::Object>) -> Option<Self::IntoItem> {
                object.$getter()
            }
        }
    };
    [!+ $object:ty, $attribute:ident, $small_type:ty, $getter:ident] => {
        impl<'a> Getter<'a> for $attribute {
            type IntoItem = ItemWithData<'a, $small_type>;
            fn get(&self, object: &ItemWithData<'a, Self::Object>) -> Self::IntoItem {
                object.$getter()
            }
        }
        impl<'a> OptionGetter<'a> for $attribute {
            type IntoItem = ItemWithData<'a, $small_type>;
            fn get_opt(&self, object: &ItemWithData<'a, Self::Object>) -> Option<Self::IntoItem> {
                Some(object.$getter())
            }
        }
    };
    [?+ $object:ty, $attribute:ident, $small_type:ty, $getter:ident] => {
        impl<'a> Getter<'a> for $attribute {
            type IntoItem = Option<ItemWithData<'a, $small_type>>;
            fn get(&self, object: &ItemWithData<'a, Self::Object>) -> Self::IntoItem {
                object.$getter()
            }
        }
        impl<'a> OptionGetter<'a> for $attribute {
            type IntoItem = ItemWithData<'a, $small_type>;
            fn get_opt(&self, object: &ItemWithData<'a, Self::Object>) -> Option<Self::IntoItem> {
                object.$getter()
            }
        }
    };
    [!+.. $object:ty, $attribute:ident, $small_type:ty, $getter:ident] => {
        impl<'a> Getter<'a> for $attribute {
            type IntoItem = Vec<ItemWithData<'a, $small_type>>;
            fn get(&self, object: &ItemWithData<'a, Self::Object>) -> Self::IntoItem {
                object.$getter()
            }
        }
        impl<'a> OptionGetter<'a> for $attribute {
            type IntoItem = Vec<ItemWithData<'a, $small_type>>;
            fn get_opt(&self, object: &ItemWithData<'a, Self::Object>) -> Option<Self::IntoItem> {
                Some(object.$getter())
            }
        }
    };
    [?+.. $object:ty, $attribute:ident, $small_type:ty, $getter:ident] => {
        impl<'a> Getter<'a> for $attribute {
            type IntoItem = Option<Vec<ItemWithData<'a, $small_type>>>;
            fn get(&self, object: &ItemWithData<'a, Self::Object>) -> Self::IntoItem {
                object.$getter()
            }
        }
        impl<'a> OptionGetter<'a> for $attribute {
            type IntoItem = Vec<ItemWithData<'a, $small_type>>;
            fn get_opt(&self, object: &ItemWithData<'a, Self::Object>) -> Option<Self::IntoItem> {
                object.$getter()
            }
        }
    };
}

macro_rules! impl_attribute_count {
    [! $object:ty, $attribute:ident, $counter:ident] => {
        impl<'a> Countable<'a> for $attribute {
            fn count(&self, object: &ItemWithData<'a, Self::Object>) -> usize {
                object.$counter()
            }
        }
        impl<'a> OptionCountable<'a> for $attribute {
            fn count(&self, object: &ItemWithData<'a, Self::Object>) -> Option<usize> {
                Some(object.$counter())
            }
        }
    };
    [? $object:ty, $attribute:ident, $counter:ident] => {
        impl<'a> OptionCountable<'a> for $attribute {
            fn count(&self, object: &ItemWithData<'a, Self::Object>) -> Option<usize> {
                object.$counter()
            }
        }
    }
}

macro_rules! impl_attribute_filter {
    [$object:ty, $attribute:ident] => {
        impl<'a> Filter<'a> for $attribute {
            type Item = $object;
            fn accept(&self, item_with_data: &ItemWithData<'a, Self::Item>) -> bool {
                self.get(item_with_data).unwrap_or(false)
            }
        }
    }
}

macro_rules! impl_attribute {
    [! $object:ty, $attribute:ident] => {
        impl_attribute_definition![$object, $attribute];
        impl_attribute_getter![! $object, $attribute];
    };
    [! $object:ty, $attribute:ident, bool, $getter:ident] => {
        impl_attribute_definition![$object, $attribute];
        impl_attribute_getter![! $object, $attribute, bool, $getter];
        impl_attribute_filter![$object, $attribute];
    };
    [! $object:ty, $attribute:ident, $small_type:ty, $getter:ident] => {
        impl_attribute_definition![$object, $attribute];
        impl_attribute_getter![! $object, $attribute, $small_type, $getter];
    };
    [!+ $object:ty, $attribute:ident, $small_type:ty, $getter:ident] => {
        impl_attribute_definition![$object, $attribute];
        impl_attribute_getter![!+ $object, $attribute, $small_type, $getter];
    };
    [? $object:ty, $attribute:ident, bool, $getter:ident] => {
        impl_attribute_definition![$object, $attribute];
        impl_attribute_getter![? $object, $attribute, bool, $getter];
        impl_attribute_filter![$object, $attribute];
    };
    [? $object:ty, $attribute:ident, $small_type:ty, $getter:ident] => {
        impl_attribute_definition![$object, $attribute];
        impl_attribute_getter![? $object, $attribute, $small_type, $getter];
    };
    [?+ $object:ty, $attribute:ident, $small_type:ty, $getter:ident] => {
        impl_attribute_definition![$object, $attribute];
        impl_attribute_getter![?+ $object, $attribute, $small_type, $getter];
    };
    [!.. $object:ty, $attribute:ident, $small_type:ty, $getter:ident, $counter:ident] => {
        impl_attribute_definition![$object, $attribute];
        impl_attribute_getter![! $object, $attribute, Vec<$small_type>, $getter];
        impl_attribute_count![! $object, $attribute, $counter];
    };
    [!+.. $object:ty, $attribute:ident, $small_type:ty, $getter:ident, $counter:ident] => {
        impl_attribute_definition![$object, $attribute];
        impl_attribute_getter![!+.. $object, $attribute, $small_type, $getter];
        impl_attribute_count![! $object, $attribute, $counter];
    };
    [?.. $object:ty, $attribute:ident, $small_type:ty, $getter:ident, $counter:ident] => {
        impl_attribute_definition![$object, $attribute];
        impl_attribute_getter![? $object, $attribute, Vec<$small_type>, $getter];
        impl_attribute_count![? $object, $attribute, $counter];
    };
    [?+.. $object:ty, $attribute:ident, $small_type:ty, $getter:ident, $counter:ident] => {
        impl_attribute_definition![$object, $attribute];
        impl_attribute_getter![?+.. $object, $attribute, $small_type, $getter];
        impl_attribute_count![? $object, $attribute, $counter];
    };
}

pub mod project {
    use crate::query::*;
    impl_attribute![!     objects::Project, Itself];
    impl_attribute![!     objects::Project, Id, objects::ProjectId, id];
    impl_attribute![!     objects::Project, URL, String, url];
    impl_attribute![?     objects::Project, Issues, usize, issue_count];
    impl_attribute![?     objects::Project, BuggyIssues, usize, buggy_issue_count];
    impl_attribute![?     objects::Project, IsFork, bool, is_fork];
    impl_attribute![?     objects::Project, IsArchived, bool, is_archived];
    impl_attribute![?     objects::Project, IsDisabled, bool, is_disabled];
    impl_attribute![?     objects::Project, Stars, usize, star_count];
    impl_attribute![?     objects::Project, Watchers, usize, watcher_count];
    impl_attribute![?     objects::Project, Size, usize, size];
    impl_attribute![?     objects::Project, OpenIssues, usize, open_issue_count];
    impl_attribute![?     objects::Project, Forks, usize, fork_count];
    impl_attribute![?     objects::Project, Subscribers, usize, subscriber_count];
    impl_attribute![?     objects::Project, License, String, license];
    impl_attribute![?     objects::Project, Language, objects::Language, language];
    impl_attribute![?     objects::Project, Description, String, description];
    impl_attribute![?     objects::Project, Homepage, String, homepage];
    impl_attribute![?     objects::Project, HasIssues, bool, has_issues];
    impl_attribute![?     objects::Project, HasDownloads, bool, has_downloads];
    impl_attribute![?     objects::Project, HasWiki, bool, has_wiki];
    impl_attribute![?     objects::Project, HasPages, bool, has_pages];
    impl_attribute![?     objects::Project, Created, i64, created];
    impl_attribute![?     objects::Project, Updated, i64, updated];
    impl_attribute![?     objects::Project, Pushed, i64, pushed];
    impl_attribute![?     objects::Project, DefaultBranch, String, default_branch];
    impl_attribute![?     objects::Project, Age, Duration, lifetime];
    impl_attribute![?..   objects::Project, Heads, objects::Head, heads, head_count];
    impl_attribute![?+..  objects::Project, Commits, objects::Commit, commits_with_data, commit_count];
    impl_attribute![?+..  objects::Project, Authors, objects::User, authors_with_data, author_count];
    impl_attribute![?+..  objects::Project, Committers, objects::User, committers_with_data, committer_count];
    impl_attribute![?+..  objects::Project, Users, objects::User, users_with_data, user_count];
    impl_attribute![?..   objects::Project, Paths, objects::Path, paths, path_count];
    impl_attribute![?..   objects::Project, Snapshots, objects::Snapshot, snapshots, snapshot_count];
}

pub mod commit {
    use crate::query::*;
    impl_attribute![!    objects::Commit, Id, objects::CommitId, id];
    impl_attribute![!+   objects::Commit, Committer, objects::User, committer_with_data];
    impl_attribute![!+   objects::Commit, Author, objects::User, author_with_data];
    impl_attribute![?    objects::Commit, Hash, String, hash];
    impl_attribute![?    objects::Commit, Message, String, message];
    impl_attribute![?    objects::Commit, MessageLength, usize, message_length];
    impl_attribute![?    objects::Commit, AuthoredTimestamp, i64, author_timestamp];
    impl_attribute![?    objects::Commit, CommittedTimestamp, i64, committer_timestamp];
    impl_attribute![?..  objects::Commit, Paths, objects::Path, changed_paths, changed_path_count];
    impl_attribute![?..  objects::Commit, Snapshots, objects::Snapshot, changed_snapshots, changed_snapshot_count];
    impl_attribute![!+.. objects::Commit, Parents, objects::Commit, parents_with_data, parent_count];
}

pub mod head {
    use crate::query::*;
    impl_attribute![!   objects::Head, Name, String, name];
    impl_attribute![!   objects::Head, Commit, objects::Commit, commit];
}

pub mod user {
    use crate::query::*;
    impl_attribute![!    objects::User, Id, objects::UserId, id];
    impl_attribute![!    objects::User, Email, String, email];
    impl_attribute![?    objects::User, AuthorExperience, Duration, author_experience];
    impl_attribute![?    objects::User, CommitterExperience, Duration, committer_experience];
    impl_attribute![?    objects::User, Experience, Duration, experience];
    impl_attribute![?+.. objects::User, AuthoredCommits, objects::Commit, authored_commits_with_data, authored_commit_count];
    impl_attribute![?+.. objects::User, CommittedCommits, objects::Commit, committed_commits_with_data, committed_commit_count];
}

pub mod path {
    use crate::query::*;
    impl_attribute![!   objects::Path, Id, objects::PathId, id];
    impl_attribute![!   objects::Path, Location, String, location];
    impl_attribute![?   objects::Path, Language, objects::Language, language];
}

pub mod snapshot {
    use crate::query::*;
    impl_attribute![!   objects::Snapshot, Id, objects::SnapshotId, id];
    impl_attribute![!   objects::Snapshot, Bytes, Vec<u8>, raw_contents_owned];
    impl_attribute![!   objects::Snapshot, Contents, String, contents_owned];
}

pub mod require {
    use crate::query::*;
    use crate::iterators::ItemWithData;

    macro_rules! impl_comparison {
        ($name:ident, $trait_limit:ident, $comparator:ident, $default:expr) => {
            pub struct $name<A, N>(pub A, pub N) where A: Attribute; // + OptionGetter<'a, IntoItem=N>;
            impl<'a, A, N, T> Filter<'a> for $name<A, N> where A: OptionGetter<'a, IntoItem=N> + Attribute<Object=T>, N: $trait_limit {
                type Item = T;
                fn accept(&self, item_with_data: &ItemWithData<'a, Self::Item>) -> bool {
                    self.0.get_opt(item_with_data).map_or($default, |n| n.$comparator(&self.1))
                }
            }
        }
    }
                                               /* vs None */
    impl_comparison!(LessThan, PartialOrd, lt, false);
    impl_comparison!(AtMost,   PartialOrd, le, false);
    impl_comparison!(Equal,    Eq,         eq, false);
    impl_comparison!(AtLeast,  PartialOrd, ge, true);
    impl_comparison!(MoreThan, PartialOrd, gt, true);

    macro_rules! impl_binary {
        ($name:ident, $comparator:expr) => {
            pub struct $name<A, B>(pub A, pub B); // where A: Attribute, B: Attribute;
            impl<'a, A, B, T> Filter<'a> for $name<A, B> where A: Filter<'a, Item=T>, B: Filter<'a, Item=T> {
                type Item = T;
                fn accept(&self, item_with_data: &ItemWithData<'a, Self::Item>) -> bool {
                    $comparator(self.0.accept(item_with_data),
                                self.1.accept(item_with_data))
                }
            }
        }
    }

    impl_binary!(And, |a, b| a && b); // TODO Effectively does not short circuit.
    impl_binary!(Or,  |a, b| a || b);

    macro_rules! impl_unary {
        ($name:ident, $comparator:expr) => {
            pub struct $name<A>(pub A); // where A: Attribute;
            impl<'a, A, T> Filter<'a> for $name<A> where A: Filter<'a, Item=T> {
                type Item = T;
                fn accept(&self, item_with_data: &ItemWithData<'a, Self::Item>) -> bool {
                    $comparator(self.0.accept(item_with_data))
                }
            }
        }
    }

    impl_unary!(Not,  |a: bool| !a);

    macro_rules! impl_existential {
        ($name:ident, $method:ident) => {
            pub struct $name<A>(pub A) where A: Attribute; // + OptionGetter<'a>;
            impl<'a, A, T> Filter<'a> for $name<A> where A: OptionGetter<'a>, A: Attribute<Object=T> {
                type Item = T;
                fn accept(&self, item_with_data: &ItemWithData<'a, Self::Item>) -> bool {
                    self.0.get_opt(item_with_data).$method()
                }
            }
        }
    }

    impl_existential!(Exists,  is_some);
    impl_existential!(Missing, is_none);

    pub struct Same<'a, A>(pub A, pub &'a str) where A: OptionGetter<'a>;
    impl<'a, A, T> Filter<'a> for Same<'a, A> where A: OptionGetter<'a, IntoItem=String>, A: Attribute<Object=T> {
        type Item = T;
        fn accept(&self, item_with_data: &ItemWithData<'a, Self::Item>) -> bool {
            self.0.get_opt(item_with_data).map_or(false, |e| e.as_str() == self.1)
        }
    }

    pub struct Contains<'a, A>(pub A, pub &'a str) where A: OptionGetter<'a>;
    impl<'a, A, T> Filter<'a> for Contains<'a, A> where A: OptionGetter<'a, IntoItem=String>, A: Attribute<Object=T> {
        type Item = T;
        fn accept(&self, item_with_data: &ItemWithData<'a, Self::Item>) -> bool {
            self.0.get_opt(item_with_data).map_or(false, |e| e.contains(self.1))
        }
    }

    #[macro_export] macro_rules! regex { ($str:expr) => { regex::Regex::new($str).unwrap() }}
    pub struct Matches<A>(pub A, pub regex::Regex) where A: Attribute;
    impl<'a, A, T> Filter<'a> for  Matches<A> where A: OptionGetter<'a, IntoItem=String>, A: Attribute<Object=T> {
        type Item = T;
        fn accept(&self, item_with_data: &ItemWithData<'a, Self::Item>) -> bool {
            self.0.get_opt(item_with_data).map_or(false, |e| self.1.is_match(&e))
        }
    }

    // TODO any all
}

pub mod sample {
    use crate::attrib::{Sampler, Attribute, OptionGetter};
    use crate::iterators::ItemWithData;
    use rand_pcg::Pcg64Mcg;
    use rand::SeedableRng;
    use rand::seq::IteratorRandom;
    use std::hash::{Hash, Hasher};
    use std::collections::BTreeSet;
    use std::iter::FromIterator;
    use itertools::Itertools;
    // #[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)] pub struct Distinct<S, C>(pub S, pub C);


    #[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)] pub struct Top(pub usize);
    impl<'a, T> Sampler<'a, T> for Top {
        fn sample<I>(&self, iter: I) -> Vec<ItemWithData<'a, T>>
            where I: Iterator<Item=ItemWithData<'a, T>> {

            iter.take(self.0).collect()
        }
    }

    #[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Ord, PartialOrd)] pub struct Seed(pub u128);
    impl Seed {
        pub fn to_be_bytes(&self) -> [u8; 16] { self.0.to_be_bytes() }
        pub fn to_le_bytes(&self) -> [u8; 16] { self.0.to_le_bytes() }
    }

    #[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)] pub struct Random(pub usize, pub Seed);
    impl<'a, T> Sampler<'a, T> for Random {
        fn sample<I>(&self, iter: I) -> Vec<ItemWithData<'a, T>>
            where I: Iterator<Item=ItemWithData<'a, T>> {

            let mut rng = Pcg64Mcg::from_seed(self.1.to_be_bytes());
            iter.choose_multiple(&mut rng, self.0)
        }
    }

    pub trait SimilarityCriterion<'a> {
        type Item;
        type IntoItem;
        type Similarity: Similarity<Self::IntoItem>;
        fn from(&self, object: &ItemWithData<'a, Self::Item>) -> Self::Similarity;
    }
    pub trait Similarity<T>: Eq + Hash { }

    pub struct MinRatio<T> { min_ratio: f64, items: Option<BTreeSet<T>> }
    impl<T> Hash for MinRatio<T> {
        // Everything needs to be compared explicitly.
        fn hash<H: Hasher>(&self, state: &mut H) { state.write_u64(42) }
    }
    impl<T> Eq for MinRatio<T> where T: Ord {}
    impl<T> PartialEq for MinRatio<T> where T: Ord {
        fn eq(&self, other: &Self) -> bool {
            match (&self.items, &other.items) {
                (None, None) => true,
                (Some(me), Some(them)) => {
                    let mine: f64 = me.len() as f64;
                    let same: f64 = me.intersection(&them).count() as f64;
                    same / mine > self.min_ratio
                }
                _ => false,
            }
        }
    }
    impl<T> Similarity<T> for MinRatio<T> where T: Ord {}

    #[derive(Debug, Clone, Copy)] pub struct Ratio<A: Attribute>(pub A, pub f64);
    impl<'a, A, T, I> SimilarityCriterion<'a> for Ratio<A> where A: Attribute<Object=T> + OptionGetter<'a, IntoItem=Vec<I>>, I: Ord {
        type Item = T;
        type IntoItem = I;
        type Similarity = MinRatio<Self::IntoItem>;

        fn from(&self, object: &ItemWithData<'a, Self::Item>) -> Self::Similarity {
            let items = self.0.get_opt(object).map(|e| {
                BTreeSet::from_iter(e.into_iter())
            });
            MinRatio { min_ratio: self.1, items }
        }
    }

    //#[derive(Debug, Clone, Copy)] pub struct All<A: Attribute>(pub A, pub f64); Seems superfluous
    #[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)] pub struct Distinct<S, C>(pub S, pub C);
    impl<'a, T, S, C> Sampler<'a, T> for Distinct<S, C> where S: Sampler<'a, T>, C: SimilarityCriterion<'a, Item=T> {
        fn sample<I>(&self, iter: I) -> Vec<ItemWithData<'a, T>>
            where I: Iterator<Item=ItemWithData<'a, T>> {
            let filtered_iter = iter.unique_by(|object| {
                self.1.from(object)
            });
            self.0.sample(filtered_iter)
        }
    }
}

pub mod select {
    #[allow(unused_imports)] use crate::attrib::*;
    use crate::query::*;

    macro_rules! impl_select {
        ($n:ident, $($ti:ident -> $i:tt),+) => {
            pub struct $n<$($ti: Attribute,)+> ($(pub $ti,)+);
            impl<T, $($ti,)+> Attribute for $n<$($ti,)+>
                where $($ti: Attribute<Object=T>,)+ {
                type Object = T;
            }
            impl<'a, T, $($ti,)+> OptionGetter<'a> for $n<$($ti,)+>
                where $($ti: Attribute<Object=T> + OptionGetter<'a>,)+ {
                type IntoItem = ($(Option<$ti::IntoItem>,)+);
                fn get_opt(&self, object: &ItemWithData<'a, Self::Object>) -> Option<Self::IntoItem> {
                    Some(($(self.$i.get_opt(object),)+))
                }
            }
            impl<'a, T, $($ti,)+> Getter<'a> for $n<$($ti,)+>
                where $($ti: Attribute<Object=T> + Getter<'a>,)+ {
                type IntoItem = ($($ti::IntoItem,)+);

                fn get(&self, object: &ItemWithData<'a, Self::Object>) -> Self::IntoItem {
                    ($(self.$i.get(object),)+)
                }
            }
        }
    }

    impl_select!(Select1,  Ta -> 0);
    impl_select!(Select2,  Ta -> 0, Tb -> 1);
    impl_select!(Select3,  Ta -> 0, Tb -> 1, Tc -> 2);
    impl_select!(Select4,  Ta -> 0, Tb -> 1, Tc -> 2, Td -> 3);
    impl_select!(Select5,  Ta -> 0, Tb -> 1, Tc -> 2, Td -> 3, Te -> 4);
    impl_select!(Select6,  Ta -> 0, Tb -> 1, Tc -> 2, Td -> 3, Te -> 4, Tf -> 5);
    impl_select!(Select7,  Ta -> 0, Tb -> 1, Tc -> 2, Td -> 3, Te -> 4, Tf -> 5, Tg -> 6);
    impl_select!(Select8,  Ta -> 0, Tb -> 1, Tc -> 2, Td -> 3, Te -> 4, Tf -> 5, Tg -> 6, Th -> 7);
    impl_select!(Select9,  Ta -> 0, Tb -> 1, Tc -> 2, Td -> 3, Te -> 4, Tf -> 5, Tg -> 6, Th -> 7, Ti -> 8);
    impl_select!(Select10, Ta -> 0, Tb -> 1, Tc -> 2, Td -> 3, Te -> 4, Tf -> 5, Tg -> 6, Th -> 7, Ti -> 8, Tj -> 9);
}

pub mod stats {
    #[allow(unused_imports)] use crate::attrib::*;
    use crate::query::*;
    use crate::fraction::*;

    use std::iter::Sum;
    use itertools::Itertools;

    pub struct Count<A: Attribute>(pub A);
    impl<A, T> Attribute for Count<A> where A: Attribute<Object=T> {
        type Object = T;
    }
    impl<'a, A, T> Getter<'a> for Count<A> where A: Attribute<Object=T> + OptionCountable<'a> {
        type IntoItem = usize;
        fn get(&self, object: &ItemWithData<'a, Self::Object>) -> Self::IntoItem {
            self.0.count(object).unwrap_or(0)
        }
    }
    impl<'a, A, T> OptionGetter<'a> for Count<A> where A: Attribute<Object=T> + OptionCountable<'a> {
        type IntoItem = usize;
        fn get_opt(&self, object: &ItemWithData<'a, Self::Object>) -> Option<Self::IntoItem> {
            self.0.count(object)
        }
    }

    // TODO bucket
    pub struct Bin;
    pub struct Bucket;

    trait CalculateStat<N, T>{ fn calculate(vector: Vec<N>) -> T; }
    macro_rules! impl_calculator {
        ($name:ident -> $result:ty where N: $($requirements:path),+; $calculate:item) => {
            pub struct $name<A: Attribute>(pub A);
            impl<A, T> Attribute for $name<A> where A: Attribute<Object=T> {
                type Object = T;
            }
            impl<'a, A, N, T> Getter<'a> for $name<A>
                where A: Attribute<Object=T> + OptionGetter<'a, IntoItem=Vec<N>>, N: $($requirements +)+ {
                type IntoItem = Option<$result>;
                fn get(&self, object: &ItemWithData<'a, Self::Object>) -> Self::IntoItem {
                    self.0.get_opt(object).map(|object| Self::calculate(object)).flatten()
                }
            }
            impl<'a, A, N, T> OptionGetter<'a> for $name<A>
                where A: Attribute<Object=T> + OptionGetter<'a, IntoItem=Vec<N>>, N: $($requirements +)+  { //$n: $(as_item!($requirements) +)+ {
                type IntoItem = $result;
                fn get_opt(&self, object: &ItemWithData<'a, Self::Object>) -> Option<Self::IntoItem> {
                    self.0.get_opt(object).map(|object| Self::calculate(object)).flatten()
                }
            }
            impl<A, N, T> CalculateStat<N, Option<$result>> for $name<A> where A: Attribute<Object=T>, N: $($requirements +)+  {
                $calculate
            }
        }
    }
    //
    // trait Unwrap<T,I> { fn unwrap(&self) -> I; }
    // impl Unwrap<usize, f64> for std::result::Result<f64, <usize as TryInto<f64>>::Err> {
    //     fn unwrap(&self) -> f64 {
    //         self.unwrap()
    //     }
    // }

    //TODO min_by/max_by/minmax_by
    impl_calculator!(Min -> N where N: Ord, Clone;
        fn calculate(vector: Vec<N>) -> Option<N> { vector.into_iter().min() }
    );
    impl_calculator!(Max -> N where N: Ord, Clone;
        fn calculate(vector: Vec<N>) -> Option<N> { vector.into_iter().max() }
    );
    impl_calculator!(MinMax -> (N, N) where N: Ord, Clone;
        fn calculate(vector: Vec<N>) -> Option<(N,N)> { vector.into_iter().minmax().into_option() }
    );
    impl_calculator!(Mean -> Fraction<N> where N: Sum;
        fn calculate(vector: Vec<N>) -> Option<Fraction<N>> {
            let length = vector.len();
            let sum = vector.into_iter().sum::<N>();
            if length == 0 {
                None
            } else {
                Some(Fraction::new(sum, length))
            }
        }
    );
    impl_calculator!(Median -> Fraction<N> where N: Ord, Clone, Sum;
        fn calculate(mut items: Vec<N>) -> Option<Fraction<N>> {
            items.sort();
            let length = items.len();
            if length == 0 {
                None
            } else {
                let value: Fraction<N> =
                    if length == 1 {
                        Fraction::new(items[0].clone(), 1)
                    } else if length % 2 != 0usize {
                        Fraction::new(items[length / 2].clone(), 1)
                    } else {
                        let left: N = items[(length / 2) - 1].clone();
                        let right: N = items[(length / 2)].clone();
                        Fraction::new(vec![left, right].into_iter().sum(), 2)
                    };
                Some(value)
            }
        }
    );

    pub struct Ratio<A: Attribute<Object=T>, P: Attribute<Object=T>, T>(pub A, pub P);
    impl<A, P, T> Attribute for Ratio<A, P, T>
        where A: Attribute<Object=T>,
              P: Attribute<Object=T> {

        type Object = T;
    }
    impl<'a, A, P, T> OptionGetter<'a> for Ratio<A, P, T>
        where A: Attribute<Object=T> + OptionCountable<'a>,
              P: Attribute<Object=T> + OptionCountable<'a> {
        type IntoItem = Fraction<usize>;
        fn get_opt(&self, object: &ItemWithData<'a, Self::Object>) -> Option<Self::IntoItem> {
            match (self.0.count(object), self.1.count(object)) {
                (Some(n), Some(m)) => Some(Fraction::new(n, m)),
                _ => None,
            }
        }
    }

    impl<'a, A, P, T> Getter<'a> for Ratio<A, P, T>
        where A: Attribute<Object=T> + OptionCountable<'a>,
              P: Attribute<Object=T> + OptionCountable<'a> {
        type IntoItem = Option<Fraction<usize>>;
        fn get(&self, object: &ItemWithData<'a, Self::Object>) -> Self::IntoItem {
            match (self.0.count(object), self.1.count(object)) {
                (Some(n), Some(m)) => Some(Fraction::new(n, m)),
                _ => None,
            }
        }
    }
}

pub mod get {
    use crate::attrib::*;
    use crate::iterators::ItemWithData;

    // pub struct From<O: Attribute, A: Attribute> (pub O, pub A);
    // impl<'a, O, A, T, I> Attribute for From<O, A>
    //     where O: Attribute<Object=T> + OptionGetter<'a, IntoItem=I>, A: Attribute<Object=I> {
    //     type Object = T;
    // }
    // impl<'a, O, A, T, I, E> Getter<'a> for From<O, A>
    //      where O: Attribute<Object=T> + OptionGetter<'a, IntoItem=I>,
    //            A: Attribute<Object=I> + Getter<'a, IntoItem=E> {
    //      type IntoItem = Option<E>;
    //      fn get(&self, object: &ItemWithData<'a, Self::Object>) -> Self::IntoItem {
    //          self.0.get_opt_with_data(object).map(|object| self.1.get(&object))
    //      }
    // }
    // impl<'a, O, A, T, I, E> OptionGetter<'a> for From<O, A>
    //     where O: Attribute<Object=T> + OptionGetter<'a, IntoItem=I>,
    //           A: Attribute<Object=I> + OptionGetter<'a, IntoItem=E> {
    //     type IntoItem = E;
    //     fn get_opt(&self, object: &ItemWithData<'a, Self::Object>) -> Option<Self::IntoItem> {
    //         self.0.get_opt_with_data(object).map(|object| self.1.get_opt(&object)).flatten()
    //     }
    // }

    pub struct FromEach<O: Attribute, A: Attribute> (pub O, pub A);
    impl<'a, O, A, T> Attribute for FromEach<O, A>
        where O: Attribute<Object=T> /*+ OptionGetter<'a, IntoItem=Vec<I>>)*/, A: Attribute { //<Object=I>*/ {
        type Object = T;
    }
    impl<'a, O, A, T, I, E> Getter<'a> for FromEach<O, A>
        where O: Attribute<Object=T> + OptionGetter<'a, IntoItem=Vec<ItemWithData<'a, I>>>,
              A: Attribute<Object=I> + Getter<'a, IntoItem=E> {
        type IntoItem = Option<Vec<E>>;
        fn get(&self, object: &ItemWithData<'a, Self::Object>) -> Self::IntoItem {
            self.0.get_opt(object).map(|v| {
                v.iter().map(|object| { self.1.get(object) }).collect()
            })
        }
    }
    impl<'a, O, A, T, I, E> OptionGetter<'a> for FromEach<O, A>
        where O: Attribute<Object=T> + OptionGetter<'a, IntoItem=Vec<ItemWithData<'a, I>>>,
              A: Attribute<Object=I> + OptionGetter<'a, IntoItem=E> {
        type IntoItem = Vec<E>;
        fn get_opt(&self, object: &ItemWithData<'a, Self::Object>) -> Option<Self::IntoItem> {
            self.0.get_opt(object).map(|v| {
                v.iter().flat_map(|object| { self.1.get_opt(object) }).collect()
            })
        }
    }

    // pub struct FromEachWith<O: Attribute, A: Attribute, P: Filter> (pub O, pub A, pub P);
    // impl<O, A, P, T, I> Attribute for FromEachWith<O, A, P>
    //     where O: Attribute<Object=T> + OptionGetter<'a, IntoItem=Vec<I>>,
    //           P: Filter<Item=I>,
    //           A: Attribute<Object=I> {
    //     type Object = T;
    // }
    // impl<'a, O, A, P, T, I, E> OptionGetter<'a> for FromEachWith<O, A, P>
    //     where O: Attribute<Object=T> + OptionGetter<'a, IntoItem=Vec<I>>,
    //           P: Filter<Item=I>,
    //           A: Attribute<Object=I> + OptionGetter<'a, IntoItem=E> {
    //     type IntoItem = Vec<E>;
    //     fn get_opt(object: &ItemWithData<Self::Object>) -> Option<Self::IntoItem> {
    //         O::get_opt_each_with_data(object).map(|v| {
    //             v.iter().
    //                 flat_map(|object| { A::get_opt(object) }).collect()
    //         })
    //     }
    // }
}

pub mod with {
    use crate::attrib::*;
    use crate::iterators::*;

    pub struct Requirement<A: Attribute, P> (pub A, pub P);
    impl<'a, A, P, T> Attribute for Requirement<A, P>
        where A: Attribute<Object=T> {
        type Object = T;
    }
    impl<'a, A, P, T, I> OptionGetter<'a> for Requirement<A, P>
        where A: Attribute<Object=T> + OptionGetter<'a, IntoItem=Vec<ItemWithData<'a, I>>>,
              P: Filter<'a, Item=I> {
        type IntoItem = Vec<ItemWithData<'a, I>>;
        fn get_opt(&self, object: &ItemWithData<'a, Self::Object>) -> Option<Self::IntoItem> {
            self.0.get_opt(object).map(|items| {
                 items.into_iter()
                     .filter(|item| self.1.accept(item))
                     .collect()
            })
        }
    }
    impl<'a, A, P, T, I> Getter<'a> for Requirement<A, P>
        where A: Attribute<Object=T> + OptionGetter<'a, IntoItem=Vec<ItemWithData<'a, I>>>,
              P: Filter<'a, Item=I> {
        type IntoItem = Option<Vec<ItemWithData<'a, I>>>;
        fn get(&self, object: &ItemWithData<'a, Self::Object>) -> Self::IntoItem {
            self.0.get_opt(object).map(|items|{
                items.into_iter()
                    .filter(|item| self.1.accept(item))
                    .collect()
            })
        }
    }
    impl<'a, A, P, T, I> Countable<'a> for Requirement<A, P>
        where A: Attribute<Object=T> + OptionGetter<'a, IntoItem=Vec<ItemWithData<'a, I>>>,
              P: Filter<'a, Item=I> {
        fn count(&self, object: &ItemWithData<'a, Self::Object>) -> usize {
            self.get_opt(object).map_or(0, |vector| vector.len())
            // Could potentially count straight from iter, but would have to reimplement all of
            // get_opt. It would save allocating the vector.
        }
    }
    impl<'a, A, P, T, I> OptionCountable<'a> for Requirement<A, P>
        where A: Attribute<Object=T> + OptionGetter<'a, IntoItem=Vec<ItemWithData<'a, I>>>,
              P: Filter<'a, Item=I> {
        fn count(&self, object: &ItemWithData<'a, Self::Object>) -> Option<usize> {
            self.get_opt(object).map(|vector| vector.len())
        }
    }
}