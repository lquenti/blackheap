// TODO: Replace unwraps and excepts
// TODO: Replace paths with AsRef<Path>
use std::fmt;
use std::fs::{canonicalize, create_dir, create_dir_all, DirEntry, File, read_dir, ReadDir};
use std::io::BufReader;
use std::io::prelude::*;
use std::path::PathBuf;
use std::process::{Command, Stdio};

use clap::{AppSettings, IntoApp, Parser, Subcommand};

use criterion_stats::univariate::kde::kernel::Gaussian;
use criterion_stats::univariate::kde::{Bandwidth, Kde};
use criterion_stats::univariate::Sample;

use itertools_num::linspace;

use linregress::{FormulaRegressionBuilder, RegressionDataBuilder};

use plotlib::page::Page;
use plotlib::repr::Plot;
use plotlib::style::{LineStyle, LineJoin, PointMarker, PointStyle};
use plotlib::view::ContinuousView;

use sailfish::TemplateOnce;

use serde::{Serialize, Deserialize};

const NAME: &str = "io-modeller";
const AUTHOR: &str = "Lars Quentin <lars.quentin@gwdg.de>";
const VERSION: &str = "0.1";
const ABOUT: &str = "A blackbox modeller for I/O-classification";

// TODO: some have to be cwd, some path of io-benchmarker
// Probably we should use lazy_static
const DEFAULT_MODEL_PATH: &str = "./default-model";
const DEFAULT_BENCHMARK_FILE_PATH: &str = "/tmp/io_benchmark_test_file.dat";

#[derive(Parser)]
#[clap(name = NAME, author = AUTHOR, version = VERSION, about = ABOUT, long_about = None)]
#[clap(global_setting(AppSettings::InferLongArgs))]
#[clap(global_setting(AppSettings::PropagateVersion))]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Create a new performance model
    CreateModel {
        /// Path to where the models will be saved.
        #[clap(short, long, default_value_t = String::from(DEFAULT_MODEL_PATH))]
        to: String,
        /// Path to where the benchmark should be done.
        #[clap(short, long, default_value_t = String::from(DEFAULT_BENCHMARK_FILE_PATH))]
        file: String,
        #[clap(short, long, required = true)]
        benchmarker: String,
    },
    /// Evaluate recorded I/O accesses according to previously created benchmark.
    UseModel {
        /// Path to model on which the performane will be evaluated on.
        #[clap(short, long, required = true)]
        model: String,
        /// Path to the recorded io accesses.
        #[clap(short, long, required = true)]
        file: String,
    },
}
#[derive(Debug)]
enum AccessPattern {
    Off0,
    Seq,
    Rnd,
}

impl fmt::Display for AccessPattern {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", format!("{:?}", self).to_lowercase())
    }
}

#[derive(Debug)]
enum BenchmarkType {
    RandomUncached,
}

impl fmt::Display for BenchmarkType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}


#[derive(Debug)]
struct PerformanceBenchmark {
    benchmark_type: BenchmarkType,

    is_read_op: bool,
    mem_pattern: AccessPattern,
    file_pattern: AccessPattern,
    repeats: u32,
    memory_buffer_size_in_bytes: u64,
    file_buffer_size_in_bytes: u64,
    use_o_direct: bool,
    drop_cache_before: bool,
    reread_every_block: bool,
    delete_afterwards: bool,

    benchmarker_path: String,
    file_path: String,

    available_ram_in_bytes: Option<i32>,
}

impl PerformanceBenchmark {
  fn new_random_uncached(benchmarker_path: &String, file_path: &String) -> Self {
    PerformanceBenchmark {
        benchmark_type: BenchmarkType::RandomUncached,
        is_read_op: true,
        mem_pattern: AccessPattern::Rnd,
        file_pattern: AccessPattern::Rnd,
        repeats: 1000,
        memory_buffer_size_in_bytes: 4 * u64::pow(1024, 3),
        file_buffer_size_in_bytes: 25 * u64::pow(1024, 3),
        use_o_direct: false,
        drop_cache_before: true,
        reread_every_block: false,
        delete_afterwards: true,

        benchmarker_path: benchmarker_path.clone(),
        file_path: file_path.clone(),

        available_ram_in_bytes: None,
    }
  }

