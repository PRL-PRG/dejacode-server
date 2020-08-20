# Getting started

Download this repository

```sh
git clone https://github.com/PRL-PRG/dejacode-server.git
```

Download the downloader

```sh
git clone https://github.com/PRL-PRG/dejacode-downloader.git
```

Install cargo if needed (linux/mac):

```sh
curl https://sh.rustup.rs -sSf | sh
```

Build downloader


```sh
cd dejacode-downloader/
cargo build
cd ..
```

Prepare a dataset using the downloader's toolchain from a pre-downloaded sample, for example:

```sh
cd dejacode-downloader/target/release
./dcd-ghtorrent "$USER/my-datasets/dataset-15lang-1K" "/dejavuii/dejacode/ghtorrent/sample-15-1000/" --new
cd ../..
```

Install cargo if needed (linux/mac):

```sh
curl https://sh.rustup.rs -sSf | sh
```

Compile example queries using cargo

```sh
cd dejacode-server/examples
cargo build
cd ../..
```

Run example queries:

```sh
cd dejacode-server/examples
```

Group by language, sort by number of commits, take top 50 in each language

```sh
cargo run --package examples --bin commits -- --dataset="$USER/my-datasets/dataset-15lang-1K"" --output="commits.csv"
```

Group by language, sort by average number of changes in project's commits, take top 50 in each language.

```sh
cargo run --package examples --bin changes_in_commits -- --dataset="$USER/my-datasets/dataset-15lang-1K"" --output="changes_in_commits.csv"
```

Group by language, sort by number of issues in a project, take top 50 in each language:

```sh
cargo run --package examples --bin issues -- --dataset="$USER/my-datasets/dataset-15lang-1K"" --output="issues.csv"
```

Group by language, sort by number of buggy issues in a project, take top 50 in each language:

```sh
cargo run --package examples --bin buggy_issues -- --dataset="$USER/my-datasets/dataset-15lang-1K"" --output="buggy_issues.csv"
```

# Hacking togethet queries

First we need access to the dataset:

```rust
let database: DCD = DCD::new(/* path to dataset */);
```

There's a configuration file which will provide the path form commandline arguments:

```rust
let database: DCD = DCD::new(configuration.dataset_path_as_string());
```


There's a function called `sort_and_sample` that does:"Group by language, sort
by X, sample using Y". X and Y are closures.

## Sorting

Sorting requires a closure that takes two project references as arguments and
returns an `Ordering` object. Here's an example of such a closure that compares
projects by the number of stars and creates a descending order:

```rust
    let sorter = 
        |p1: &Project, p2: &Project| {
            let (n1, n2) = p1.get_stars(), p2.get_stars()
            let ascending_ordering =
                     if n1 < n2 { Ordering::Less    }
                else if n1 > n2 { Ordering::Greater }
                else            { Ordering::Equal   };

             ascending_ordering.reverse()            
        }
```

Since sorting closures always look the same, there's a macro to generate them,
so that we can create an equivalent sorter like this:

```rust
    let sorter = sort_by_numbers!(Direction::Descending,  |p: &Project| {p.get_stars_or_zero()})
```

This macro works for numerical values. There's also a version that works on
numbers wrapped in Option:


```rust
    let sorter = sort_by_numbers_opt!(Direction::Descending,  |p: &Project| {p.get_stars()})
```

## Sampling

Sampling requires a closure that takes a vector of projects and returns a
vector of projects. Her's a sampler that takes top 50 projects (in each
language group):

```rust
    let sampler = |projects: Vec<Project>| {
        projects.into_iter().take(50).collect::<Vec<Project>>()
    };
```

There's also a macro to generate that:

```rust
    let sampler = top!(50);
```

Here's how random sampling would work:

```rust
    let seed_bytes = 42u64.to_be_bytes();           // Seed is 42
    let mut rng = Pcg64Mcg::from_seed(seed_bytes);
    let sampler = move |projects: Vec<Project>| {
        projects.into_iter().choose_multiple(&mut rng, 50)
    }
```

## Putting it together

When we have a sampler and a sorter, we plug them into `sort_and_sample`. It
also needs a database.

```
let projects: Vec<Project> = sort_and_sample(&database, sorter, sampler)
```

This returns a vector of Project objects. There's an output function which will
print out the IDs of these objects to a file for us at a location specified by
the configuration:

```rust
write_to_output(&configuration, &projects)
```

## Filtering

For filtering, there's also a function called `filter_sort_and_sample`. In
addition to the closure we used before it needs one more, to filter projects.
This closure takes a reference to a project as argument and returns true (to
keep) or false (to discard). 

This closure removes all projects that have fewer than 5 users:

```rust
    let filter = |project: &Project| {
        project.get_user_count_in(&database) > 5
    }
```

The we can plug it into the coordinating function like this:

```rust
   let projects: Vec<Project> = filter_sort_and_sample(&database, filter, sorter, sampler) 
```