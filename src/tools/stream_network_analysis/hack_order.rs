/* 
This tool is part of the WhiteboxTools geospatial analysis library.
Authors: Dr. John Lindsay
Created: June 25, 2017
Last Modified: November 16, 2017
License: MIT
*/
extern crate time;

use std::env;
use std::path;
use std::f64;
use raster::*;
use std::io::{Error, ErrorKind};
use structures::Array2D;
use tools::*;

pub struct HackStreamOrder {
    name: String,
    description: String,
    parameters: Vec<ToolParameter>,
    example_usage: String,
}

impl HackStreamOrder {
    pub fn new() -> HackStreamOrder { // public constructor
        let name = "HackStreamOrder".to_string();
        
        let description = "Assigns the Hack stream order to each tributary in a stream network.".to_string();
        
        // let mut parameters = "--d8_pntr          Input D8 pointer raster file.\n".to_owned();
        // parameters.push_str("--streams          Input streams raster file.\n");
        // parameters.push_str("-o, --output       Output raster file.\n");
        // parameters.push_str("--esri_pntr        Flag indicating whether the D8 pointer uses the ESRI style scheme (default is false).\n");
        // parameters.push_str("--zero_background  Flag indicating whether the background value of zero should be used.\n");

        let mut parameters = vec![];
        parameters.push(ToolParameter{
            name: "Input D8 Pointer File".to_owned(), 
            flags: vec!["--d8_pntr".to_owned()], 
            description: "Input raster D8 pointer file.".to_owned(),
            parameter_type: ParameterType::ExistingFile(ParameterFileType::Raster),
            default_value: None,
            optional: false
        });

        parameters.push(ToolParameter{
            name: "Input Streams File".to_owned(), 
            flags: vec!["--streams".to_owned()], 
            description: "Input raster streams file.".to_owned(),
            parameter_type: ParameterType::ExistingFile(ParameterFileType::Raster),
            default_value: None,
            optional: false
        });

        parameters.push(ToolParameter{
            name: "Output File".to_owned(), 
            flags: vec!["-o".to_owned(), "--output".to_owned()], 
            description: "Output raster file.".to_owned(),
            parameter_type: ParameterType::NewFile(ParameterFileType::Raster),
            default_value: None,
            optional: false
        });

        parameters.push(ToolParameter{
            name: "Does the pointer file use the ESRI pointer scheme?".to_owned(), 
            flags: vec!["--esri_pntr".to_owned()], 
            description: "D8 pointer uses the ESRI style scheme.".to_owned(),
            parameter_type: ParameterType::Boolean,
            default_value: Some("false".to_owned()),
            optional: true
        });

        parameters.push(ToolParameter{
            name: "Should a background value of zero be used?".to_owned(), 
            flags: vec!["--zero_background".to_owned()], 
            description: "Flag indicating whether a background value of zero should be used.".to_owned(),
            parameter_type: ParameterType::Boolean,
            default_value: None,
            optional: true
        });

        let sep: String = path::MAIN_SEPARATOR.to_string();
        let p = format!("{}", env::current_dir().unwrap().display());
        let e = format!("{}", env::current_exe().unwrap().display());
        let mut short_exe = e.replace(&p, "").replace(".exe", "").replace(".", "").replace(&sep, "");
        if e.contains(".exe") {
            short_exe += ".exe";
        }
        let usage = format!(">>.*{0} -r={1} --wd=\"*path*to*data*\" --d8_pntr=D8.dep --streams=streams.dep -o=output.dep
>>.*{0} -r={1} --wd=\"*path*to*data*\" --d8_pntr=D8.flt --streams=streams.flt -o=output.flt --esri_pntr --zero_background", short_exe, name).replace("*", &sep);
    
        HackStreamOrder { name: name, description: description, parameters: parameters, example_usage: usage }
    }
}

impl WhiteboxTool for HackStreamOrder {
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