  fn get_parameters(&self, access_size: &u64) -> Vec<String> {
    let mut params = vec![
        String::from(if self.is_read_op { "--read" } else { "--write" }),
        format!("--mem-pattern={}", self.mem_pattern),
        format!("--file-pattern={}", self.file_pattern),
        format!("--repeats={}", self.repeats),
        format!("--mem-buf={}", self.memory_buffer_size_in_bytes),
        format!("--file-buf={}", self.file_buffer_size_in_bytes),
        format!("--access-size={}", access_size),
        format!("--file={}", self.file_path),
    ];
    if self.use_o_direct {
        params.push(String::from("--o-direct"));
    }
    if let Some(bytes) = self.available_ram_in_bytes {
        params.push(format!("--free-ram={}", bytes));
    }
    if self.drop_cache_before {
        params.push(String::from("--drop-cache"));
    }
    if self.reread_every_block {
        params.push(String::from("--reread"));
    }
    if self.delete_afterwards {
        params.push(String::from("--delete-afterwards"));
    }
    params
  }

  fn run_test(&self, access_size: &u64) -> std::result::Result<String, String> {
    let child = Command::new(&self.benchmarker_path)
        .args(self.get_parameters(access_size))
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Process could not be spawned");

    let output = child
        .wait_with_output()
        .expect("failed to wait on child");

    if !output.status.success() {
        let error_message = std::str::from_utf8(&output.stderr)
            .expect("Invalid UTF-8 sequence!");
        return Err(String::from(error_message));
    }

    let ret = std::str::from_utf8(&output.stdout)
        .expect("Invalid UTF-8 sequence!");
    Ok(String::from(ret))
  }

  fn run_test_and_save_to_file(&self, access_size: &u64, file_path: &str){
      let run_res = self.run_test(access_size);
      match run_res {
        Ok(output) => {
            let mut file = File::create(file_path).unwrap();
            file.write_all(output.as_bytes()).unwrap();
        }
        Err(error) => {
            eprintln!("Error: {}", error);
        }
      }
  }

  fn get_benchmark_folder(&self, model_path: &String) -> String {
    format!("{}/{}", model_path, self.benchmark_type.to_string())
  }



  fn run_and_save_all_benchmarks(&self, model_path: &String) -> Result<(), std::io::Error> {
    let benchmark_folder_path = self.get_benchmark_folder(model_path);
    create_dir(&benchmark_folder_path)?;

    for i in 1..28 {
        let access_size = u64::pow(2, i);
        println!("Running {} with access_size {}", self.benchmark_type.to_string(), access_size);

        let path: PathBuf = [
            &benchmark_folder_path,
            &format!("{}.json", access_size)
        ].iter().collect();

        self.run_test_and_save_to_file(&access_size, &path.to_str().unwrap());
    }
    Ok(())
  }
}

// --------------------------------------
// Begin JSON

fn get_all_jsons_from_directory(folder: &PathBuf) -> Vec<PathBuf> {
    let folder: PathBuf = canonicalize(&folder).unwrap();
    let dir: ReadDir = read_dir(&folder).unwrap();

    let mut valid_dir_entries: Vec<DirEntry> = Vec::new();
    for dir_entry in dir {
        match dir_entry {
            Ok(d) => { valid_dir_entries.push(d); },
            Err(e) => { println!("Warning: Could not read '{:?}' because '{}'", folder, e); }
        }
    }

    let mut valid_jsons = Vec::new();
    for dir_entry in valid_dir_entries {
        match dir_entry.file_type() {
            Ok(file_type) => {
                if !file_type.is_file() {
                    continue;
                }
            },
            Err(_) => { continue; },
        }

        let path: PathBuf = dir_entry.path();
        match path.extension() {
            Some(ext) => {
                if ext.to_ascii_lowercase() != "json" {
                    continue;
                }
            },
            None => { continue; },
        }
        valid_jsons.push(path);
    }
    valid_jsons
}

