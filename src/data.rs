use std::collections::{BTreeMap, BTreeSet};
use std::cell::RefCell;
use std::marker::PhantomData;
use std::iter::FromIterator;

use itertools::{Itertools, MinMaxResult};

use dcd::DatastoreView;

use crate::objects::*;
use crate::piracy::*;
use crate::persistent::*;
use crate::iterators::*;
use crate::metadata::*;
use crate::log::*;
use crate::weights_and_measures::Weighed;
use crate::time::Duration;
use crate::csv::*;

// Internally Mutable Data
pub struct Database { data: RefCell<Data>, store: DatastoreView, log: Log }

// Constructors
impl Database {
    pub fn from_store<S>(store: DatastoreView, cache_dir: S, log: Log) -> Database where S: Into<String> {
        Database { data: RefCell::new(Data::new(cache_dir, &log)), store, log }
    }
}

// Prequincunx
impl Database {
    pub fn all_project_ids(&self) -> Vec<ProjectId> { self.data.borrow_mut().all_project_ids(&self.store)  }
    pub fn all_user_ids(&self)    -> Vec<UserId>    { self.data.borrow_mut().all_user_ids(&self.store)     }
    pub fn all_path_ids(&self)    -> Vec<PathId>    { self.data.borrow_mut().all_path_ids(&self.store)     }
    pub fn all_commit_ids(&self)  -> Vec<CommitId>  { self.data.borrow_mut().all_commit_ids(&self.store)   }
}

pub struct OptionIter<I> where I: Iterator {
    pub iter: Option<I>
}

impl<I> OptionIter<I> where I: Iterator {
    pub fn new() -> Self {
        OptionIter { iter: None }
    }
}

impl<I> Iterator for OptionIter<I> where I: Iterator {
    type Item = I::Item;
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.as_mut().map(|iter| iter.next()).flatten()
    }
}

// Quincunx
impl Database {
    pub fn projects(&self)  -> QuincunxIter<Project>  { QuincunxIter::<Project>::new(&self)  }
    pub fn commits(&self)   -> QuincunxIter<Commit>   { QuincunxIter::<Commit>::new(&self)   }
    pub fn users(&self)     -> QuincunxIter<User>     { QuincunxIter::<User>::new(&self)     }
    pub fn paths(&self)     -> QuincunxIter<Path>     { QuincunxIter::<Path>::new(&self)     }
}

// Uncached stuff
impl Database {
    pub fn snapshot(&self, id: &SnapshotId) -> Option<Snapshot> {
        self.store.content_data(id.into())
            .map(|content| Snapshot::new(id.clone(), content))
    }
    pub fn snapshots<'a>(&'a self) -> impl Iterator<Item=Snapshot> + 'a {
        LogIter::new(
            "reading snapshots",
            &self.log,Verbosity::Log,
            self.store.contents_data()
                .map(|(id, content)| {
                    Snapshot::new(SnapshotId::from(id), content)
                })
        )
    }
    pub fn snapshot_ids<'a>(&'a self) -> impl Iterator<Item=SnapshotId> + 'a {
        self.store.contents().map(|(id, _hash_id)| SnapshotId::from(id))
    }
    pub fn snapshots_with_data<'a>(&'a self) -> impl Iterator<Item=ItemWithData<'a, Snapshot>> + 'a {
        self.snapshots().attach_data_to_each(self)
    }
}

impl Database {
    pub fn project(&self, id: &ProjectId) -> Option<Project> {
        self.data.borrow_mut().project(&self.store, id)
    }
    pub fn project_issues(&self, id: &ProjectId) -> Option<usize> {
        self.data.borrow_mut().project_issues(&self.store, id)
    }
    pub fn project_buggy_issues(&self, id: &ProjectId) -> Option<usize> {
        self.data.borrow_mut().project_buggy_issues(&self.store, id)
    }
    pub fn project_is_fork(&self, id: &ProjectId) -> Option<bool> {
        self.data.borrow_mut().project_is_fork(&self.store, id)
    }
    pub fn project_is_archived(&self, id: &ProjectId) -> Option<bool> {
        self.data.borrow_mut().project_is_archived(&self.store, id)
    }
    pub fn project_is_disabled(&self, id: &ProjectId) -> Option<bool> {
        self.data.borrow_mut().project_is_disabled(&self.store, id)
    }
    pub fn project_star_gazer_count(&self, id: &ProjectId) -> Option<usize> {
        self.data.borrow_mut().project_star_gazer_count(&self.store, id)
    }
    pub fn project_watcher_count(&self, id: &ProjectId) -> Option<usize> {
        self.data.borrow_mut().project_watcher_count(&self.store, id)
    }
    pub fn project_size(&self, id: &ProjectId) -> Option<usize> {
        self.data.borrow_mut().project_size(&self.store, id)
    }
    pub fn project_open_issue_count(&self, id: &ProjectId) -> Option<usize> {
        self.data.borrow_mut().project_open_issue_count(&self.store, id)
    }
    pub fn project_fork_count(&self, id: &ProjectId) -> Option<usize> {
        self.data.borrow_mut().project_fork_count(&self.store, id)
    }
    pub fn project_subscriber_count(&self, id: &ProjectId) -> Option<usize> {
        self.data.borrow_mut().project_subscriber_count(&self.store, id)
    }
    pub fn project_license(&self, id: &ProjectId) -> Option<String> {
        self.data.borrow_mut().project_license(&self.store, id)
    }
    pub fn project_language(&self, id: &ProjectId) -> Option<Language> {
        self.data.borrow_mut().project_language(&self.store, id)
    }
    pub fn project_description(&self, id: &ProjectId) -> Option<String> {
        self.data.borrow_mut().project_description(&self.store, id)
    }
    pub fn project_homepage(&self, id: &ProjectId) -> Option<String> {
        self.data.borrow_mut().project_homepage(&self.store, id)
    }
    pub fn project_has_issues(&self, id: &ProjectId) -> Option<bool> {
        self.data.borrow_mut().project_has_issues(&self.store, id)
    }
    pub fn project_has_downloads(&self, id: &ProjectId) -> Option<bool> {
        self.data.borrow_mut().project_has_downloads(&self.store, id)
    }
    pub fn project_has_wiki(&self, id: &ProjectId) -> Option<bool> {
        self.data.borrow_mut().project_has_wiki(&self.store, id)
    }
    pub fn project_has_pages(&self, id: &ProjectId) -> Option<bool> {
        self.data.borrow_mut().project_has_pages(&self.store, id)
    }
    pub fn project_created(&self, id: &ProjectId) -> Option<i64> {
        self.data.borrow_mut().project_created(&self.store, id)
    }
    pub fn project_updated(&self, id: &ProjectId) -> Option<i64> {
        self.data.borrow_mut().project_updated(&self.store, id)
    }
    pub fn project_pushed(&self, id: &ProjectId) -> Option<i64> {
        self.data.borrow_mut().project_pushed(&self.store, id)
    }
    pub fn project_master(&self, id: &ProjectId) -> Option<String> {
        self.data.borrow_mut().project_master(&self.store, id)
    }
    pub fn project_url(&self, id: &ProjectId) -> Option<String> {
        self.data.borrow_mut().project_url(&self.store, id)
    }
    // pub fn project_head_ids(&self, id: &ProjectId) -> Option<Vec<(String, CommitId)>> {
    //     self.data.borrow_mut().project_head_ids(&self.store, id)
    // }
    pub fn project_heads(&self, id: &ProjectId) -> Option<Vec<Head>> {
        self.data.borrow_mut().project_heads(&self.store, id)
    }
    pub fn project_commit_ids(&self, id: &ProjectId) -> Option<Vec<CommitId>> {
        self.data.borrow_mut().project_commit_ids(&self.store, id).pirate()
    }
    pub fn project_commits(&self, id: &ProjectId) -> Option<Vec<Commit>> {
        self.data.borrow_mut().project_commits(&self.store, id)
    }
    pub fn project_commit_count(&self, id: &ProjectId) -> Option<usize> {
        self.data.borrow_mut().project_commit_count(&self.store, id)
    }
    pub fn project_author_ids(&self, id: &ProjectId) -> Option<Vec<UserId>> {
        self.data.borrow_mut().project_author_ids(&self.store, id).pirate()
    }
    pub fn project_authors(&self, id: &ProjectId) -> Option<Vec<User>> {
        self.data.borrow_mut().project_authors(&self.store, id)
    }
    pub fn project_author_count(&self, id: &ProjectId) -> Option<usize> {
        self.data.borrow_mut().project_author_count(&self.store, id)
    }
    pub fn project_path_ids(&self, id: &ProjectId) -> Option<Vec<PathId>> {
        self.data.borrow_mut().project_path_ids(&self.store, id).pirate()
    }
    pub fn project_paths(&self, id: &ProjectId) -> Option<Vec<Path>> {
        self.data.borrow_mut().project_paths(&self.store, id)
    }
    pub fn project_path_count(&self, id: &ProjectId) -> Option<usize> {
        self.data.borrow_mut().project_path_count(&self.store, id)
    }
    pub fn project_snapshot_ids(&self, id: &ProjectId) -> Option<Vec<SnapshotId>> {
        self.data.borrow_mut().project_snapshot_ids(&self.store, id).pirate()
    }
    pub fn project_snapshots(&self, id: &ProjectId) -> Option<Vec<Snapshot>> {
        self.project_snapshot_ids(id).map(|vector| {
            vector.into_iter().flat_map(|id| {
                self.store.content_data(id.into()).map(|content| {
                    Snapshot::new(id, content)
                })
            }).collect::<Vec<Snapshot>>()
        })
    }
    pub fn project_snapshot_count(&self, id: &ProjectId) -> Option<usize> {
        self.data.borrow_mut().project_snapshot_count(&self.store, id)
    }
    pub fn project_committer_ids(&self, id: &ProjectId) -> Option<Vec<UserId>> {
        self.data.borrow_mut().project_committer_ids(&self.store, id).pirate()
    }
    pub fn project_committers(&self, id: &ProjectId) -> Option<Vec<User>> {
        self.data.borrow_mut().project_committers(&self.store, id)
    }
    pub fn project_committer_count(&self, id: &ProjectId) -> Option<usize> {
        self.data.borrow_mut().project_committer_count(&self.store, id)
    }
    pub fn project_user_ids(&self, id: &ProjectId) -> Option<Vec<UserId>> {
        self.data.borrow_mut().project_user_ids(&self.store, id).pirate()
    }
    pub fn project_users(&self, id: &ProjectId) -> Option<Vec<User>> {
        self.data.borrow_mut().project_users(&self.store, id)
    }
    pub fn project_user_count(&self, id: &ProjectId) -> Option<usize> {
        self.data.borrow_mut().project_user_count(&self.store, id)
    }
    pub fn project_lifetime(&self, id: &ProjectId) -> Option<Duration> {
        self.data.borrow_mut().project_lifetime(&self.store, id)
    }
    pub fn user(&self, id: &UserId) -> Option<User> {
        self.data.borrow_mut().user(&self.store, id).pirate()
    }
    pub fn path(&self, id: &PathId) -> Option<Path> {
        self.data.borrow_mut().path(&self.store, id).pirate()
    }
    pub fn commit(&self, id: &CommitId) -> Option<Commit> {
        self.data.borrow_mut().commit(&self.store, id).pirate()
    }
    pub fn commit_hash(&self, id: &CommitId) -> Option<String> {
        self.data.borrow_mut().commit_hash(&self.store, id).pirate()
    }
    pub fn commit_message(&self, id: &CommitId) -> Option<String> {
        self.data.borrow_mut().commit_message(&self.store, id).pirate()
    }
    pub fn commit_author_timestamp(&self, id: &CommitId) -> Option<i64> {
        self.data.borrow_mut().commit_author_timestamp(&self.store, id)
    }
    pub fn commit_committer_timestamp(&self, id: &CommitId) -> Option<i64> {
        self.data.borrow_mut().commit_committer_timestamp(&self.store, id)
    }
    pub fn commit_changes(&self, id: &CommitId) -> Option<Vec<Change>> {
        self.data.borrow_mut().commit_changes(&self.store, id)
    }
    pub fn commit_changed_paths(&self, id: &CommitId) -> Option<Vec<Path>> {
        self.data.borrow_mut().commit_changed_paths(&self.store, id)
    }
    pub fn commit_change_count(&self, id: &CommitId) -> Option<usize> {
        self.data.borrow_mut().commit_change_count(&self.store, id)
    }
    pub fn commit_changed_path_count(&self, id: &CommitId) -> Option<usize> {
        self.data.borrow_mut().commit_changed_path_count(&self.store, id)
    }
    pub fn user_committed_commit_ids(&self, id: &UserId) -> Option<Vec<CommitId>> {
        self.data.borrow_mut().user_committed_commit_ids(&self.store, id).pirate()
    }
    pub fn user_authored_commits(&self, id: &UserId) -> Option<Vec<Commit>> {
        self.data.borrow_mut().user_authored_commits(&self.store, id)
    }
    pub fn user_authored_commit_ids(&self, id: &UserId) -> Option<Vec<CommitId>> {
        self.data.borrow_mut().user_authored_commit_ids(&self.store, id).pirate()
    }
    pub fn user_committed_experience(&self, id: &UserId) -> Option<Duration> {
        self.data.borrow_mut().user_committed_experience(&self.store, id)
    }
    pub fn user_author_experience(&self, id: &UserId) -> Option<Duration> {
        self.data.borrow_mut().user_author_experience(&self.store, id)
    }
    pub fn user_experience(&self, id: &UserId) -> Option<Duration> {
        self.data.borrow_mut().user_experience(&self.store, id)
    }
    pub fn user_committed_commit_count(&self, id: &UserId) -> Option<usize> {
        self.data.borrow_mut().user_committed_commit_count(&self.store, id)
    }
    pub fn user_authored_commit_count(&self, id: &UserId) -> Option<usize> {
        self.data.borrow_mut().user_authored_commit_count(&self.store, id)
    }
    pub fn user_committed_commits(&self, id: &UserId) -> Option<Vec<Commit>> {
        self.data.borrow_mut().user_committed_commits(&self.store, id)
    }
}

