/*
This tool is part of the WhiteboxTools geospatial analysis library.
Authors: Dr. John Lindsay
Created: 20/06/2019
Last Modified: 20/06/2019
License: MIT
*/

use crate::raster::*;
use crate::rendering::html::*;
use crate::rendering::LineGraph;
use crate::structures::Array2D;
use crate::tools::*;
use crate::vector::{ShapeType, Shapefile};
use num_cpus;
use std::env;
use std::f64;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufWriter;
use std::io::{Error, ErrorKind};
use std::path;
use std::process::Command;
use std::sync::mpsc;
use std::sync::Arc;
use std::thread;

pub struct MultiscaleStdDevNormalsSignature {
    name: String,
    description: String,
    toolbox: String,
    parameters: Vec<ToolParameter>,
    example_usage: String,
}

impl MultiscaleStdDevNormalsSignature {
    pub fn new() -> MultiscaleStdDevNormalsSignature {
        // public constructor
        let name = "MultiscaleStdDevNormalsSignature".to_string();
        let toolbox = "Geomorphometric Analysis".to_string();
        let description =
            "Calculates the surface roughness for points over a range of spatial scales."
                .to_string();

        let mut parameters = vec![];
        parameters.push(ToolParameter {
            name: "Input DEM File".to_owned(),
            flags: vec!["-i".to_owned(), "--dem".to_owned()],
            description: "Input raster DEM file.".to_owned(),
            parameter_type: ParameterType::ExistingFile(ParameterFileType::Raster),
            default_value: None,
            optional: false,
        });

        parameters.push(ToolParameter {
            name: "Input Vector Points File".to_owned(),
            flags: vec!["--points".to_owned()],
            description: "Input vector points file.".to_owned(),
            parameter_type: ParameterType::ExistingFile(ParameterFileType::Vector(
                VectorGeometryType::Point,
            )),
            default_value: None,
            optional: false,
        });

        parameters.push(ToolParameter {
            name: "Output HTML File".to_owned(),
            flags: vec!["-o".to_owned(), "--output".to_owned()],
            description: "Output HTML file.".to_owned(),
            parameter_type: ParameterType::NewFile(ParameterFileType::Html),
            default_value: None,
            optional: false,
        });

        parameters.push(ToolParameter {
            name: "Minimum Search Neighbourhood Radius (grid cells)".to_owned(),
            flags: vec!["--min_scale".to_owned()],
            description: "Minimum search neighbourhood radius in grid cells.".to_owned(),
            parameter_type: ParameterType::Integer,
            default_value: Some("1".to_string()),
            optional: true,
        });

        parameters.push(ToolParameter {
            name: "Base Step Size".to_owned(),
            flags: vec!["--step".to_owned()],
            description: "Step size as any positive non-zero integer.".to_owned(),
            parameter_type: ParameterType::Integer,
            default_value: Some("1".to_owned()),
            optional: true,
        });

        parameters.push(ToolParameter {
            name: "Number of Steps".to_owned(),
            flags: vec!["--num_steps".to_owned()],
            description: "Number of steps".to_owned(),
            parameter_type: ParameterType::Integer,
            default_value: Some("10".to_owned()),
            optional: false,
        });

        parameters.push(ToolParameter {
            name: "Step Nonlinearity".to_owned(),
            flags: vec!["--step_nonlinearity".to_owned()],
            description: "Step nonlinearity factor (1.0-2.0 is typical)".to_owned(),
            parameter_type: ParameterType::Float,
            default_value: Some("1.0".to_owned()),
            optional: false,
        });

        let sep: String = path::MAIN_SEPARATOR.to_string();
        let p = format!("{}", env::current_dir().unwrap().display());
        let e = format!("{}", env::current_exe().unwrap().display());
        let mut short_exe = e
            .replace(&p, "")
            .replace(".exe", "")
            .replace(".", "")
            .replace(&sep, "");
        if e.contains(".exe") {
            short_exe += ".exe";
        }
        let usage = format!(">>.*{} -r={} -v --wd=\"*path*to*data*\" --dem=DEM.tif --points=sites.shp --output=roughness.html --min_scale=1 --step=5 --num_steps=100 --step_nonlinearity=1.5", short_exe, name).replace("*", &sep);

        MultiscaleStdDevNormalsSignature {
            name: name,
            description: description,
            toolbox: toolbox,
            parameters: parameters,
            example_usage: usage,
        }
    }
}

