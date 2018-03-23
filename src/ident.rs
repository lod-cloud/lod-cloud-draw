//! Code used to identify the domains (bubble colours) of the bubbles
use data::Dataset;
use std::collections::HashMap;
use noisy_float::prelude::*;

/// Find the domain by the neighbours of a dataset, the domain will be
/// set to the most frequent among neighbours
pub fn domain_by_most_neighbours(datasets : &mut HashMap<String, Dataset>) {
    let mut incoming = HashMap::new();
    for (_, dataset) in datasets.iter() {
        for link in dataset.links.iter() {
            incoming.entry(link.target.clone())
                .or_insert_with(|| Vec::new()).push(dataset.identifier.clone());
        }
    }
    let mut ds2domain : HashMap<String, String> = datasets.iter().map(|k| {
        (k.0.clone(), k.1.domain.clone())
    }).collect();
    let mut last_fails = -1;
    let mut fails = 0;
    while fails != last_fails {
        last_fails = fails;
        fails = 0;
        for (_, dataset) in datasets.iter_mut() {
            if dataset.domain == "" {
                let mut counts = HashMap::new();
                for link in dataset.links.iter() {
                    match ds2domain.get(&link.target) {
                        Some(d) if d != "" => {
                            let c : i32 = *counts.get(d).unwrap_or(&0);
                            counts.insert(d.clone(), c + 1);
                        },
                        _ => {}
                    }
                }

                let empty = Vec::new();
                for link in incoming.get(&dataset.identifier).unwrap_or_else(|| &empty).iter() {
                    match ds2domain.get(link) {
                        Some(d) if d != "" => {
                            let c : i32 = *counts.get(d).unwrap_or(&0);
                            counts.insert(d.clone(), c + 1);
                        },
                        _ => {}
                    }
                }

                let mut best_domain = String::new();
                let mut best_count = -1;
                for (k, v) in counts.iter() {
                    if *v > best_count {
                        best_domain = k.clone();
                        best_count = *v;
                    }
                }
                if best_domain == "" {
                    fails += 1;
                } else {
                    dataset.domain = best_domain.to_string();
                    ds2domain.insert(dataset.identifier.to_string(),
                        best_domain.to_string());
                }
            }
        }
//        eprintln!("Fails: {} ({})", fails, last_fails);
    }
}

const ALPHA : f64 = 0.0001f64;

/// Find the domain of a dataset by the set of keywords. A naive Bayes classifier
/// is created from the labelled datasets and this is applied to all the 
/// unlabelled datasets
pub fn domain_by_keywords(datasets : &mut HashMap<String, Dataset>) {
    let mut tag_cat_freq = HashMap::new();
    let mut cat_freq = HashMap::new();
    let mut tag_freq = HashMap::new();
    let mut total = 0;

    for (_, dataset) in datasets.iter() {
        let cat = dataset.domain.clone();
        if cat != "" {
            let c = *cat_freq.get(&cat).unwrap_or(&0);
            for tag in dataset.keywords.iter() {
                let x = tag_cat_freq.entry(tag.clone()).or_insert_with(|| HashMap::new());
                let c2 = *x.get(&cat).unwrap_or(&0);
                x.insert(cat.clone(), c2 + 1);
                let c3 = *tag_freq.get(tag).unwrap_or(&0);
                tag_freq.insert(tag.clone(), c3 + 1);
            }
            cat_freq.insert(cat, c + 1);
            total += 1;
        }
    }

    let n_alpha = (cat_freq.len() as f64) * ALPHA;

    let tag_cat_prob : HashMap<(String, String), f64> = 
        tag_freq.keys().flat_map(|_tag| {
            let tag = _tag.clone();
            let v : Vec<((String, String), f64)> =
                cat_freq.keys().map(|_cat| {
                let cat = _cat.clone();
                let tcf = *tag_cat_freq[&tag].get(&cat).unwrap_or(&0) as f64 + ALPHA;
                let tf = *tag_freq.get(&tag).unwrap_or(&0) as f64 + ALPHA;
                let cf = *cat_freq.get(&cat).unwrap_or(&0) as f64 + n_alpha;
                let p = (tcf / cf).ln() - (tf / (total as f64 + n_alpha)).ln();
                ((tag.clone(), cat.clone()), p)
            }).collect();
            v
        }).collect();

    let cat_prob : HashMap<String, f64> = cat_freq.iter().map(|cf| {
        (cf.0.clone(), ((*cf.1 as f64 + ALPHA) / (total as f64 + n_alpha)).ln())
    }).collect();

    let cats : Vec<String> = cat_prob.keys().map(|x| x.clone()).collect();

    for (_, dataset) in datasets.iter_mut() {
        if dataset.domain == "" {
            if let Some((cat, _p)) = cats.iter().map(|c| {
                let mut prob = *cat_prob.get(c).unwrap_or(&(ALPHA / (total as f64 + n_alpha)));
                for tag in dataset.keywords.iter() {
                    prob += *tag_cat_prob.get(&(tag.to_string(), c.to_string())).unwrap_or(&0.0);
                }
                (c.clone(), r64(prob))
            }).max_by_key(|c| c.1) {
                dataset.domain = cat;
            }
        }
    }
}

#[cfg(test)]
mod tests{
    use std::collections::HashMap;
    use data::{Dataset,IntLike};
    use ident::*;

    fn make_dataset(s : &str, d : &str) -> Dataset {
        Dataset {
            description: HashMap::new(),
            title: None,
            links: Vec::new(),
            identifier: s.to_string(),
            domain: d.to_string(),
            triples: IntLike::from(0),
            keywords : s.chars().map(|x| x.to_string()).collect()
        }
    }

    #[test]
    fn test_domain_by_keywords() {
        let mut datasets = HashMap::new();
        datasets.insert("foo".to_string(), make_dataset("foo", "a"));
        datasets.insert("bar".to_string(), make_dataset("bar", "b"));
        datasets.insert("baz".to_string(), make_dataset("baz", "b"));
        datasets.insert("bao".to_string(), make_dataset("bao", ""));
        datasets.insert("fod".to_string(), make_dataset("fod", ""));

        domain_by_keywords(&mut datasets);

        assert_eq!(datasets["bao"].domain, "b");
        assert_eq!(datasets["fod"].domain, "a");
    }
}