struct IdExtractor<Id: Identity + Persistent> { _type: PhantomData<Id> }
impl<Id> IdExtractor<Id> where Id: Identity + Persistent {
    pub fn _new() -> IdExtractor<Id> {
        IdExtractor { _type: PhantomData }
    }
}
impl<Id> VectorExtractor for IdExtractor<Id> where Id: Identity + Persistent {
    type Value = Id;
}
impl<Id> SingleVectorExtractor for IdExtractor<Id> where Id: Identity + Persistent  {
    type A = BTreeMap<Id, String>;
    fn extract(whatever: &Self::A) -> Vec<Self::Value> {
        whatever.keys().collect::<Vec<&Id>>().pirate()
    }
}

struct ProjectUrlExtractor;
impl MapExtractor for ProjectUrlExtractor {
    type Key = ProjectId;
    type Value = String;
}
impl SingleMapExtractor for ProjectUrlExtractor {
    type A = DatastoreView;
    fn extract(store: &Self::A) -> BTreeMap<Self::Key, Self::Value> {
        store.project_urls().map(|(project_id, url)| {
            (ProjectId::from(project_id), url)
        }).collect()
    }
}

struct ProjectHeadsExtractor;
impl MapExtractor for ProjectHeadsExtractor {
    type Key = ProjectId;
    type Value = Vec<Head>;
}
impl SingleMapExtractor for ProjectHeadsExtractor {
    type A = DatastoreView;
    fn extract(store: &Self::A) -> BTreeMap<Self::Key, Self::Value> {
        store.project_heads().map(|(project_id, heads)| {
            (ProjectId::from(project_id), heads.into_iter().map(|(name, commit_id)| {
                Head::new(name, CommitId::from(commit_id))
            }).collect())
        }).collect()
    }
}

struct ProjectSnapshotsExtractor {}
impl MapExtractor for ProjectSnapshotsExtractor {
    type Key = ProjectId;
    type Value = Vec<SnapshotId>;
}
impl DoubleMapExtractor for ProjectSnapshotsExtractor {
    type A = BTreeMap<ProjectId, Vec<CommitId>>;
    type B = BTreeMap<CommitId, Vec<ChangeTuple>>;

    fn extract(project_commit_ids: &Self::A, commit_change_ids: &Self::B) -> BTreeMap<Self::Key, Self::Value> {
        project_commit_ids.iter().map(|(project_id, commit_ids)| {
            let path_ids /* Iterator equivalent of Vec<Vec<PathId>>*/ =
                commit_ids.iter().flat_map(|commit_id| {
                    let path_ids_option =
                        commit_change_ids.get(commit_id).map(|changes| {
                            let vector: Vec<SnapshotId> =
                                changes.iter().flat_map(|change| {
                                    change.1/*snapshot_id()*/
                                }).collect();
                            vector
                        });
                    path_ids_option
                });
            (project_id.clone(), path_ids.flatten().unique().collect())
        }).collect()
    }
}

struct ProjectPathsExtractor {}
impl MapExtractor for ProjectPathsExtractor {
    type Key = ProjectId;
    type Value = Vec<PathId>;
}
impl DoubleMapExtractor for ProjectPathsExtractor {
    type A = BTreeMap<ProjectId, Vec<CommitId>>;
    type B = BTreeMap<CommitId, Vec<ChangeTuple>>;

    fn extract(project_commit_ids: &Self::A, commit_change_ids: &Self::B) -> BTreeMap<Self::Key, Self::Value> {
        project_commit_ids.iter().map(|(project_id, commit_ids)| {
            let path_ids /* Iterator equivalent of Vec<Vec<PathId>>*/ =
                commit_ids.iter().flat_map(|commit_id| {
                    let path_ids_option =
                        commit_change_ids.get(commit_id).map(|changes| {
                            let vector: Vec<PathId> =
                                changes.iter().map(|change| {
                                    change.0//path_id()
                                }).collect();
                            vector
                        });
                    path_ids_option
                });
            (project_id.clone(), path_ids.flatten().unique().collect())
        }).collect()
    }
}