    fn run<'a>(&self, args: Vec<String>, working_directory: &'a str, verbose: bool) -> Result<(), Error> {
        let mut d8_file = String::new();
        let mut streams_file = String::new();
        let mut output_file = String::new();
        let mut esri_style = false;
        let mut background_val = f64::NEG_INFINITY;
        
        if args.len() == 0 {
            return Err(Error::new(ErrorKind::InvalidInput,
                                "Tool run with no paramters. Please see help (-h) for parameter descriptions."));
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
            if vec[0].to_lowercase() == "-d8_pntr" || vec[0].to_lowercase() == "--d8_pntr" {
                if keyval {
                    d8_file = vec[1].to_string();
                } else {
                    d8_file = args[i+1].to_string();
                }
            } else if vec[0].to_lowercase() == "-streams" || vec[0].to_lowercase() == "--streams" {
                if keyval {
                    streams_file = vec[1].to_string();
                } else {
                    streams_file = args[i+1].to_string();
                }
            } else if vec[0].to_lowercase() == "-o" || vec[0].to_lowercase() == "--output" {
                if keyval {
                    output_file = vec[1].to_string();
                } else {
                    output_file = args[i+1].to_string();
                }
            } else if vec[0].to_lowercase() == "-esri_pntr" || vec[0].to_lowercase() == "--esri_pntr" || vec[0].to_lowercase() == "--esri_style" {
                esri_style = true;
            } else if vec[0].to_lowercase() == "-zero_background" || vec[0].to_lowercase() == "--zero_background" {
                background_val = 0f64;
            }
        }

        if verbose {
            println!("***************{}", "*".repeat(self.get_tool_name().len()));
            println!("* Welcome to {} *", self.get_tool_name());
            println!("***************{}", "*".repeat(self.get_tool_name().len()));
        }

        let sep: String = path::MAIN_SEPARATOR.to_string();

        let mut progress: usize;
        let mut old_progress: usize = 1;

        if !d8_file.contains(&sep) {
            d8_file = format!("{}{}", working_directory, d8_file);
        }
        if !streams_file.contains(&sep) {
            streams_file = format!("{}{}", working_directory, streams_file);
        }
        if !output_file.contains(&sep) {
            output_file = format!("{}{}", working_directory, output_file);
        }

        if verbose { println!("Reading pointer data...") };
        let pntr = Raster::new(&d8_file, "r")?;
        let pntr_nodata = pntr.configs.nodata;
        if verbose { println!("Reading streams data...") };
        let streams = Raster::new(&streams_file, "r")?;
        
        let start = time::now();

        let rows = pntr.configs.rows as isize;
        let columns = pntr.configs.columns as isize;
        let num_cells = pntr.num_cells();
        let nodata = streams.configs.nodata;
        if background_val == f64::NEG_INFINITY {
            background_val = nodata;
        }
        let cell_size_x = streams.configs.resolution_x;
        let cell_size_y = streams.configs.resolution_y;
        let diag_cell_size = (cell_size_x * cell_size_x + cell_size_y * cell_size_y).sqrt();
        
        
        // make sure the input files have the same size
        if streams.configs.rows != pntr.configs.rows || streams.configs.columns != pntr.configs.columns {
            return Err(Error::new(ErrorKind::InvalidInput,
                                "The input files must have the same number of rows and columns and spatial extent."));
        }

        let mut output = Raster::initialize_using_file(&output_file, &streams);
        let mut stack = Vec::with_capacity((rows * columns) as usize);

        let mut upstream_stack = vec![];
        let mut hack_order = vec![];

        // calculate the number of inflowing cells
        let mut num_inflowing: Array2D<i8> = Array2D::new(rows, columns, -1, -1)?;
        let mut trib_length: Array2D<f64> = Array2D::new(rows, columns, nodata, nodata)?;
        let d_x = [ 1, 1, 1, 0, -1, -1, -1, 0 ];
        let d_y = [ -1, 0, 1, 1, 1, 0, -1, -1 ];
        let grid_lengths = [diag_cell_size, cell_size_x, diag_cell_size, cell_size_y, diag_cell_size, cell_size_x, diag_cell_size, cell_size_y];
        let mut inflowing_vals = [ 16f64, 32f64, 64f64, 128f64, 1f64, 2f64, 4f64, 8f64 ];
        if esri_style {
            inflowing_vals = [ 8f64, 16f64, 32f64, 64f64, 128f64, 1f64, 2f64, 4f64 ];
        }
        let mut num_solved_cells = 0;
        let mut count: i8;
        let mut current_id = 1f64;
        for row in 0..rows {
            for col in 0..columns {
                if streams[(row, col)] > 0.0 {
                    count = 0i8;
                    for i in 0..8 {
                        if streams[(row + d_y[i], col + d_x[i])] > 0.0 &&
                            pntr[(row + d_y[i], col + d_x[i])] == inflowing_vals[i] {
                            count += 1;
                        }
                    }
                    num_inflowing[(row, col)] = count;
                    if count == 0 {
                        // It's a headwater; add it to the stack
                        stack.push((row, col));
                        output[(row, col)] = current_id;
                        current_id += 1f64;
                        trib_length[(row, col)] = 0f64;
                        hack_order.push(0f64);
                    }
                } else {
                    if pntr[(row, col)] != pntr_nodata {
                        output[(row, col)] = background_val;
                    } else {
                        output[(row, col)] = nodata;
                    }
                    num_solved_cells += 1;
                }
            }
            if verbose {
                progress = (100.0_f64 * num_solved_cells as f64 / (num_cells - 1) as f64) as usize;
                if progress != old_progress {
                    println!("Progress: {}%", progress);
                    old_progress = progress;
                }
            }
        }

        // Create a mapping from the pointer values to cells offsets.
        // This may seem wasteful, using only 8 of 129 values in the array,
        // but the mapping method is far faster than calculating z.ln() / ln(2.0).
        // It's also a good way of allowing for different point styles.
        let mut pntr_matches: [usize; 129] = [999usize; 129];
        if !esri_style {
            // This maps Whitebox-style D8 pointer values
            // onto the cell offsets in d_x and d_y.
            pntr_matches[1] = 0usize;
            pntr_matches[2] = 1usize;
            pntr_matches[4] = 2usize;
            pntr_matches[8] = 3usize;
            pntr_matches[16] = 4usize;
            pntr_matches[32] = 5usize;
            pntr_matches[64] = 6usize;
            pntr_matches[128] = 7usize;
        } else {
            // This maps Esri-style D8 pointer values
            // onto the cell offsets in d_x and d_y.
            pntr_matches[1] = 1usize;
            pntr_matches[2] = 2usize;
            pntr_matches[4] = 3usize;
            pntr_matches[8] = 4usize;
            pntr_matches[16] = 5usize;
            pntr_matches[32] = 6usize;
            pntr_matches[64] = 7usize;
            pntr_matches[128] = 0usize;
        }

        let (mut row, mut col): (isize, isize);
        let (mut row_n, mut col_n): (isize, isize);
        let mut dir: usize;
        let mut length: f64;
        let mut val: f64;
        let mut c: usize;
        while !stack.is_empty() {
            let cell = stack.pop().unwrap();
            row = cell.0;
            col = cell.1;

            val = output[(row, col)];

            // find the downstream cell
            dir = pntr[(row, col)] as usize;
            if dir > 0 {
                if dir > 128 || pntr_matches[dir] == 999 {
                    return Err(Error::new(ErrorKind::InvalidInput,
                        "An unexpected value has been identified in the pointer image. This tool requires a pointer grid that has been created using either the D8 or Rho8 tools."));
                }

                c = pntr_matches[dir];
                row_n = row + d_y[c];
                col_n = col + d_x[c];

                length = trib_length[(row, col)] + grid_lengths[c];
                if trib_length[(row_n, col_n)] < length || trib_length[(row_n, col_n)] == nodata {
                    trib_length[(row_n, col_n)] = length;
                    output[(row_n, col_n)] = val;
                }

                num_inflowing.decrement(row_n, col_n, 1);
                if num_inflowing[(row_n, col_n)] == 0 {
                    stack.push((row_n, col_n));
                }
            } else {
                upstream_stack.push((row, col));
            }

            if verbose {
                progress = (100.0_f64 * num_solved_cells as f64 / (num_cells - 1) as f64) as usize;
                if progress != old_progress {
                    println!("Progress: {}%", progress);
                    old_progress = progress;
                }
            }
        }

        let mut trib_val: f64;
        let mut trib_val_n: f64;
        let mut ho: f64;
        while !upstream_stack.is_empty() {
            let cell = upstream_stack.pop().unwrap();
            row = cell.0;
            col = cell.1;

            trib_val = output[(row, col)];
            ho = hack_order[(trib_val - 1.0) as usize];
            if ho == 0.0 {
                ho = 1.0;
                output[(row, col)] = 1.0;
            } else {
                output[(row, col)] = ho;
            }

            // find any inflowing stream cells
            for i in 0..8 {
                row_n = row + d_y[i];
                col_n = col + d_x[i];
                if streams[(row_n, col_n)] > 0.0 && pntr[(row_n, col_n)] == inflowing_vals[i] {
                    trib_val_n = output[(row_n, col_n)];
                    if trib_val_n != trib_val {
                        hack_order[(trib_val_n - 1.0) as usize] = ho + 1.0;
                    }
                    upstream_stack.push((row_n, col_n));
                }
            }
        }

        let end = time::now();
        let elapsed_time = end - start;
        output.configs.palette = "qual.plt".to_string();
        output.configs.photometric_interp = PhotometricInterpretation::Categorical;
        output.add_metadata_entry(format!("Created by whitebox_tools\' {} tool", self.get_tool_name()));
        output.add_metadata_entry(format!("Input d8 pointer file: {}", d8_file));
        output.add_metadata_entry(format!("Input streams file: {}", streams_file));
        output.add_metadata_entry(format!("Elapsed Time (excluding I/O): {}", elapsed_time).replace("PT", ""));

        if verbose { println!("Saving data...") };
        let _ = match output.write() {
            Ok(_) => if verbose { println!("Output file written") },
            Err(e) => return Err(e),
        };

        println!("{}", &format!("Elapsed Time (excluding I/O): {}", elapsed_time).replace("PT", ""));

        Ok(())
    }
}