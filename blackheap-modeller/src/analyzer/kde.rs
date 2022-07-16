use criterion_stats::univariate::kde::kernel::Gaussian;
use criterion_stats::univariate::kde::{Bandwidth, Kde};
use criterion_stats::univariate::Sample;

use itertools_num::linspace;

use serde::{Deserialize, Serialize};

use crate::analyzer::json_reader::BenchmarkJSON;

#[derive(Debug, Serialize, Deserialize)]
pub struct BenchmarkKde {
    pub access_size: u64,
    pub xs: Vec<f64>,
    pub ys: Vec<f64>,
    pub significant_clusters: Vec<Cluster>,
    pub global_maximum: (f64, f64),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Cluster {
    xs: Vec<f64>,
    ys: Vec<f64>,
    maximum: (f64, f64),
}

impl Cluster {
    fn merge(&self, next: &Cluster) -> Cluster {
        let mut global_xs: Vec<f64> = self.xs.clone();
        let mut global_ys: Vec<f64> = self.ys.clone();
        global_xs.append(&mut next.xs.clone());
        global_ys.append(&mut next.ys.clone());
        let global_max: (f64, f64) = if self.maximum.1 > next.maximum.1 {
            self.maximum
        } else {
            next.maximum
        };
        Cluster {
            xs: global_xs,
            ys: global_ys,
            maximum: global_max,
        }
    }

    fn is_significant(&self, global_maximum: f64) -> bool {
        (self.maximum.1 - self.ys[self.ys.len() - 1]) >= 0.1 * global_maximum
    }
}

impl BenchmarkKde {
    pub fn from_benchmark(b: &BenchmarkJSON, n: usize) -> BenchmarkKde {
        // Generate kde values
        let slice: &[f64] = &b.durations[..];
        let data: &Sample<f64> = Sample::new(slice);
        let kde: Kde<f64, Gaussian> = Kde::new(data, Gaussian, Bandwidth::Silverman);
        let h: f64 = kde.bandwidth();
        let (left, right): (f64, f64) = (data.min() - 5. * h, data.max() + 5. * h);
        let xs: Vec<f64> = linspace::<f64>(left, right, n).collect();
        let ys: Vec<f64> = kde.map(&xs).to_vec();

        // compute significant clusters
        let (minima, maxima) = Self::get_all_extrema(&xs, &ys);
        let global_maximum: (f64, f64) = Self::get_global_maximum(&maxima);
        let significant_clusters: Vec<Cluster> =
            Self::to_significant_clusters(&xs, &ys, minima, maxima);
        let access_size: u64 = b.access_size_in_bytes;
        BenchmarkKde {
            xs,
            ys,
            significant_clusters,
            global_maximum,
            access_size,
        }
    }

    #[allow(clippy::type_complexity)] // Single function usage so it's fine
    fn get_all_extrema(xs: &[f64], ys: &[f64]) -> (Vec<(f64, f64)>, Vec<(f64, f64)>) {
        let mut minima: Vec<(f64, f64)> = Vec::new();
        let mut maxima: Vec<(f64, f64)> = Vec::new();
        for i in 1..(xs.len() - 1) {
            let is_increasing_before: bool = ys[i - 1] <= ys[i];
            let is_increasing_after: bool = ys[i] <= ys[i + 1];
            if is_increasing_before == is_increasing_after {
                continue;
            }
            if is_increasing_before && !is_increasing_after {
                maxima.push((xs[i], ys[i]));
            } else {
                minima.push((xs[i], ys[i]));
            }
        }
        (minima, maxima)
    }

    fn to_all_cluster(
        xs: &[f64],
        ys: &[f64],
        mut minima: Vec<(f64, f64)>,
        maxima: Vec<(f64, f64)>,
    ) -> Vec<Cluster> {
        // We want a delimiter at the front and back
        minima.insert(0, (xs[0], ys[0]));
        minima.push((xs[xs.len() - 1], ys[ys.len() - 1]));

        let mut ret: Vec<Cluster> = Vec::new();
        for i in 0..maxima.len() {
            let left_minimum: f64 = minima[i].0;
            let maximum: (f64, f64) = maxima[i];
            let right_minimum: f64 = minima[i + 1].0;

            // this is inperformant but who cares for now
            let left_index: usize = xs.iter().position(|&x| x == left_minimum).unwrap();
            let right_index: usize = xs.iter().position(|&x| x == right_minimum).unwrap();

            let xs: Vec<f64> = xs[left_index..right_index + 1].to_vec();
            let ys: Vec<f64> = ys[left_index..right_index + 1].to_vec();

            let cluster: Cluster = Cluster { xs, ys, maximum };

            ret.push(cluster);
        }
        ret
    }

    fn to_significant_clusters(
        xs: &[f64],
        ys: &[f64],
        minima: Vec<(f64, f64)>,
        maxima: Vec<(f64, f64)>,
    ) -> Vec<Cluster> {
        let clusters: Vec<Cluster> = Self::to_all_cluster(xs, ys, minima, maxima);
        let global_maximum: f64 = clusters.iter().fold(0.0f64, |max, new| {
            if max > new.maximum.1 {
                max
            } else {
                new.maximum.1
            }
        });

        // So a cluster is a cluster iff
        // (maxima[i] - minima[i+1]) < 0.1 * global_maximum
        //
        // We join clusters together until that is true.

        let mut res: Vec<Cluster> = Vec::new();
        let mut curr_cluster: Option<Cluster> = None;
        for c in clusters {
            // If we have none, this means last was significant, i.e we cut off
            // If we have some, this means it was not significant, so maybe our joined one will be.
            curr_cluster = match curr_cluster {
                None => Some(c),
                Some(cluster) => Some(cluster.merge(&c)),
            };
            // At this point we __know__ it has to be Some()
            if curr_cluster
                .as_ref()
                .unwrap()
                .is_significant(global_maximum)
            {
                res.push(curr_cluster.unwrap());
                curr_cluster = None;
            }
        }
        res
    }

    fn get_global_maximum(maxima: &[(f64, f64)]) -> (f64, f64) {
        maxima.iter().fold((0.0, 0.0), |curr_max, new| {
            if new.1 > curr_max.1 {
                (new.0, new.1)
            } else {
                curr_max
            }
        })
    }
}