struct ProjectUsersExtractor {}
impl MapExtractor for ProjectUsersExtractor {
    type Key = ProjectId;
    type Value = Vec<UserId>;
}
impl DoubleMapExtractor for ProjectUsersExtractor {
    type A = BTreeMap<ProjectId, Vec<UserId>>;
    type B = BTreeMap<ProjectId, Vec<UserId>>;
    fn extract(project_authors: &Self::A, project_committers: &Self::B) -> BTreeMap<Self::Key, Self::Value> {
        project_authors.iter().map(|(project_id, authors)| {
            let mut users: Vec<UserId> = vec![];
            let committers = project_committers.get(project_id);
            if let Some(committers) = committers {
                users.extend(committers.iter().map(|user_id| user_id.clone()));
            }
            users.extend(authors.iter().map(|user_id| user_id.clone()));
            (project_id.clone(), users.into_iter().unique().collect())
        }).collect()
    }
}

struct ProjectAuthorsExtractor {}
impl MapExtractor for ProjectAuthorsExtractor {
    type Key = ProjectId;
    type Value = Vec<UserId>;
}
impl DoubleMapExtractor for ProjectAuthorsExtractor {
    type A = BTreeMap<ProjectId, Vec<CommitId>>;
    type B = BTreeMap<CommitId, Commit>;
    fn extract(project_commits: &Self::A, commits: &Self::B) -> BTreeMap<Self::Key, Self::Value> {
        project_commits.iter().map(|(project_id, commit_ids)| {
            (project_id.clone(), commit_ids.iter().flat_map(|commit_id| {
                commits.get(commit_id).map(|c| c.author_id())
            }).unique().collect())
        }).collect()
    }
}

struct ProjectCommittersExtractor {}
impl MapExtractor for ProjectCommittersExtractor {
    type Key = ProjectId;
    type Value = Vec<UserId>;
}
impl DoubleMapExtractor for ProjectCommittersExtractor {
    type A = BTreeMap<ProjectId, Vec<CommitId>>;
    type B = BTreeMap<CommitId, Commit>;
    fn extract(project_commits: &Self::A, commits: &Self::B) -> BTreeMap<Self::Key, Self::Value> {
        project_commits.iter().map(|(project_id, commit_ids)| {
            (project_id.clone(), commit_ids.iter().flat_map(|commit_id| {
                commits.get(commit_id).map(|c| c.committer_id())
            }).unique().collect())
        }).collect()
    }
}

struct CountPerKeyExtractor<K: Clone + Ord + Persistent, V>(PhantomData<(K, V)>);
impl<K, V> MapExtractor for CountPerKeyExtractor<K, V> where K: Clone + Ord + Persistent + Weighed {
    type Key = K;
    type Value = usize;
}
impl<K, V> SingleMapExtractor for CountPerKeyExtractor<K, V> where K: Clone + Ord + Persistent + Weighed {
    type A = BTreeMap<K, Vec<V>>;

    fn extract(primary: &Self::A) -> BTreeMap<Self::Key, Self::Value> {
        primary.iter().map(|(key, value)| (key.clone(), value.len())).collect()
    }
}

struct ProjectCommitsExtractor {}
impl ProjectCommitsExtractor {
    fn commits_from_head(commits: &BTreeMap<CommitId, Commit>, head: &CommitId) -> BTreeSet<CommitId> {
        let mut commits_in_head: BTreeSet<CommitId> = BTreeSet::new();
        let mut stack = vec![head.clone()];
        let mut visited: BTreeSet<CommitId> = BTreeSet::new();
        while !stack.is_empty() {
            let commit_id = stack.pop().unwrap();
            if !visited.insert(commit_id) { continue } // If the set **did have** this value present, `false` is returned.
            commits_in_head.insert(commit_id);
            if let Some(commit) = commits.get(&commit_id) {// Potentially explosive?
                let parents = commit.parent_ids();
                stack.extend(parents)
            } else {
                eprintln!("WARNING: commit id {} was found as a parent of another commit, but it does not have a commit associated with it", commit_id)
            }
        }
        commits_in_head
    }
}
impl MapExtractor for ProjectCommitsExtractor {
    type Key = ProjectId;
    type Value = Vec<CommitId>;
}
impl DoubleMapExtractor for ProjectCommitsExtractor {
    type A = BTreeMap<ProjectId, Vec<Head>>;
    type B = BTreeMap<CommitId, Commit>;
    fn extract(heads: &Self::A, commits: &Self::B) -> BTreeMap<Self::Key, Self::Value> {
        heads.iter().map(|(project_id, heads)| {
            (project_id.clone(),
             heads.iter().flat_map(|head| {
                 Self::commits_from_head(commits, &head.commit_id())
             }).collect::<BTreeSet<CommitId>>())
        }).map(|(project_id, commits)| {
            (project_id, Vec::from_iter(commits.into_iter()))
        }).collect()
    }
}

struct ProjectLifetimesExtractor {}
impl MapExtractor for ProjectLifetimesExtractor {
    type Key = ProjectId;
    type Value = u64;
}
impl TripleMapExtractor for ProjectLifetimesExtractor {
    type A = BTreeMap<ProjectId, Vec<CommitId>>;
    type B = BTreeMap<CommitId, i64>;
    type C = BTreeMap<CommitId, i64>;
    fn extract(project_commits: &Self::A,
               authored_timestamps: &Self::B,
               committed_timestamps: &Self::B) -> BTreeMap<Self::Key, Self::Value> {

       project_commits.iter().flat_map(|(project_id, commit_ids)| {
           let min_max =
               commit_ids.iter()
                   .flat_map(|commit_id: &CommitId| {
                       let mut timestamps: Vec<i64> = Vec::new();
                       let author_timestamp = authored_timestamps.get(commit_id);
                       let committer_timestamp = committed_timestamps.get(commit_id);
                       if let Some(timestamp) = author_timestamp { timestamps.push(*timestamp) }
                       if let Some(timestamp) = committer_timestamp { timestamps.push(*timestamp) }
                       timestamps
                   })
                   .minmax();

           match min_max {
               MinMaxResult::NoElements => { None }
               MinMaxResult::OneElement(_) => { Some((project_id.clone(), 0)) }
               MinMaxResult::MinMax(min, max) => { Some((project_id.clone(), (max - min) as u64)) }
           }
       }).collect()
    }
}

struct UserExtractor {}
impl MapExtractor for UserExtractor {
    type Key = UserId;
    type Value = User;
}
impl SingleMapExtractor for UserExtractor {
    type A = DatastoreView;
    fn extract(store: &Self::A) -> BTreeMap<Self::Key, Self::Value> {
        store.users().map(|(id, email)| {
            (UserId::from(id), User::new(UserId::from(id), email))
        }).collect()
    }
}

struct UserAuthoredCommitsExtractor {}
impl MapExtractor for UserAuthoredCommitsExtractor {
    type Key = UserId;
    type Value = Vec<CommitId>;
}
impl SingleMapExtractor for UserAuthoredCommitsExtractor {
    type A = BTreeMap<CommitId, Commit>;
    fn extract(commits: &Self::A) -> BTreeMap<Self::Key, Self::Value> {
        commits.iter()
            .map(|(commit_id, commit)| {
                (commit.author_id().clone(), commit_id.clone(), )
            }).into_group_map()
            .into_iter()
            .collect()
    }
}

struct UserExperienceExtractor {}
impl MapExtractor for UserExperienceExtractor {
    type Key = UserId;
    type Value = u64;
}
impl DoubleMapExtractor for UserExperienceExtractor  {
    type A = BTreeMap<UserId, Vec<CommitId>>;
    type B = BTreeMap<CommitId, i64>;
    fn extract(user_commits: &Self::A, timestamps: &Self::B) -> BTreeMap<Self::Key, Self::Value> {
        user_commits.iter()
        .flat_map(|(user_id, commit_ids)| {
            let min_max = commit_ids.iter()
                .flat_map(|commit_id| {
                    timestamps.get(commit_id).pirate()
                })
                .minmax();

            match min_max {
                MinMaxResult::NoElements => None,
                MinMaxResult::OneElement(_) => Some((user_id.clone(), 0)),
                MinMaxResult::MinMax(min, max) => Some((user_id.clone(), (max - min) as u64)),
            }
        }).collect()
    }
}