// TODO: scream louder when something goes wrong
fn benchmark_json_to_struct(file_path: &PathBuf) -> Option<BenchmarkJSON> {
    let file = File::open(file_path);

    if let Err(_) = file {
        return None;
    }
    let file = file.unwrap();
    let reader = BufReader::new(file);
    match serde_json::from_reader(reader) {
        Ok(json) => { json },
        Err(e) => { println!("{}", e); return None; },
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct BenchmarkJSON {
    filepath: String,
    repeats: u64,
    memory_buffer_in_bytes: u64,
    file_size_in_bytes: u64,
    access_size_in_bytes: u64,
    access_pattern_in_memory: String,
    access_pattern_in_file: String,
    io_operation: String,
    prepare_file_size: bool,
    restricted_ram_in_bytes: u64,
    use_o_direct: bool,
    drop_cache_first: bool,
    reread_every_block: bool,
    delete_afterwards: bool,
    durations: Vec<f64>,
}

impl BenchmarkJSON {
    fn new_from_dir(folder: &PathBuf) -> Vec<Self> {
        let json_paths: Vec<PathBuf> = get_all_jsons_from_directory(&folder);
        let jsons: Vec<BenchmarkJSON> = json_paths.iter()
            .map(|path| benchmark_json_to_struct(path))
            .filter(|x| x.is_some())
            .map(|x| x.unwrap())
            .collect();
        jsons
    }

    fn generate_kde_from(&self, n: usize) -> BenchmarkKde {
        let slice = &self.durations[..];
        let data = Sample::new(slice);
        let kde = Kde::new(data, Gaussian, Bandwidth::Silverman);
        let h = kde.bandwidth();
        let (left, right): (f64, f64) = (data.min() - 5. * h, data.max() + 5. * h);
        let xs: Vec<f64> = linspace::<f64>(left,right, n).collect();
        let ys: Vec<f64> = kde.map(&xs).to_vec();
        BenchmarkKde { xs, ys, }
    }
}

struct BenchmarkKde {
    xs: Vec<f64>,
    ys: Vec<f64>
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
    fn to_svg(&self) -> String{
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

    fn get_global_maximum(&self) -> (f64,f64) {
        let (_, maxima) = self.get_all_extrema();
        maxima.iter().fold((0.0, 0.0), |curr_max, new| if new.1 > curr_max.1 { (new.0, new.1) } else { curr_max })
    }
}



// -------
// main

fn path_exists(path: &PathBuf) -> Result<(), std::io::Error> {
    if !path.exists() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            format!("{:?} does not exist!", path)
        ));
    }
    Ok(())
}

fn path_does_not_exist(path: &PathBuf) -> Result<(), std::io::Error> {
    if path.exists() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::AlreadyExists,
            format!("{:?} already exists!", path),
        ));
    }
    Ok(())
}

fn validate_create_model(model_path: &String, benchmarker_path: &String) -> Result<(), std::io::Error> {
    // The model path should be non-existing
    //TODO
    //path_does_not_exist(&PathBuf::from(model_path))?;

    // The benchmarker should obviously exist
    //TODO
    //path_exists(&PathBuf::from(benchmarker_path))?;

    Ok(())
}

#[derive(TemplateOnce)]
#[template(path = "result.stpl")]
struct ResultTemplate<'a> {
    benchmark_name: String,
    jsons_kdes: Vec<(&'a BenchmarkJSON, &'a BenchmarkKde)>,
    linear_model: LinearModel,
    linear_model_svg: String
}

/// y=aX+b
struct LinearModel {
    a: f64,
    b: f64,
    max_access_size: f64
}

impl LinearModel {
    // TODO: A lot of double work with to_svg, rewrite me
    // TODO: From next minimum instead of maximum
    fn from_jsons_kdes(jsons: &Vec<BenchmarkJSON>, kdes: &Vec<BenchmarkKde>) -> Self {
        let max_access_size = jsons[jsons.len()-1].access_size_in_bytes as f64;
        let (xs, ys) = Self::get_xs_ys(jsons, kdes);
        let data = vec![("X", xs), ("Y", ys)];
        let formula = "Y ~ X";
        let data = RegressionDataBuilder::new().build_from(data).unwrap();
        let model = FormulaRegressionBuilder::new()
        .data(&data)
        .formula(formula)
        .fit().unwrap();

        let parameters = model.parameters;
        let a = parameters.regressor_values[0];
        let b = parameters.intercept_value;
        Self {
            a, b, max_access_size
        }
    }