impl WhiteboxTool for MultiscaleStdDevNormalsSignature {
    fn get_source_file(&self) -> String {
        String::from(file!())
    }

    fn get_tool_name(&self) -> String {
        self.name.clone()
    }

    fn get_tool_description(&self) -> String {
        self.description.clone()
    }

    fn get_tool_parameters(&self) -> String {
        let mut s = String::from("{\"parameters\": [");
        for i in 0..self.parameters.len() {
            if i < self.parameters.len() - 1 {
                s.push_str(&(self.parameters[i].to_string()));
                s.push_str(",");
            } else {
                s.push_str(&(self.parameters[i].to_string()));
            }
        }
        s.push_str("]}");
        s
    }

    fn get_example_usage(&self) -> String {
        self.example_usage.clone()
    }

    fn get_toolbox(&self) -> String {
        self.toolbox.clone()
    }

    fn run<'a>(
        &self,
        args: Vec<String>,
        working_directory: &'a str,
        verbose: bool,
    ) -> Result<(), Error> {
        let mut input_file = String::new();
        let mut points_file = String::new();
        let mut output_file = String::new();
        let mut min_scale = 1isize;
        let mut step = 1isize;
        let mut num_steps = 10isize;
        let mut step_nonlinearity = 1.0f32;
        if args.len() == 0 {
            return Err(Error::new(
                ErrorKind::InvalidInput,
                "Tool run with no parameters.",
            ));
        }
        for i in 0..args.len() {
            let mut arg = args[i].replace("\"", "");
            arg = arg.replace("\'", "");
            let cmd = arg.split("="); // in case an equals sign was used
            let vec = cmd.collect::<Vec<&str>>();
            let mut keyval = false;
            if vec.len() > 1 {
                keyval = true;
            }
            let flag_val = vec[0].to_lowercase().replace("--", "-");
            if flag_val == "-i" || flag_val == "-input" || flag_val == "-dem" {
                input_file = if keyval {
                    vec[1].to_string()
                } else {
                    args[i + 1].to_string()
                };
            } else if flag_val == "-points" {
                points_file = if keyval {
                    vec[1].to_string()
                } else {
                    args[i + 1].to_string()
                };
            } else if flag_val == "-o" || flag_val == "-output" {
                output_file = if keyval {
                    vec[1].to_string()
                } else {
                    args[i + 1].to_string()
                };
            } else if flag_val == "-min_scale" {
                min_scale = if keyval {
                    vec[1].to_string().parse::<isize>().unwrap()
                } else {
                    args[i + 1].to_string().parse::<isize>().unwrap()
                };
                if min_scale < 1 {
                    min_scale = 1;
                }
            } else if flag_val == "-step" {
                step = if keyval {
                    vec[1].to_string().parse::<isize>().unwrap()
                } else {
                    args[i + 1].to_string().parse::<isize>().unwrap()
                };
            } else if flag_val == "-num_steps" {
                num_steps = if keyval {
                    vec[1].to_string().parse::<isize>().unwrap()
                } else {
                    args[i + 1].to_string().parse::<isize>().unwrap()
                };
            } else if flag_val == "-step_nonlinearity" {
                step_nonlinearity = if keyval {
                    vec[1]
                        .to_string()
                        .parse::<f32>()
                        .expect(&format!("Error parsing {}", flag_val))
                } else {
                    args[i + 1]
                        .to_string()
                        .parse::<f32>()
                        .expect(&format!("Error parsing {}", flag_val))
                };
            }
        }

        if step < 1 {
            eprintln!("Warning: Step value must be at least 1.0. Value set to 1.0.");
            step = 1;
        }

        if step_nonlinearity < 1.0 {
            eprintln!("Warning: Step nonlinearity value must be great than 1.0. Value set to 1.0.");
            step_nonlinearity = 1.0;
        }

        if step_nonlinearity > 4.0 {
            eprintln!("Warning: Step nonlinearity is set too high. Value reset to 4.0.");
            step_nonlinearity = 4.0;
        }

        if num_steps < 1 {
            eprintln!("Warning: Number of steps must be at least 1.");
            num_steps = 1;
        }

        if verbose {
            println!("***************{}", "*".repeat(self.get_tool_name().len()));
            println!("* Welcome to {} *", self.get_tool_name());
            println!("***************{}", "*".repeat(self.get_tool_name().len()));
        }

        let sep: String = path::MAIN_SEPARATOR.to_string();

        let mut progress: usize;
        let mut old_progress: usize = 1;

        if !input_file.contains(&sep) && !input_file.contains("/") {
            input_file = format!("{}{}", working_directory, input_file);
        }
        if !points_file.contains(&sep) && !points_file.contains("/") {
            points_file = format!("{}{}", working_directory, points_file);
        }
        if !output_file.contains(&sep) && !output_file.contains("/") {
            output_file = format!("{}{}", working_directory, output_file);
        }

        if verbose {
            println!("Reading DEM data...")
        };
        let input_raster = Raster::new(&input_file, "r")?; // Memory requirements: 2.0X, assuming data is stored as f32s
        let start = Instant::now();

        let configs = input_raster.configs.clone();
        let is_in_geographic_coordinates = input_raster.is_in_geographic_coordinates();
        let mut input = input_raster.get_data_as_f32_array2d(); // Memory requirements: 3.0X
        drop(input_raster); // Memory requirements: 1.0X

        let rows = configs.rows as isize;
        let columns = configs.columns as isize;
        let nodata = configs.nodata as f32;
        let min_val = configs.minimum as f32;

        let mut z_factor = 1f32;
        if is_in_geographic_coordinates {
            // calculate a new z-conversion factor
            let mut mid_lat = (configs.north - configs.south) as f32 / 2.0;
            if mid_lat <= 90.0 && mid_lat >= -90.0 {
                mid_lat = mid_lat.to_radians();
                z_factor = 1.0 / (113200.0 * mid_lat.cos());
            }
        }

        if verbose {
            println!("Reading points data...")
        };
        let points = Shapefile::read(&points_file)?;

        // make sure the input vector file is of points type
        if points.header.shape_type.base_shape_type() != ShapeType::Point {
            return Err(Error::new(
                ErrorKind::InvalidInput,
                "The input vector data must be of point base shape type.",
            ));
        }

        // read the points' corresponding row and columns into a list
        let mut signature_sites = vec![];
        let mut xdata = vec![];
        let mut ydata = vec![];
        let mut series_names = vec![];
        for record_num in 0..points.num_records {
            let record = points.get_record(record_num);
            let row =
                ((configs.north - record.points[0].y) / configs.resolution_y).floor() as isize; //input.get_row_from_y(record.points[0].y);
            let col = ((record.points[0].x - configs.west) / configs.resolution_x).floor() as isize; // input.get_column_from_x(record.points[0].x);
            if row >= 0 && col >= 0 && row < rows && col < columns {
                signature_sites.push((row, col));
                xdata.push(vec![]);
                ydata.push(vec![]);
                series_names.push(format!("Site {}", record_num + 1));
            }

            if verbose {
                progress = (100.0_f64 * row as f64 / (rows - 1) as f64) as usize;
                if progress != old_progress {
                    println!("Finding site row/column values: {}%", progress);
                    old_progress = progress;
                }
            }
        }

        if verbose {
            println!("Initializing grids...");
        }

        // calculate the 'n' itegral image
        let mut i_n: Array2D<u32> = Array2D::new(rows, columns, 0, 0)?;
        let mut sum: u32;
        let mut val: u32;
        for row in 0..rows {
            if row > 0 {
                sum = 0u32;
                for col in 0..columns {
                    sum += if input.get_value(row, col) != nodata {
                        input.set_value(row, col, input.get_value(row, col) - min_val);
                        1
                    } else {
                        //input.set_value(row, col, input.get_value(row, col) - min_val);
                        0
                    };
                    i_n.set_value(row, col, sum + i_n.get_value(row - 1, col));
                }
            } else {
                if input.get_value(0, 0) != nodata {
                    i_n.set_value(0, 0, 1);
                } else {
                    i_n.set_value(0, 0, 0);
                }
                for col in 1..columns {
                    val = if input.get_value(row, col) != nodata {
                        input.set_value(row, col, input.get_value(row, col) - min_val);
                        1
                    } else {
                        //input.set_value(row, col, input.get_value(row, col) - min_val);
                        0
                    };
                    i_n.set_value(row, col, val + i_n.get_value(row, col - 1));
                }
            }
        }

        let i_n = Arc::new(i_n);
        let input = Arc::new(input);

        ///////////////////////////////
        // Perform the main analysis //
        ///////////////////////////////
        let num_procs = num_cpus::get() as isize;
        for s in min_scale..(min_scale + num_steps) {
            let midpoint = min_scale
                + ((((s - min_scale) * step) as f32).powf(step_nonlinearity)).floor() as isize;
            println!("Loop {} / {}", s - min_scale + 1, num_steps);

            let filter_size = midpoint * 2 + 1;

            ////////////////////////////////////////
            // Smooth the DEM using Gaussian blur //
            ////////////////////////////////////////
            let mut smoothed_dem = input.duplicate(); // Memory requirements: 4.5X
            let sigma = (midpoint as f32 + 0.5) / 3f32;
            let pi: f32 = std::f32::consts::PI;
            if sigma < 1.8 && filter_size > 3 {
                let recip_root_2_pi_times_sigma_d = 1.0 / ((2.0 * pi).sqrt() * sigma);
                let two_sigma_sqr_d = 2.0 * sigma * sigma;

                // figure out the size of the filter
                let mut filter_size_smooth = 0;
                let mut weight: f32;
                for i in 0..250 {
                    weight = recip_root_2_pi_times_sigma_d
                        * (-1.0 * ((i * i) as f32) / two_sigma_sqr_d).exp();
                    if weight <= 0.001 {
                        filter_size_smooth = i * 2 + 1;
                        break;
                    }
                }

                // the filter dimensions must be odd numbers such that there is a middle pixel
                if filter_size_smooth % 2 == 0 {
                    filter_size_smooth += 1;
                }

                if filter_size_smooth < 3 {
                    filter_size_smooth = 3;
                }

                let num_pixels_in_filter = filter_size_smooth * filter_size_smooth;
                let mut d_x = vec![0isize; num_pixels_in_filter];
                let mut d_y = vec![0isize; num_pixels_in_filter];
                let mut weights = vec![0.0; num_pixels_in_filter];

                // fill the filter d_x and d_y values and the distance-weights
                let midpoint_smoothed: isize =
                    (filter_size_smooth as f32 / 2f32).floor() as isize + 1;
                let mut a = 0;
                let (mut x, mut y): (isize, isize);
                for row in 0..filter_size {
                    for col in 0..filter_size {
                        x = col as isize - midpoint_smoothed;
                        y = row as isize - midpoint_smoothed;
                        d_x[a] = x;
                        d_y[a] = y;
                        weight = recip_root_2_pi_times_sigma_d
                            * (-1.0 * ((x * x + y * y) as f32) / two_sigma_sqr_d).exp();
                        weights[a] = weight;
                        a += 1;
                    }
                }

                let d_x = Arc::new(d_x);
                let d_y = Arc::new(d_y);
                let weights = Arc::new(weights);

                let (tx, rx) = mpsc::channel();
                for tid in 0..num_procs {
                    let input = input.clone();
                    let d_x = d_x.clone();
                    let d_y = d_y.clone();
                    let weights = weights.clone();
                    let tx1 = tx.clone();
                    thread::spawn(move || {
                        let (mut sum, mut z_final): (f32, f32);
                        let mut z: f32;
                        let mut zn: f32;
                        let (mut x, mut y): (isize, isize);
                        for row in (0..rows).filter(|r| r % num_procs == tid) {
                            let mut data = vec![nodata; columns as usize];
                            for col in 0..columns {
                                z = input.get_value(row, col);
                                if z != nodata {
                                    sum = 0.0;
                                    z_final = 0.0;
                                    for a in 0..num_pixels_in_filter {
                                        x = col + d_x[a];
                                        y = row + d_y[a];
                                        zn = input.get_value(y, x);
                                        if zn != nodata {
                                            sum += weights[a];
                                            z_final += weights[a] * zn;
                                        }
                                    }
                                    data[col as usize] = z_final / sum;
                                }
                            }

                            tx1.send((row, data)).unwrap();
                        }
                    });
                }

                for _ in 0..rows {
                    let data = rx.recv().unwrap();
                    smoothed_dem.set_row_data(data.0, data.1);
                }
            } else if filter_size > 3 {
                // use a fast almost Gaussian filter for larger smoothing operations.
                let n = 4;
                let w_ideal = (12f32 * sigma * sigma / n as f32 + 1f32).sqrt();
                let mut wl = w_ideal.floor() as isize;
                if wl % 2 == 0 {
                    wl -= 1;
                } // must be an odd integer
                let wu = wl + 2;
                let m = ((12f32 * sigma * sigma
                    - (n * wl * wl) as f32
                    - (4 * n * wl) as f32
                    - (3 * n) as f32)
                    / (-4 * wl - 4) as f32)
                    .round() as isize;

                let mut integral: Array2D<f64> = Array2D::new(rows, columns, 0f64, nodata as f64)?; // Memory requirements: 6.5X
                let mut val: f32;
                let mut sum: f64;
                let mut i_prev: f64;
                let (mut x1, mut x2, mut y1, mut y2): (isize, isize, isize, isize);
                let mut num_cells: u32;

                for iteration_num in 0..n {
                    // Warning: midpoint shadows the loop iterator name here.
                    let midpoint = if iteration_num <= m {
                        (wl as f32 / 2f32).floor() as isize
                    } else {
                        (wu as f32 / 2f32).floor() as isize
                    };

                    if iteration_num == 0 {
                        // First iteration
                        // Create the integral images.
                        for row in 0..rows {
                            sum = 0f64;
                            // sum_n = 0;
                            for col in 0..columns {
                                val = input.get_value(row, col);
                                if val == nodata {
                                    val = 0f32;
                                    // } else {
                                    // sum_n += 1;
                                }
                                sum += val as f64;
                                if row > 0 {
                                    i_prev = integral.get_value(row - 1, col);
                                    integral.set_value(row, col, sum + i_prev);
                                } else {
                                    integral.set_value(row, col, sum);
                                }
                            }
                        }
                    } else {
                        // Create the integral image based on previous iteration output.
                        // We don't need to recalculate the num_cells integral image.
                        for row in 0..rows {
                            sum = 0f64;
                            for col in 0..columns {
                                val = smoothed_dem.get_value(row, col);
                                if val == nodata {
                                    val = 0f32;
                                }
                                sum += val as f64;
                                if row > 0 {
                                    i_prev = integral.get_value(row - 1, col);
                                    integral.set_value(row, col, sum + i_prev);
                                } else {
                                    integral.set_value(row, col, sum);
                                }
                            }
                        }
                    }

                    // Perform Filter
                    for row in 0..rows {
                        y1 = row - midpoint - 1;
                        if y1 < 0 {
                            y1 = 0;
                        }
                        y2 = row + midpoint;
                        if y2 >= rows {
                            y2 = rows - 1;
                        }

                        for col in 0..columns {
                            if input.get_value(row, col) != nodata {
                                x1 = col - midpoint - 1;
                                if x1 < 0 {
                                    x1 = 0;
                                }
                                x2 = col + midpoint;
                                if x2 >= columns {
                                    x2 = columns - 1;
                                }

                                num_cells = i_n.get_value(y2, x2) + i_n.get_value(y1, x1)
                                    - i_n.get_value(y1, x2)
                                    - i_n.get_value(y2, x1);
                                if num_cells > 0 {
                                    sum = integral.get_value(y2, x2) + integral.get_value(y1, x1)
                                        - integral.get_value(y1, x2)
                                        - integral.get_value(y2, x1);
                                    smoothed_dem.set_value(
                                        row,
                                        col,
                                        (sum / num_cells as f64) as f32,
                                    );
                                } else {
                                    // should never hit here since input(row, col) != nodata above, therefore, num_cells >= 1
                                    smoothed_dem.set_value(row, col, 0f32);
                                }
                            }
                        }
                    }
                }
                // Memory requirements: 4.5X (integral is dropped at end of scope)
            }

            ///////////////////////////
            // Calculate the normals //
            ///////////////////////////
            let resx = configs.resolution_x as f32;
            let resy = configs.resolution_y as f32;
            let smoothed_dem = Arc::new(smoothed_dem);
            let (tx, rx) = mpsc::channel();
            for tid in 0..num_procs {
                let smoothed_dem = smoothed_dem.clone();
                let tx = tx.clone();
                thread::spawn(move || {
                    let dx = [1, 1, 1, 0, -1, -1, -1, 0];
                    let dy = [-1, 0, 1, 1, 1, 0, -1, -1];
                    let mut n: [f32; 8] = [0.0; 8];
                    let mut z: f32;
                    let (mut fx, mut fy): (f32, f32);
                    let fz = 1f32;
                    let fz_sqrd = fz * fz;
                    let mut magnitude: f32;
                    let resx8 = resx * 8f32;
                    let resy8 = resy * 8f32;
                    for row in (0..rows).filter(|r| r % num_procs == tid) {
                        let mut xdata = vec![0f64; columns as usize];
                        let mut ydata = vec![0f64; columns as usize];
                        let mut zdata = vec![0f64; columns as usize];
                        for col in 0..columns {
                            z = smoothed_dem.get_value(row, col);
                            if z != nodata {
                                for c in 0..8 {
                                    n[c] = smoothed_dem.get_value(row + dy[c], col + dx[c]);
                                    if n[c] != nodata {
                                        n[c] = n[c] * z_factor;
                                    } else {
                                        n[c] = z * z_factor;
                                    }
                                }
                                fx = (n[2] - n[4] + 2.0 * (n[1] - n[5]) + n[0] - n[6]) / resx8;
                                fy = (n[6] - n[4] + 2.0 * (n[7] - n[3]) + n[0] - n[2]) / resy8;
                                if fx != 0f32 || fy != 0f32 {
                                    magnitude = (fx * fx + fy * fy + fz_sqrd).sqrt();
                                    xdata[col as usize] = (-fx / magnitude) as f64;
                                    ydata[col as usize] = (-fy / magnitude) as f64;
                                    zdata[col as usize] = (fz / magnitude) as f64;
                                } else {
                                    xdata[col as usize] = 0f64;
                                    ydata[col as usize] = 0f64;
                                    zdata[col as usize] = 1f64;
                                }
                            }
                        }
                        tx.send((row, xdata, ydata, zdata)).unwrap();
                    }
                });
            }

            // These have to be f64s to hold the precision of the integral images
            let mut xc: Array2D<f64> = Array2D::new(rows, columns, 0f64, -1f64)?; // Memory requirements: 6.5X
            let mut yc: Array2D<f64> = Array2D::new(rows, columns, 0f64, -1f64)?; // Memory requirements: 8.5X
            let mut zc: Array2D<f64> = Array2D::new(rows, columns, 0f64, -1f64)?; // Memory requirements: 10.5X
            for _ in 0..rows {
                let data = rx.recv().unwrap();
                xc.set_row_data(data.0, data.1);
                yc.set_row_data(data.0, data.2);
                zc.set_row_data(data.0, data.3);
            }

            drop(smoothed_dem); // Memory requirements: 9.5X

            ////////////////////////////////////////
            // Convert normals to integral images //
            ////////////////////////////////////////
            let (mut sumx, mut sumy, mut sumz): (f64, f64, f64);
            for row in 0..rows {
                if row > 0 {
                    sumx = 0f64;
                    sumy = 0f64;
                    sumz = 0f64;
                    for col in 0..columns {
                        sumx += xc.get_value(row, col);
                        sumy += yc.get_value(row, col);
                        sumz += zc.get_value(row, col);
                        xc.set_value(row, col, sumx + xc.get_value(row - 1, col));
                        yc.set_value(row, col, sumy + yc.get_value(row - 1, col));
                        zc.set_value(row, col, sumz + zc.get_value(row - 1, col));
                    }
                } else {
                    for col in 1..columns {
                        xc.increment(row, col, xc.get_value(row, col - 1));
                        yc.increment(row, col, yc.get_value(row, col - 1));
                        zc.increment(row, col, zc.get_value(row, col - 1));
                    }
                }
            }

            ////////////////////////////////////////////////////////////////
            // Calculate the spherical standard deviations of the normals //
            ////////////////////////////////////////////////////////////////
            let (mut sumx, mut sumy, mut sumz): (f64, f64, f64);
            let mut mean: f32;
            let (mut x1, mut x2, mut y1, mut y2): (isize, isize, isize, isize);
            let mut n: f32;
            let mut z: f32;
            for site in 0..signature_sites.len() {
                let (row, col) = signature_sites[site];
                z = input.get_value(row, col);
                if z != nodata {
                    y1 = row - midpoint - 1;
                    if y1 < 0 {
                        y1 = 0;
                    }

                    y2 = row + midpoint;
                    if y2 >= rows {
                        y2 = rows - 1;
                    }

                    x1 = col - midpoint - 1;
                    if x1 < 0 {
                        x1 = 0;
                    }

                    x2 = col + midpoint;
                    if x2 >= columns {
                        x2 = columns - 1;
                    }

                    n = (i_n.get_value(y2, x2) + i_n.get_value(y1, x1)
                        - i_n.get_value(y1, x2)
                        - i_n.get_value(y2, x1)) as f32;
                    if n > 0f32 {
                        sumx = xc.get_value(y2, x2) + xc.get_value(y1, x1)
                            - xc.get_value(y1, x2)
                            - xc.get_value(y2, x1);
                        sumy = yc.get_value(y2, x2) + yc.get_value(y1, x1)
                            - yc.get_value(y1, x2)
                            - yc.get_value(y2, x1);
                        sumz = zc.get_value(y2, x2) + zc.get_value(y1, x1)
                            - zc.get_value(y1, x2)
                            - zc.get_value(y2, x1);
                        mean = ((sumx * sumx + sumy * sumy + sumz * sumz) as f32).sqrt() / n;
                        if mean > 1f32 {
                            mean = 1f32;
                        }
                        xdata[site].push((midpoint) as f64);
                        ydata[site].push((-2f32 * mean.ln()).sqrt().to_degrees() as f64);
                    }
                }
            }

            // drop(xc); // Memory requirements: 7.5X (automatically freed at end of scope)
            // drop(yc); // Memory requirements: 5.5X
            // drop(zc); // Memory requirements: 3.5X

            // Update progress
            if verbose {
                progress = (s as f32 / num_steps as f32 * 100f32) as usize;
                if progress != old_progress {
                    println!("Progress: {}%", progress);
                    old_progress = progress;
                }
            }
        }

        let elapsed_time = get_formatted_elapsed_time(start);
        if verbose {
            println!(
                "\n{}",
                &format!("Elapsed Time (excluding I/O): {}", elapsed_time)
            );
        }

        let f = File::create(output_file.clone())?;
        let mut writer = BufWriter::new(f);

        writer.write_all(&r#"<!DOCTYPE html PUBLIC \"-//W3C//DTD XHTML 1.0 Transitional//EN\" \"http://www.w3.org/TR/xhtml1/DTD/xhtml1-transitional.dtd\">
        <head>
            <meta content=\"text/html; charset=UTF-8\" http-equiv=\"content-type\">
            <title>Multiscale Roughness</title>"#.as_bytes())?;

        // get the style sheet
        writer.write_all(&get_css().as_bytes())?;

        writer.write_all(
            &r#"</head>
        <body>
            <h1>Multiscale Roughness</h1>"#
                .as_bytes(),
        )?;

        writer
            .write_all((format!("<p><strong>Input DEM</strong>: {}<br>", input_file)).as_bytes())?;

        writer.write_all(("</p>").as_bytes())?;

        let multiples = xdata.len() > 2 && xdata.len() < 12;

        let graph = LineGraph {
            parent_id: "graph".to_string(),
            width: 700f64,
            height: 500f64,
            data_x: xdata.clone(),
            data_y: ydata.clone(),
            series_labels: series_names.clone(),
            x_axis_label: "Filter Radius (cells)".to_string(),
            y_axis_label: "Roughness (degrees)".to_string(),
            draw_points: false,
            draw_gridlines: true,
            draw_legend: multiples,
            draw_grey_background: false,
        };

        writer.write_all(
            &format!("<div id='graph' align=\"center\">{}</div>", graph.get_svg()).as_bytes(),
        )?;

        writer.write_all("</body>".as_bytes())?;

        let _ = writer.flush();

        if verbose {
            if cfg!(target_os = "macos") || cfg!(target_os = "ios") {
                let output = Command::new("open")
                    .arg(output_file.clone())
                    .output()
                    .expect("failed to execute process");

                let _ = output.stdout;
            } else if cfg!(target_os = "windows") {
                let output = Command::new("explorer.exe")
                    .arg(output_file.clone())
                    .output()
                    .expect("failed to execute process");

                let _ = output.stdout;
            } else if cfg!(target_os = "linux") {
                let output = Command::new("xdg-open")
                    .arg(output_file.clone())
                    .output()
                    .expect("failed to execute process");

                let _ = output.stdout;
            }

            println!("Complete! Please see {} for output.", output_file);
        }

        Ok(())
    }
}