struct CombinedUserExperienceExtractor {}
impl MapExtractor for CombinedUserExperienceExtractor {
    type Key = UserId;
    type Value = u64;
}
impl TripleMapExtractor for CombinedUserExperienceExtractor  {
    type A = BTreeMap<UserId, Vec<CommitId>>;
    type B = BTreeMap<CommitId, i64>;
    type C = BTreeMap<CommitId, i64>;
    fn extract(user_commits: &Self::A, authored_timestamps: &Self::B, committed_timestamps: &Self::C) -> BTreeMap<Self::Key, Self::Value> {
        user_commits.iter()
            .flat_map(|(user_id, commit_ids)| {
                let min_max = commit_ids.iter()
                    .flat_map(|commit_id| {
                        let mut timestamps: Vec<i64> = Vec::new();
                        let authored_timestamp = authored_timestamps.get(commit_id).pirate();
                        let committed_timestamp = committed_timestamps.get(commit_id).pirate();
                        if let Some(timestamp) = authored_timestamp { timestamps.push(timestamp) }
                        if let Some(timestamp) = committed_timestamp { timestamps.push(timestamp) }
                        timestamps
                    })
                    .minmax();

                match min_max {
                    MinMaxResult::NoElements => None,
                    MinMaxResult::OneElement(_) => Some((user_id.clone(), 0)),
                    MinMaxResult::MinMax(min, max) => Some((user_id.clone(), (max - min) as u64)),
                }
            }).collect()
    }
}

struct PathExtractor {}
impl MapExtractor for PathExtractor {
    type Key = PathId;
    type Value = Path;
}
impl SingleMapExtractor for PathExtractor {
    type A = DatastoreView;
    fn extract(store: &Self::A) -> BTreeMap<Self::Key, Self::Value> {
        store.paths().map(|(id, location)| {
            (PathId::from(id), Path::new(PathId::from(id), location))
        }).collect()
    }
}

struct SnapshotExtractor {}
impl MapExtractor for SnapshotExtractor {
    type Key = SnapshotId;
    type Value = Snapshot;
}
impl SingleMapExtractor for SnapshotExtractor {
    type A = DatastoreView;
    fn extract(store: &Self::A) -> BTreeMap<Self::Key, Self::Value> {
        store.contents_data().map(|(id, contents)| {
            (SnapshotId::from(id), Snapshot::new(SnapshotId::from(id), contents))
        }).collect()
    }
}

struct CommitExtractor {}
impl MapExtractor for CommitExtractor {
    type Key = CommitId;
    type Value = Commit;
}
impl SingleMapExtractor for CommitExtractor {
    type A = DatastoreView;
    fn extract(store: &Self::A) -> BTreeMap<Self::Key, Self::Value> {
        store.commits().map(|(id, commit)| {
            (CommitId::from(id), Commit::from((id, commit)))
        }).collect()
    }
}

struct CommitHashExtractor {}
impl MapExtractor for CommitHashExtractor {
    type Key = CommitId;
    type Value = String;
}
impl SingleMapExtractor for CommitHashExtractor {
    type A = DatastoreView;
    fn extract(store: &Self::A) -> BTreeMap<Self::Key, Self::Value> {
        store.commit_hashes().map(|(id, commit_hash)| {
            (CommitId::from(id), commit_hash.to_string())
        }).collect()
    }
}

struct CommitMessageExtractor {}
impl MapExtractor for CommitMessageExtractor {
    type Key = CommitId;
    type Value = String;
}
impl SingleMapExtractor for CommitMessageExtractor {
    type A = DatastoreView;
    fn extract(store: &Self::A) -> BTreeMap<Self::Key, Self::Value> {
        store.commits().map(|(id, commit)| {
            (CommitId::from(id), commit.message)
        }).collect() // TODO maybe return iter?
    }
}

struct CommitterTimestampExtractor {}
impl MapExtractor for CommitterTimestampExtractor {
    type Key = CommitId;
    type Value = i64;
}
impl SingleMapExtractor for CommitterTimestampExtractor {
    type A = DatastoreView;
    fn extract(store: &Self::A) -> BTreeMap<Self::Key, Self::Value> {
        store.commits().map(|(id, commit)| {
            (CommitId::from(id), commit.committer_time)
        }).collect() // TODO maybe return iter?
    }
}

pub type ChangeTuple = (PathId, Option<SnapshotId>);
struct CommitChangesExtractor {}
impl MapExtractor for CommitChangesExtractor {
    type Key = CommitId;
    type Value = Vec<ChangeTuple>;
}
impl SingleMapExtractor for CommitChangesExtractor {
    type A = DatastoreView;
    fn extract(store: &Self::A) -> BTreeMap<Self::Key, Self::Value> {
        let hash_id_to_content_id_map: BTreeMap<u64, u64> = store.contents()
            .map(|(content_id, hash_id)| (hash_id, content_id))
            .collect();

        store.commits().map(|(id, commit)| {
            let commit_id = CommitId::from(id);
            let changes = commit.changes.iter()
                .map(|(path_id, hash_id)| {
                    let snapshot_id = hash_id_to_content_id_map.get(hash_id)
                        .map(|content_id| SnapshotId::from(content_id));
                    (PathId::from(path_id), snapshot_id)
                }).collect::<Vec<ChangeTuple>>();
            (commit_id, changes)
        }).collect()
    }
}

struct AuthorTimestampExtractor {}
impl MapExtractor for AuthorTimestampExtractor {
    type Key = CommitId;
    type Value = i64;
}
impl SingleMapExtractor for AuthorTimestampExtractor {
    type A = DatastoreView;
    fn extract(store: &Self::A) -> BTreeMap<Self::Key, Self::Value> {
        store.commits().map(|(id, commit)| {
            (CommitId::from(id), commit.author_time)
        }).collect() // TODO maybe return iter?
    }
}

impl From<(u64, dcd::Commit)> for Commit {
    fn from((id, c): (u64, dcd::Commit)) -> Self {
        Commit {
            id: CommitId::from(id),
            committer: UserId::from(c.committer),
            author: UserId::from(c.author),
            parents: c.parents.into_iter().map(|id| CommitId::from(id)).collect(),
        }
    }
}

pub(crate) struct Data {
    project_metadata:            ProjectMetadataSource,
    project_urls:                PersistentMap<ProjectUrlExtractor>,
    project_heads:               PersistentMap<ProjectHeadsExtractor>,
    project_paths:               PersistentMap<ProjectPathsExtractor>,
    project_snapshots:           PersistentMap<ProjectSnapshotsExtractor>,
    project_users:               PersistentMap<ProjectUsersExtractor>,
    project_authors:             PersistentMap<ProjectAuthorsExtractor>,
    project_committers:          PersistentMap<ProjectCommittersExtractor>,
    project_commits:             PersistentMap<ProjectCommitsExtractor>,
    project_lifetimes:           PersistentMap<ProjectLifetimesExtractor>,

    project_path_count:          PersistentMap<CountPerKeyExtractor<ProjectId, PathId>>,
    project_snapshot_count:      PersistentMap<CountPerKeyExtractor<ProjectId, SnapshotId>>,
    project_user_count:          PersistentMap<CountPerKeyExtractor<ProjectId, UserId>>,
    project_author_count:        PersistentMap<CountPerKeyExtractor<ProjectId, UserId>>,
    project_committer_count:     PersistentMap<CountPerKeyExtractor<ProjectId, UserId>>,
    project_commit_count:        PersistentMap<CountPerKeyExtractor<ProjectId, CommitId>>,

    users:                       PersistentMap<UserExtractor>,
    user_authored_commits:       PersistentMap<UserAuthoredCommitsExtractor>,
    user_committed_commits:      PersistentMap<UserAuthoredCommitsExtractor>,
    user_author_experience:      PersistentMap<UserExperienceExtractor>,
    user_committer_experience:   PersistentMap<UserExperienceExtractor>,
    user_experience:             PersistentMap<CombinedUserExperienceExtractor>,

    user_authored_commit_count:  PersistentMap<CountPerKeyExtractor<UserId, CommitId>>,
    user_committed_commit_count: PersistentMap<CountPerKeyExtractor<UserId, CommitId>>,

    paths:                       PersistentMap<PathExtractor>,
    //snapshots:                   PersistentMap<SnapshotExtractor>,

    commits:                     PersistentMap<CommitExtractor>,
    commit_hashes:               PersistentMap<CommitHashExtractor>,
    commit_messages:             PersistentMap<CommitMessageExtractor>,
    commit_author_timestamps:    PersistentMap<AuthorTimestampExtractor>,
    commit_committer_timestamps: PersistentMap<CommitterTimestampExtractor>,
    commit_changes:              PersistentMap<CommitChangesExtractor>,

    commit_change_count:         PersistentMap<CountPerKeyExtractor<CommitId, ChangeTuple>>,

    // TODO frequency of commits/regularity of commits
    // TODO maybe some of these could be pre-cached all at once (eg all commit properties)
}

