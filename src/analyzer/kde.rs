use plotlib::page::Page;
use plotlib::repr::Plot;
use plotlib::style::{LineStyle, LineJoin, PointMarker, PointStyle};
use plotlib::view::ContinuousView;

use criterion_stats::univariate::kde::kernel::Gaussian;
use criterion_stats::univariate::kde::{Bandwidth, Kde};
use criterion_stats::univariate::Sample;

use itertools_num::linspace;

use crate::analyzer::json_reader::BenchmarkJSON;

pub struct BenchmarkKde {
    pub xs: Vec<f64>,
    pub ys: Vec<f64>
}

struct Cluster {
    xs: Vec<f64>,
    ys: Vec<f64>,
    maximum: (f64, f64),
}

impl Cluster {
    fn merge(&self, next: &Cluster) -> Cluster {
        let global_max;
        let mut global_xs = self.xs.clone();
        let mut global_ys = self.ys.clone();
        global_xs.append(&mut next.xs.clone());
        global_ys.append(&mut next.ys.clone());
        if self.maximum.1 > next.maximum.1 {
            global_max = self.maximum;
        } else {
            global_max = next.maximum;
        }
        Cluster {
            xs: global_xs,
            ys: global_ys,
            maximum: global_max
        }
    }

    fn is_significant(&self, global_maximum: f64) -> bool {
        (self.maximum.1 - self.ys[self.ys.len()-1]) >= 0.1 * global_maximum
    }
}

impl BenchmarkKde {
    pub fn from_benchmark(b: &BenchmarkJSON, n: usize) -> BenchmarkKde {
        let slice = &b.durations[..];
        let data = Sample::new(slice);
        let kde = Kde::new(data, Gaussian, Bandwidth::Silverman);
        let h = kde.bandwidth();
        let (left, right): (f64, f64) = (data.min() - 5. * h, data.max() + 5. * h);
        let xs: Vec<f64> = linspace::<f64>(left,right, n).collect();
        let ys: Vec<f64> = kde.map(&xs).to_vec();
        BenchmarkKde { xs, ys, }
    }

    pub fn to_svg(&self) -> String{
        let kde_points: Vec<(f64, f64)> = self.xs.iter().cloned().zip(self.ys.iter().cloned()).collect();

        // KDE itself
        let line_plot = Plot::new(kde_points).line_style(
            LineStyle::new()
            .colour("#e1c16e")
            .linejoin(LineJoin::Round)
        );

        // clusters
        let sig_clusters = self.to_significant_clusters();
        let mut maxima = Vec::new();
        for cluster in &sig_clusters {
            maxima.push(cluster.maximum);
        }
        let maxima_plot = Plot::new(maxima).point_style(
            PointStyle::new()
            .marker(PointMarker::Circle)
            .colour("#ff0000")
        );
        // TODO: Replace once we have good box support
        let mut cluster_plots = Vec::new();
        for cluster in &sig_clusters {
            let left = cluster.xs[0];
            let right = cluster.xs[cluster.xs.len()-1];
            let lines = vec![
                (left,0.0f64),
                (left, cluster.maximum.1),
                (right, cluster.maximum.1),
                (right, 0.0f64),
                (left, 0.0f64)
            ];
            cluster_plots.push(
                Plot::new(lines).line_style(
                    LineStyle::new()
                    .colour("#3f888f")
                    .linejoin(LineJoin::Round)
                )
            );
        }


        let mut v = ContinuousView::new()
            .add(line_plot)
            .add(maxima_plot);
        for p in cluster_plots {
            v = v.add(p);
        }
        v = v.x_label("time in seconds").y_label("approximated propability");
        Page::single(&v).to_svg().unwrap().to_string()
    }

    fn get_all_extrema(&self) -> (Vec<(f64, f64)>, Vec<(f64, f64)>){
        let mut minima = Vec::new();
        let mut maxima = Vec::new();
        for i in 1..(self.xs.len()-1) {
            let is_increasing_before = self.ys[i-1] <= self.ys[i];
            let is_increasing_after = self.ys[i] <= self.ys[i+1];
            if is_increasing_before == is_increasing_after {
                continue
            }
            if is_increasing_before && !is_increasing_after {
                maxima.push((self.xs[i], self.ys[i]));
            } else {
                minima.push((self.xs[i], self.ys[i]));
            }
        }
        (minima, maxima)
    }

    fn to_all_cluster(&self) -> Vec<Cluster> {
        let (mut minima, maxima) = self.get_all_extrema();

        // We want a delimiter at the front and back
        minima.insert(0, (self.xs[0], self.ys[0]));
        minima.push((self.xs[self.xs.len()-1], self.ys[self.ys.len()-1]));

        let mut ret = Vec::new();
        for i in 0..maxima.len() {
            let left_minimum = minima[i].0;
            let maximum = maxima[i];
            let right_minimum = minima[i+1].0;

            // TODO god this is inperformant but who cares for now
            let left_index = self.xs.iter().position(|&x| x == left_minimum).unwrap();
            let right_index = self.xs.iter().position(|&x| x == right_minimum).unwrap();

            let xs_cluster =  self.xs[left_index..right_index+1].to_vec();
            let ys_cluster = self.ys[left_index..right_index+1].to_vec();

            let cluster = Cluster {
                xs: xs_cluster,
                ys: ys_cluster,
                maximum: maximum,
            };

            ret.push(cluster);
        }
        ret
    }

    fn to_significant_clusters(&self) -> Vec<Cluster> {
        let clusters = self.to_all_cluster();
        let global_maximum = clusters.iter().
            fold(0.0f64, |max, new| if max > new.maximum.1 { max } else { new.maximum.1 });

        // So a cluster is a cluster iff
        // (maxima[i] - minima[i+1]) < 0.1 * global_maximum
        //
        // We join clusters together until that is true.

        let mut res = Vec::new();
        let mut curr_cluster = None;
        for c in clusters {
            // If we have none, this means last was significant, i.e we cut off
            // If we have some, this means it was not significant, so maybe our joined one will be.
            curr_cluster = match curr_cluster {
                None => { Some(c) },
                Some(cluster) => { Some(cluster.merge(&c)) },
            };
            // At this point we __know__ it has to be Some()
            if curr_cluster.as_ref().unwrap().is_significant(global_maximum) {
                res.push(curr_cluster.unwrap());
                curr_cluster = None;
            }
        }
        res
    }

    pub fn get_global_maximum(&self) -> (f64,f64) {
        let (_, maxima) = self.get_all_extrema();
        maxima.iter().fold((0.0, 0.0), |curr_max, new| if new.1 > curr_max.1 { (new.0, new.1) } else { curr_max })
    }
}