    fn get_xs_ys(jsons: &Vec<BenchmarkJSON>, kdes: &Vec<BenchmarkKde>) -> (Vec<f64>, Vec<f64>) {
        let mut xs = Vec::new();
        let mut ys = Vec::new();
        for i in 0..jsons.len() {
            xs.push(jsons[i].access_size_in_bytes as f64);
            ys.push(kdes[i].get_global_maximum().0);
        }
        (xs, ys)
    }

    // TODO refactor me as well
    fn to_svg(&self, jsons: &Vec<BenchmarkJSON>, kdes: &Vec<BenchmarkKde>) -> String {
        let (xs, ys) = Self::get_xs_ys(jsons, kdes);
        let mut xs_ys: Vec<(f64, f64)> = xs.iter().cloned().zip(ys.iter().cloned()).collect();
        let pts = Plot::new(xs_ys).point_style(
            PointStyle::new()
            .colour("#ff0000")
            .marker(PointMarker::Cross)
        );
        let line = Plot::new(vec![(0.0f64, self.b), (self.max_access_size, self.max_access_size* self.a)])
            .line_style(
                LineStyle::new()
                .colour("#0000ff")
                .linejoin(LineJoin::Round)
            );
        let v = ContinuousView::new()
            .add(line)
            .add(pts)
            .x_label("Access Sizes in Bytes")
            .y_label("Expected Size in sec");
        Page::single(&v).to_svg().unwrap().to_string()
    }
}


fn create_model(model_path: &String, benchmark_file_path: &String, benchmarker_path: &String) -> Result<(), std::io::Error> {
    // create folders
    create_dir_all(model_path)?;

    let mut parent = PathBuf::from(benchmark_file_path);
    parent.pop();
    create_dir_all(parent)?;

    // Create Benchmarks
    let random_uncached = PerformanceBenchmark::new_random_uncached(benchmarker_path, benchmark_file_path);
    //random_uncached.run_and_save_all_benchmarks(model_path)?;

    // re-read benchmarks
    let benchmark_folder = random_uncached.get_benchmark_folder(model_path);
    let mut jsons = BenchmarkJSON::new_from_dir(&PathBuf::from(benchmark_folder));
    jsons.sort_by_key(|j| j.access_size_in_bytes);

    // Generate KDEs
    let kdes: Vec<BenchmarkKde> = jsons.iter().map(|j| j.generate_kde_from(100)).collect();
    let jsons_kdes: Vec<(&BenchmarkJSON, &BenchmarkKde)> = jsons.iter().zip(kdes.iter()).collect();

    // Create linear model
    let linear_model = LinearModel::from_jsons_kdes(&jsons, &kdes);
    let linear_model_svg = linear_model.to_svg(&jsons, &kdes);

    // Generate HTML report
    let ctx = ResultTemplate {
        benchmark_name: random_uncached.benchmark_type.to_string(),
        jsons_kdes,
        linear_model,
        linear_model_svg,
    };
    let html: String = ctx.render_once().unwrap();

    let html_template_path = format!("{}/{}", model_path, String::from("html"));
    create_dir(&html_template_path)?;

    let mut output = File::create(format!("{}/{}.html", &html_template_path, random_uncached.benchmark_type.to_string()))?;
    write!(output, "{}", html)?;

    Ok(())
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::CreateModel { to, file, benchmarker } => {
            if let Err(e) = validate_create_model(to, benchmarker) {
                let mut app = Cli::into_app();
                app.error(
                    clap::ErrorKind::InvalidValue,
                    format!("{:?}", e)
                ).exit();
            }
            match create_model(to, file, benchmarker)  {
                Ok(_) => { },
                Err(e) => eprintln!("{:?}", e),
            }
        },
        Commands::UseModel { .. } => {
        },
    }
}