impl Data {
    pub fn new<S>(/*store: DatastoreView,*/ cache_dir: S, log: &Log) -> Data where S: Into<String> {
        let dir = cache_dir.into();
        Data {
            project_urls:                PersistentMap::new("project_urls",                log, dir.clone()).without_cache(),
            project_heads:               PersistentMap::new("project_heads",               log, dir.clone()),
            project_paths:               PersistentMap::new("project_paths",               log, dir.clone()),
            project_path_count:          PersistentMap::new("project_path_count",          log, dir.clone()),
            project_snapshots:           PersistentMap::new("project_snapshots",           log, dir.clone()),
            project_snapshot_count:      PersistentMap::new("project_snapshot_count",      log, dir.clone()),
            project_users:               PersistentMap::new("project_users",               log, dir.clone()),
            project_user_count:          PersistentMap::new("project_user_count",          log, dir.clone()),
            project_authors:             PersistentMap::new("project_authors",             log, dir.clone()),
            project_author_count:        PersistentMap::new("project_author_count",        log, dir.clone()),
            project_committers:          PersistentMap::new("project_committers",          log, dir.clone()),
            project_committer_count:     PersistentMap::new("project_committer_count",     log, dir.clone()),
            project_commits:             PersistentMap::new("project_commits",             log, dir.clone()),
            project_commit_count:        PersistentMap::new("project_commit_count",        log, dir.clone()),
            project_lifetimes:           PersistentMap::new("project_lifetimes",           log, dir.clone()),

            project_metadata:            ProjectMetadataSource::new("project",             log, dir.clone()),

            users:                       PersistentMap::new("users",                       log, dir.clone()).without_cache(),
            user_authored_commits:       PersistentMap::new("user_authored_commits",       log, dir.clone()),
            user_committed_commits:      PersistentMap::new("user_committed_commits",      log, dir.clone()),
            user_author_experience:      PersistentMap::new("user_author_experience",      log, dir.clone()),
            user_committer_experience:   PersistentMap::new("user_committer_experience",   log, dir.clone()),
            user_experience:             PersistentMap::new("user_experience",             log, dir.clone()),

            user_authored_commit_count:  PersistentMap::new("user_authored_commit_count",  log, dir.clone()),
            user_committed_commit_count: PersistentMap::new("user_committed_commit_count", log, dir.clone()),

            paths:                       PersistentMap::new("paths",                       log, dir.clone()).without_cache(),
            //snapshots:                   PersistentMap::new("snapshots",                   dir.clone()),

            commits:                     PersistentMap::new("commits",                     log, dir.clone()),
            commit_hashes:               PersistentMap::new("commit_hashes",               log, dir.clone()).without_cache(),
            commit_messages:             PersistentMap::new("commit_messages",             log, dir.clone()).without_cache(),
            commit_author_timestamps:    PersistentMap::new("commit_author_timestamps",    log, dir.clone()),
            commit_committer_timestamps: PersistentMap::new("commit_committer_timestamps", log, dir.clone()),
            commit_changes:              PersistentMap::new("commit_changes",              log, dir.clone()).without_cache(),
            commit_change_count:         PersistentMap::new("commit_change_count",         log, dir.clone()),
        }
    }
}

impl Data { // Prequincunx, sort of
    pub fn all_project_ids(&mut self, store: &DatastoreView) -> Vec<ProjectId> {
        self.smart_load_project_urls(store).keys().collect::<Vec<&ProjectId>>().pirate()
    }
    pub fn all_user_ids(&mut self, store: &DatastoreView) -> Vec<UserId> {
        self.smart_load_users(store).keys().collect::<Vec<&UserId>>().pirate()
    }
    pub fn all_path_ids(&mut self, store: &DatastoreView) -> Vec<PathId> {
        self.smart_load_paths(store).keys().collect::<Vec<&PathId>>().pirate()
    }
    pub fn all_commit_ids(&mut self, store: &DatastoreView) -> Vec<CommitId> {
        self.smart_load_commits(store).keys().collect::<Vec<&CommitId>>().pirate()
    }
}

impl Data { // Quincunx, sort of
    #[allow(dead_code)] pub fn projects<'a>(&'a mut self, store: &DatastoreView) -> impl Iterator<Item=Project> + 'a {
        self.smart_load_project_urls(store).iter().map(|(id, url)| {
            Project::new(id.clone(), url.clone())
        })
    }

    #[allow(dead_code)] pub fn users<'a>(&'a mut self, store: &DatastoreView) -> impl Iterator<Item=&'a User> + 'a {
        self.smart_load_users(store).iter().map(|(_, user)| user)
    }

    #[allow(dead_code)] pub fn paths<'a>(&'a mut self, store: &DatastoreView) -> impl Iterator<Item=&'a Path> + 'a {
        self.smart_load_paths(store).iter().map(|(_, path)| path)
    }

    #[allow(dead_code)] pub fn commits<'a>(&'a mut self, store: &DatastoreView) -> impl Iterator<Item=&'a Commit> + 'a {
        self.smart_load_commits(store).iter().map(|(_, commit)| commit)
    }
}

