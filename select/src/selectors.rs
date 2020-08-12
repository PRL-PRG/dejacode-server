use dcd::{ProjectId, Database, Project};
use std::cmp::Ordering;
use itertools::Itertools;
use crate::meta::ProjectMeta;

pub fn group_by_language_order_by_stars_top_n(database: &impl Database,
                                              top_n: usize)
                                              -> Vec<ProjectId> {

    let star_sorter_descending = |p1: &Project, p2: &Project| {
        match (p1.get_stars(), p2.get_stars()) {
            (Some(n1), Some(n2)) =>
                     if n1 < n2 { Ordering::Greater }
                else if n1 > n2 { Ordering::Less    }
                else            { Ordering::Equal   },

            (None, None) =>       Ordering::Equal,
            (None,    _) =>       Ordering::Greater,
            (_,    None) =>       Ordering::Less,
        }
    };

    database.projects()
        .map(|p| (p.get_language(), p))
        .into_group_map()
        .into_iter()
        .flat_map(|(_language, mut projects)| {
            projects.sort_by(star_sorter_descending);
            projects.iter().take(top_n).map(|p| p.id).collect::<Vec<ProjectId>>()
        })
        .collect()
}