impl Data {
    pub fn project(&mut self, store: &DatastoreView, id: &ProjectId) -> Option<Project> {
        self.smart_load_project_urls(store).get(id)
            .map(|url| Project::new(id.clone(), url.clone()))
    }
    pub fn project_issues(&mut self, _store: &DatastoreView, _id: &ProjectId) -> Option<usize> {
        unimplemented!()
    }         // FIXME
    pub fn project_buggy_issues(&mut self, _store: &DatastoreView, _id: &ProjectId) -> Option<usize> {
        unimplemented!()
    }   // FIXME
    pub fn project_is_fork(&mut self, store: &DatastoreView, id: &ProjectId) -> Option<bool> {
        self.project_metadata.is_fork(store, id)
    }
    pub fn project_is_archived(&mut self, store: &DatastoreView, id: &ProjectId) -> Option<bool> {
        self.project_metadata.is_archived(store, id)
    }
    pub fn project_is_disabled(&mut self, store: &DatastoreView, id: &ProjectId) -> Option<bool> {
        self.project_metadata.is_disabled(store, id)
    }
    pub fn project_star_gazer_count(&mut self, store: &DatastoreView, id: &ProjectId) -> Option<usize> {
        self.project_metadata.star_gazers(store, id)
    }
    pub fn project_watcher_count(&mut self, store: &DatastoreView, id: &ProjectId) -> Option<usize> {
        self.project_metadata.watchers(store, id)
    }
    pub fn project_size(&mut self, store: &DatastoreView, id: &ProjectId) -> Option<usize> {
        self.project_metadata.size(store, id)
    }
    pub fn project_open_issue_count(&mut self, store: &DatastoreView, id: &ProjectId) -> Option<usize> {
        self.project_metadata.open_issues(store, id)
    }
    pub fn project_fork_count(&mut self, store: &DatastoreView, id: &ProjectId) -> Option<usize> {
        self.project_metadata.forks(store, id)
    }
    pub fn project_subscriber_count(&mut self, store: &DatastoreView, id: &ProjectId) -> Option<usize> {
        self.project_metadata.subscribers(store, id)
    }
    pub fn project_license(&mut self, store: &DatastoreView, id: &ProjectId) -> Option<String> {
        self.project_metadata.license(store, id)
    }
    pub fn project_language(&mut self, store: &DatastoreView, id: &ProjectId) -> Option<Language> {
        self.project_metadata.language(store, id)
    }
    pub fn project_description(&mut self, store: &DatastoreView, id: &ProjectId) -> Option<String> {
        self.project_metadata.description(store, id)
    }
    pub fn project_homepage(&mut self, store: &DatastoreView, id: &ProjectId) -> Option<String> {
        self.project_metadata.homepage(store, id)
    }
    pub fn project_has_issues(&mut self, store: &DatastoreView, id: &ProjectId) -> Option<bool> {
        self.project_metadata.has_issues(store, id)
    }
    pub fn project_has_downloads(&mut self, store: &DatastoreView, id: &ProjectId) -> Option<bool> {
        self.project_metadata.has_downloads(store, id)
    }
    pub fn project_has_wiki(&mut self, store: &DatastoreView, id: &ProjectId) -> Option<bool> {
        self.project_metadata.has_wiki(store, id)
    }
    pub fn project_has_pages(&mut self, store: &DatastoreView, id: &ProjectId) -> Option<bool> {
        self.project_metadata.has_pages(store, id)
    }
    pub fn project_created(&mut self, store: &DatastoreView, id: &ProjectId) -> Option<i64> {
        self.project_metadata.created(store, id)
    }
    pub fn project_updated(&mut self, store: &DatastoreView, id: &ProjectId) -> Option<i64> {
        self.project_metadata.updated(store, id)
    }
    pub fn project_pushed(&mut self, store: &DatastoreView, id: &ProjectId) -> Option<i64> {
        self.project_metadata.pushed(store, id)
    }
    pub fn project_master(&mut self, store: &DatastoreView,id: &ProjectId) -> Option<String> {
        self.project_metadata.master(store, id)
    }
    pub fn project_url(&mut self, store: &DatastoreView, id: &ProjectId) -> Option<String> {
        self.smart_load_project_urls(store).get(id).pirate()
    }
    pub fn project_heads(&mut self, store: &DatastoreView, id: &ProjectId) -> Option<Vec<Head>> {
        self.smart_load_project_heads(store).get(id).pirate()
    }
    // pub fn project_heads(&mut self, store: &DatastoreView, id: &ProjectId) -> Option<Vec<(String, Commit)>> {
    //     self.smart_load_project_heads(store).get(id).pirate().map(|v| {
    //         v.into_iter().flat_map(|(name, commit_id)| {
    //             self.commit(store, &commit_id).map(|commit| {
    //                 Head::new(name, commit.clone())
    //             })
    //         }).collect()
    //     })
    // }
    pub fn project_commit_ids(&mut self, store: &DatastoreView, id: &ProjectId) -> Option<&Vec<CommitId>> {
        self.smart_load_project_commits(store).get(id)
    }
    pub fn project_commits(&mut self, store: &DatastoreView, id: &ProjectId) -> Option<Vec<Commit>> {
        self.smart_load_project_commits(store).get(id).pirate().map(|ids| {
            ids.iter().flat_map(|id| self.commit(store, id).pirate()).collect()
            // FIXME issue warnings in situations like these (when self.commit(id) fails etc.)
        })
    }
    pub fn project_commit_count(&mut self, store: &DatastoreView, id: &ProjectId) -> Option<usize> {
        self.smart_load_project_commit_count(store).get(id).pirate()
    }
    pub fn project_path_ids(&mut self, store: &DatastoreView, id: &ProjectId) -> Option<&Vec<PathId>> {
        self.smart_load_project_paths(store).get(id)
    }
    pub fn project_paths(&mut self, store: &DatastoreView, id: &ProjectId) -> Option<Vec<Path>> {
        self.smart_load_project_paths(store).get(id).pirate().map(|ids| {
            ids.iter().flat_map(|id| self.path(store, id).pirate()).collect()
        })
    }
    pub fn project_path_count(&mut self, store: &DatastoreView, id: &ProjectId) -> Option<usize> {
        self.smart_load_project_path_count(store).get(id).pirate()
    }
    pub fn project_snapshot_ids(&mut self, store: &DatastoreView, id: &ProjectId) -> Option<&Vec<SnapshotId>> {
        self.smart_load_project_snapshots(store).get(id)
    }
    pub fn project_snapshot_count(&mut self, store: &DatastoreView, id: &ProjectId) -> Option<usize> {
        self.smart_load_project_snapshot_count(store).get(id).pirate()
    }
    pub fn project_author_ids(&mut self, store: &DatastoreView, id: &ProjectId) -> Option<&Vec<UserId>> {
        self.smart_load_project_authors(store).get(id)
    }
    pub fn project_authors(&mut self, store: &DatastoreView, id: &ProjectId) -> Option<Vec<User>> {
        self.smart_load_project_authors(store).get(id).pirate().map(|ids| {
            ids.iter().flat_map(|id| self.user(store, id).pirate()).collect()
        })
    }
    pub fn project_author_count(&mut self, store: &DatastoreView, id: &ProjectId) -> Option<usize> {
        self.smart_load_project_author_count(store).get(id).pirate()
    }
    pub fn project_committer_ids(&mut self, store: &DatastoreView, id: &ProjectId) -> Option<&Vec<UserId>> {
        self.smart_load_project_committers(store).get(id)
    }
    pub fn project_committers(&mut self, store: &DatastoreView, id: &ProjectId) -> Option<Vec<User>> {
        self.smart_load_project_committers(store).get(id).pirate().map(|ids| {
            ids.iter().flat_map(|id| self.user(store, id).pirate()).collect()
        })
    }
    pub fn project_committer_count(&mut self, store: &DatastoreView, id: &ProjectId) -> Option<usize> {
        self.smart_load_project_committer_count(store).get(id).pirate()
    }
    pub fn project_user_ids(&mut self, store: &DatastoreView, id: &ProjectId) -> Option<&Vec<UserId>> {
        self.smart_load_project_users(store).get(id)
    }
    pub fn project_users(&mut self, store: &DatastoreView, id: &ProjectId) -> Option<Vec<User>> {
        self.smart_load_project_users(store).get(id).pirate().map(|ids| {
            ids.iter().flat_map(|id| self.user(store, id).pirate()).collect()
        })
    }
    pub fn project_user_count(&mut self, store: &DatastoreView, id: &ProjectId) -> Option<usize> {
        self.smart_load_project_user_count(store).get(id).pirate()
    }
    pub fn project_lifetime(&mut self, store: &DatastoreView, id: &ProjectId) -> Option<Duration> {
        self.smart_load_project_lifetimes(store).get(id)
            .pirate()
            .map(|seconds| Duration::from(seconds))
    }
    pub fn user(&mut self, store: &DatastoreView, id: &UserId) -> Option<&User> {
        self.smart_load_users(store).get(id)
    }
    pub fn path(&mut self, store: &DatastoreView, id: &PathId) -> Option<&Path> {
        self.smart_load_paths(store).get(id)
    }
    pub fn commit(&mut self, store: &DatastoreView, id: &CommitId) -> Option<&Commit> {
        self.smart_load_commits(store).get(id)
    }
    pub fn commit_hash(&mut self, store: &DatastoreView, id: &CommitId) -> Option<&String> {
        self.smart_load_commit_hashes(store).get(id)
    }
    pub fn commit_message(&mut self, store: &DatastoreView, id: &CommitId) -> Option<&String> {
        self.smart_load_commit_messages(store).get(id)
    }
    pub fn commit_author_timestamp(&mut self, store: &DatastoreView, id: &CommitId) -> Option<i64> {
        self.smart_load_commit_author_timestamps(store).get(id).pirate()
    }
    pub fn commit_committer_timestamp(&mut self, store: &DatastoreView, id: &CommitId) -> Option<i64> {
        self.smart_load_commit_committer_timestamps(store).get(id).pirate()
    }
    pub fn commit_changes(&mut self, store: &DatastoreView, id: &CommitId) -> Option<Vec<Change>> {
        self.smart_load_commit_changes(store).get(id).map(|vector| {
            vector.iter().map(|(path_id, snapshot_id)| {
                Change::new(path_id.clone(), snapshot_id.clone())
            }).collect()
        })
    }
    pub fn commit_changed_paths(&mut self, store: &DatastoreView, id: &CommitId) -> Option<Vec<Path>> {
        self.smart_load_commit_changes(store).get(id).pirate().map(|ids| {
            ids.iter().flat_map(|change| self.path(store, &change.0/*path_id()*/).pirate()).collect()
        })
    }
    pub fn commit_change_count(&mut self, store: &DatastoreView, id: &CommitId) -> Option<usize> {
        self.smart_load_commit_change_count(store).get(id).pirate()
    }
    pub fn commit_changed_path_count(&mut self, store: &DatastoreView, id: &CommitId) -> Option<usize> {
        self.smart_load_commit_change_count(store).get(id).pirate()
    }
    pub fn user_committed_commit_ids(&mut self, store: &DatastoreView, id: &UserId) -> Option<&Vec<CommitId>> {
        self.smart_load_user_committed_commits(store).get(id)
    }
    pub fn user_authored_commits(&mut self, store: &DatastoreView, id: &UserId) -> Option<Vec<Commit>> {
        self.smart_load_user_authored_commits(store).get(id).pirate().map(|ids| {
            ids.iter().flat_map(|id| self.commit(store, id).pirate()).collect()
        })
    }
    pub fn user_authored_commit_ids(&mut self, store: &DatastoreView, id: &UserId) -> Option<&Vec<CommitId>> {
        self.smart_load_user_authored_commits(store).get(id)
    }
    pub fn user_committed_experience(&mut self, store: &DatastoreView, id: &UserId) -> Option<Duration> {
        self.smart_load_user_committer_experience(store)
            .get(id)
            .map(|seconds| Duration::from(*seconds))
    }
    pub fn user_author_experience(&mut self, store: &DatastoreView, id: &UserId) -> Option<Duration> {
        self.smart_load_user_author_experience(store)
            .get(id)
            .map(|seconds| Duration::from(*seconds))
    }
    pub fn user_experience(&mut self, store: &DatastoreView, id: &UserId) -> Option<Duration> {
        self.smart_load_user_experience(store)
            .get(id)
            .map(|seconds| Duration::from(*seconds))
    }
    pub fn user_committed_commit_count(&mut self, store: &DatastoreView, id: &UserId) -> Option<usize> {
        self.smart_load_user_committed_commit_count(store).get(id).pirate()
    }
    pub fn user_authored_commit_count(&mut self, store: &DatastoreView, id: &UserId) -> Option<usize> {
        self.smart_load_user_authored_commit_count(store).get(id).pirate()
    }
    pub fn user_committed_commits(&mut self, store: &DatastoreView, id: &UserId) -> Option<Vec<Commit>> {
        self.smart_load_user_committed_commits(store).get(id).pirate().map(|ids| {
            ids.iter().flat_map(|id| self.commit(store, id).pirate()).collect()
        })
    }
}

macro_rules! load_from_store {
    ($self:ident, $vector:ident, $store:expr)  => {{
        if !$self.$vector.is_loaded() {
            $self.$vector.load_from_one($store);
        }
        $self.$vector.grab_collection()
    }}
}

macro_rules! load_with_prerequisites {
    ($self:ident, $vector:ident, $store:expr, $n:ident, $($prereq:ident),*)  => {{
        mashup! {
            $( m["smart_load" $prereq] = smart_load_$prereq; )*
               m["load"] = load_from_$n;
        }
        if !$self.$vector.is_loaded() {
            m! { $(  $self."smart_load" $prereq($store); )*              }
            m! { $self.$vector."load"($($self.$prereq.grab_collection()), *); }
        }
        $self.$vector.grab_collection()
    }}
}

impl Data {
    fn smart_load_project_urls(&mut self, store: &DatastoreView) -> &BTreeMap<ProjectId, String> {
        load_from_store!(self, project_urls, store)
    }
    fn smart_load_project_heads(&mut self, store: &DatastoreView) -> &BTreeMap<ProjectId, Vec<Head>> {
        load_from_store!(self, project_heads, store)
    }
    fn smart_load_project_users(&mut self, store: &DatastoreView) -> &BTreeMap<ProjectId, Vec<UserId>> {
        load_with_prerequisites!(self, project_users, store, two, project_authors, project_committers)
    }
    fn smart_load_project_authors(&mut self, store: &DatastoreView) -> &BTreeMap<ProjectId, Vec<UserId>> {
        load_with_prerequisites!(self, project_authors, store, two, project_commits, commits)
    }
    fn smart_load_project_committers(&mut self, store: &DatastoreView) -> &BTreeMap<ProjectId, Vec<UserId>> {
        load_with_prerequisites!(self, project_committers, store, two, project_commits, commits)
    }
    fn smart_load_project_commits(&mut self, store: &DatastoreView) -> &BTreeMap<ProjectId, Vec<CommitId>> {
        load_with_prerequisites!(self, project_commits, store, two, project_heads, commits)
    }
    fn smart_load_project_paths(&mut self, store: &DatastoreView) -> &BTreeMap<ProjectId, Vec<PathId>> {
        load_with_prerequisites!(self, project_paths, store, two, project_commits, commit_changes)
    }
    fn smart_load_project_snapshots(&mut self, store: &DatastoreView) -> &BTreeMap<ProjectId, Vec<SnapshotId>> {
        load_with_prerequisites!(self, project_snapshots, store, two, project_commits, commit_changes)
    }
    fn smart_load_project_user_count(&mut self, store: &DatastoreView) -> &BTreeMap<ProjectId, usize> {
        load_with_prerequisites!(self, project_user_count, store, one, project_users)
    }
    fn smart_load_project_author_count(&mut self, store: &DatastoreView) -> &BTreeMap<ProjectId, usize> {
        load_with_prerequisites!(self, project_author_count, store, one, project_authors)
    }
    fn smart_load_project_path_count(&mut self, store: &DatastoreView) -> &BTreeMap<ProjectId, usize> {
        load_with_prerequisites!(self, project_path_count, store, one, project_paths)
    }
    fn smart_load_project_snapshot_count(&mut self, store: &DatastoreView) -> &BTreeMap<ProjectId, usize> {
        load_with_prerequisites!(self, project_snapshot_count, store, one, project_snapshots)
    }
    fn smart_load_project_committer_count(&mut self, store: &DatastoreView) -> &BTreeMap<ProjectId, usize> {
        load_with_prerequisites!(self, project_committer_count, store, one, project_committers)
    }
    fn smart_load_project_commit_count(&mut self, store: &DatastoreView) -> &BTreeMap<ProjectId, usize> {
        load_with_prerequisites!(self, project_commit_count, store, one, project_commits)
    }
    fn smart_load_project_lifetimes(&mut self, store: &DatastoreView) -> &BTreeMap<ProjectId, u64> {
        load_with_prerequisites!(self, project_lifetimes, store, three, project_commits,
                                                                        commit_author_timestamps,
                                                                        commit_committer_timestamps)
    }
    fn smart_load_users(&mut self, store: &DatastoreView) -> &BTreeMap<UserId, User> {
        load_from_store!(self, users, store)
    }
    fn smart_load_user_authored_commits(&mut self, store: &DatastoreView) -> &BTreeMap<UserId, Vec<CommitId>> {
        load_with_prerequisites!(self, user_authored_commits, store, one, commits)
    }
    fn smart_load_user_committed_commits(&mut self, store: &DatastoreView) -> &BTreeMap<UserId, Vec<CommitId>> {
        load_with_prerequisites!(self, user_committed_commits, store, one, commits)
    }
    fn smart_load_user_author_experience(&mut self, store: &DatastoreView) -> &BTreeMap<UserId, u64> {
        load_with_prerequisites!(self, user_author_experience, store, two, user_authored_commits,
                                                                           commit_author_timestamps)
    }
    fn smart_load_user_committer_experience(&mut self, store: &DatastoreView) -> &BTreeMap<UserId, u64> {
        load_with_prerequisites!(self, user_committer_experience, store, two, user_committed_commits,
                                                                              commit_committer_timestamps)
    }
    fn smart_load_user_experience(&mut self, store: &DatastoreView) -> &BTreeMap<UserId, u64> {
        load_with_prerequisites!(self, user_experience, store, three, user_committed_commits,
                                                                      commit_author_timestamps,
                                                                      commit_committer_timestamps)
    }
    fn smart_load_user_committed_commit_count(&mut self, store: &DatastoreView) -> &BTreeMap<UserId, usize> {
        load_with_prerequisites!(self, user_committed_commit_count, store, one, user_committed_commits)
    }
    fn smart_load_user_authored_commit_count(&mut self, store: &DatastoreView) -> &BTreeMap<UserId, usize> {
        load_with_prerequisites!(self, user_authored_commit_count, store, one, user_authored_commits)
    }
    fn smart_load_paths(&mut self, store: &DatastoreView) -> &BTreeMap<PathId, Path> {
        load_from_store!(self, paths, store)
    }
    // fn smart_load_snapshots(&mut self, store: &DatastoreView) -> &BTreeMap<SnapshotId, Snapshot> {
    //     load_from_store!(self, snapshots, store)
    // }
    fn smart_load_commits(&mut self, store: &DatastoreView) -> &BTreeMap<CommitId, Commit> {
        load_from_store!(self, commits, store)
    }
    fn smart_load_commit_hashes(&mut self, store: &DatastoreView) -> &BTreeMap<CommitId, String> {
        load_from_store!(self, commit_hashes, store)
    }
    fn smart_load_commit_messages(&mut self, store: &DatastoreView) -> &BTreeMap<CommitId, String> {
        load_from_store!(self, commit_messages, store)
    }
    fn smart_load_commit_committer_timestamps(&mut self, store: &DatastoreView) -> &BTreeMap<CommitId, i64> {
        load_from_store!(self, commit_committer_timestamps, store)
    }
    fn smart_load_commit_author_timestamps(&mut self, store: &DatastoreView) -> &BTreeMap<CommitId, i64> {
        load_from_store!(self, commit_author_timestamps, store)
    }
    fn smart_load_commit_changes(&mut self, store: &DatastoreView) -> &BTreeMap<CommitId, Vec<ChangeTuple>> {
        load_from_store!(self, commit_changes, store)
    }
    fn smart_load_commit_change_count(&mut self, store: &DatastoreView) -> &BTreeMap<CommitId, usize> {
        load_with_prerequisites!(self, commit_change_count, store, one, commit_changes)
    }
}

impl Data {
    pub fn export_to_csv<S>(&mut self, store: &DatastoreView, dir: S) -> Result<(), std::io::Error> where S: Into<String> {
        let dir = dir.into();
        std::fs::create_dir_all(&dir)?;
        macro_rules! path {
            ($filename:expr) => {
                format!("{}/{}.csv", dir, $filename)
            }
        }

        self.project_metadata.iter(store).into_csv(path!("project_metadata"))?;

        self.smart_load_project_urls(store).iter().into_csv(path!("project_urls"))?;
        self.smart_load_project_heads(store).iter().into_csv(path!("project_heads"))?;
        self.smart_load_users(store).iter().into_csv(path!("users"))?;
        self.smart_load_paths(store).iter().into_csv(path!("paths"))?;
        self.smart_load_commits(store).iter().into_csv(path!("commits"))?;
        self.smart_load_commit_hashes(store).iter().into_csv(path!("commit_hashes"))?;
        self.smart_load_commit_messages(store).iter().into_csv(path!("commit_messages"))?;
        self.smart_load_commit_committer_timestamps(store).iter().into_csv(path!("commit_committer_timestamps"))?;
        self.smart_load_commit_author_timestamps(store).iter().into_csv(path!("commit_author_timestamps"))?;
        self.smart_load_commit_changes(store).iter().into_csv(path!("commit_changes"))?;

        store.contents_data()
            .map(|(id, content)| {
                Snapshot::new(SnapshotId::from(id), content)
            }).into_csv(path!("snapshots"))?;

        Ok(())
    }
}

impl Database {
    pub fn export_to_csv<S>(&self, dir: S) -> Result<(), std::io::Error> where S: Into<String> {
        self.data.borrow_mut().export_to_csv(&self.store, dir)
    }
}

#[cfg(test)]
mod data {
    use crate::log::{Log, Verbosity};
    use dcd::DatastoreView;
    use crate::data::Database;
    use std::fs::{remove_dir_all, metadata};
    use crate::objects::{ProjectId, Project, ItemWithData};
    use std::collections::HashMap;

    const DATASET_DIR: &'static str = "/dejacode/tiny-mk2";
    const CACHE_DIR:   &'static str = "/dejacode/cache-mk2";
    const TIME:                 i64 = 1607952032i64;

    fn exists(path: &'static str) -> bool {
        metadata(std::path::Path::new(path)).map_or(false, |_| true)
    }

    // fn dir_exists(path: &Path) -> bool {
    //     metadata(path).map_or(false, |metadata| metadata.is_dir())
    // }

    fn setup_database(precached: bool) -> Database {
        if !precached && exists(CACHE_DIR) {
            remove_dir_all(CACHE_DIR)
                .expect(&format!("Could not delete directory {}", CACHE_DIR));
        }

        let log: Log = Log::new(Verbosity::Debug);
        let store = DatastoreView::new(DATASET_DIR, TIME);
        let database =  Database::from_store(store, CACHE_DIR, log);

        database
    }

    #[test]
    fn projects_against_expected() {
        let database = setup_database(false);

        let expected: HashMap<ProjectId, String> = vec![
            (0, "https://github.com/tosch/ruote-kit.git"),
            (1, "https://github.com/kennethkalmer/ruote-kit.git"),
            (2, "https://github.com/matplotlib/basemap.git"),
            (3, "https://github.com/jswhit/basemap.git"),
            (4, "https://github.com/rolandoam/cocos2d-x.git"),
            (5, "https://github.com/cocos2d/cocos2d-x.git"),
            (6, "https://github.com/pixonic/cocos2d-x.git"),
            (7, "https://github.com/nubic/ncs_navigator_core.git"),
            (8, "https://github.com/sgonyea/rake-compiler.git"),
            (9, "https://github.com/chapuni/llvm.git"),
            (10, "https://github.com/heroku/heroku-buildpack-scala.git"),
            (11, "https://github.com/rafacm/heroku-buildpack-scala.git"),
            (12, "https://github.com/fluttershy/locria.git"),
            (13, "https://github.com/edvorg/cpp-drash.git"), // "drash" in file
            (14, "https://github.com/abarocio80/clide.git"),
            (15, "https://github.com/thorlax402/thor-cms.git"),
            (16, "https://github.com/offsite/taskcodes.git"),
            (17, "https://github.com/markpasc/gameshake.git"),
            (18, "https://github.com/samuelclay/newsblur.git"),
            (19, "https://github.com/chrisjaure/git-lava.git"),
            (20, "https://github.com/es-doc/esdoc-questionnaire.git"), // "djanco-cim-forms" in file
            (21, "https://github.com/adammark/markup.js.git"),
            (22, "https://github.com/leoamigood/1stdibs_v2.1.git"),
            (23, "https://github.com/pyrovski/large-scale-forward-regression-using-a-partitioned-linear-model.git"),
            (24, "https://github.com/podarsmarty/cobertura-plugin.git"),
            (25, "https://github.com/fbettag/scala-vs-erlang.git"),
            (26, "https://github.com/rake-compiler/rake-compiler.git"),
            (27, "https://github.com/opencv/opencv.git"),
            (28, "https://github.com/jkammerl/opencv.git"),
            (29, "https://github.com/gpjt/webgl-lessons.git"),
            (30, "https://github.com/kerolasa/lelux-utiliteetit.git"),
            (31, "https://github.com/snowblindfatal/glomes.git"),
            (32, "https://github.com/pockethub/pockethub.git"),
            (33, "https://github.com/mirocow/yii-easyapns.git"),
            (34, "https://github.com/angular/angular.js.git"),
            (35, "https://github.com/wallysalami/yii-easyapns.git"),
            (36, "https://github.com/macmade/opencv-ios.git"),
            (37, "https://github.com/powmedia/buildify.git"),
            (38, "https://github.com/liberty-concepts/redmine_git_hosting.git"),
            (39, "https://github.com/kubitron/redmine_git_hosting.git"),
            (40, "https://github.com/hpc/iptablesbuild.git"),
            (41, "https://github.com/chenniaoc/opencv-ios.git"),
            (42, "https://github.com/tijsverkoyen/dotfiles.git"),
            (43, "https://github.com/6a68/browserid.git"),
            (44, "https://github.com/samtubbax/dotfiles.git"),
            (45, "https://github.com/jman01/customizations.git"),
            (46, "https://github.com/alexgorbatchev/syntaxhighlighter.git"),
            (47, "https://github.com/fredwu/jquery-endless-scroll.git"),
            (48, "https://github.com/kanishkaganguly/zero-requiem.git"),
            (49, "https://github.com/bronsa/brochure.git"),
            (50, "https://github.com/yui/yui3.git"),
            (51, "https://github.com/jesperes/protobuf-cmake.git"),
            (52, "https://github.com/pculture/unisubs.git"),
            (53, "https://github.com/imtapps/django-request-signer.git"),
            (54, "https://github.com/nadafigment/protobuf-cmake.git"),
            (55, "https://github.com/libram/django-request-signer.git"),
            (56, "https://github.com/fangpenlin/loso.git"),
            (57, "https://github.com/lucaswei/loso.git"),
            (58, "https://github.com/apipkin/yui3.git"),
            (59, "https://github.com/doctag/doctag_java.git"),
            (60, "https://github.com/llvm-mirror/llvm.git"),
            (61, "https://github.com/gini/doctag_java.git"),
            (62, "https://github.com/joyent/libuv.git"),
            (63, "https://github.com/schatten/schatten.github.com.git"),
            (64, "https://github.com/gosquared/nvm-cookbook.git"),
            (65, "https://github.com/davewid/legacy-php-talk.git"),
            (66, "https://github.com/mshk/data-journalism-handbook-ja.git"),
            (67, "https://github.com/russellspitzer/sample_app.git"),
            (68, "https://github.com/willdurand/willdurand.github.io.git"),
            (69, "https://github.com/stof/willdurand.github.com.git"),
            (70, "https://github.com/rxgx/dotfiles.git"),
            (71, "https://github.com/ablu/manaserv.git"),
            (72, "https://github.com/garyrussell/spring-integration.git"),
            (73, "https://github.com/yomoyomo/data-journalism-handbook-ja.git"),
            (74, "https://github.com/mana/manaserv.git"),
            (75, "https://github.com/bjorn/manaserv.git"),
            (76, "https://github.com/fnando/i18n-js.git"),
            (77, "https://github.com/olegz/spring-integration.git"),
            (78, "https://github.com/chapuni/llvm-project.git"),
            (79, "https://github.com/neverabc/libuv.git"),
            (80, "https://github.com/blinkbox/cucumber-js.git"),
            (81, "https://github.com/elaird/supy.git"),
            (82, "https://github.com/janrain/jump.ios.git"),
            (83, "https://github.com/timblinkbox/cucumber-js.git"),
            (84, "https://github.com/angular/angular-seed.git"),
            (85, "https://github.com/mashiro/i18n-js.git"),
            (86, "https://github.com/jakewharton/viewpagerindicator.git"),
            (87, "https://github.com/evh27/angular-seed.git"),
            (88, "https://github.com/leon/play-salat.git"),
            (89, "https://github.com/bnoordhuis/libuv.git"),
            (90, "https://github.com/oftc/libuv.git"),
            (91, "https://github.com/shepheb/jotto.git"),
            (92, "https://github.com/virgo-agent-toolkit/rackspace-monitoring-agent.git"),
            (93, "https://github.com/incuna/django-extensible-profiles.git"),
            (94, "https://github.com/redaemn/angular-seed.git"),
            (95, "https://github.com/zorgleh/try_git.git"),
            (96, "https://github.com/madrobby/zepto.git"),
            (97, "https://github.com/ochameau/addon-sdk.git"),
            (98, "https://github.com/brandonwamboldt/utilphp.git"),
        ].into_iter().map(|(id, url): (u64, &str)| (ProjectId::from(id), url.to_owned())).collect();

        database.projects().for_each(|project| {
            let expected_url = expected.get(&project.id())
                .expect(&format!("Not expected to see a project with id {}", project.id()))
                .to_owned();
            assert_eq!(expected_url, project.url())
        });

        let projects: HashMap<ProjectId, ItemWithData<Project>> = database.projects()
            .map(|project| (project.id(), project)).collect();

        expected.iter().for_each(|(id, url)| {
            let project = projects.get(id)
                .expect(&format!("Expected to see a project with id {}", id))
                .to_owned();
            assert_eq!(url.clone(), project.url())
        })
    }
}