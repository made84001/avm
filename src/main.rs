mod cli;
mod setup;
mod downloader;
mod archive_reader;
extern crate hyper;
use std::process;
use std::env;

fn install(version: String) {
    let home_directory = setup::prepare();
    println!("Prepared avm directory at {}", home_directory);

    let path = match downloader::download_source(&version, &home_directory) {
        Ok(path)  => path,
        Err(err)    => {
            println!("Download failed:\n{}", err);
            std::process::exit(1)
        }
    };
    println!("Wrote archive to {}", path);
    let destination_path = setup::avm_directory();
    println!("Unzipping to {}", destination_path);

    setup::create_version_directory(&version);
    match archive_reader::decompress(path, destination_path, &version) {
        Ok(some) => {
            println!("Successfully unpacked archive");
            let stdout = String::from_utf8(some.stderr);
            println!("Exit status: {}", some.status);
            println!("{}", stdout.unwrap());
        },
        Err(err) => println!("Error occured\n{}", err)
    };
}

fn use_version(version: String) {
   if setup::has_version(&version) {
       println!("Has version {}", version);
   } else {
       println!("Version {} not installed", version);
   }
}

fn main() {
    println!("avm");
    let args: Vec<String> = env::args()
        .skip(1)
        .collect();
    let cmd_args = cli::process_arguments(&args);
    let mut version = String::new();

    match cmd_args.option
    {
        cli::CmdOption::Install => {
            version = cmd_args.args.first().unwrap().clone();
            install(version);
        },
        cli::CmdOption::Use => {
            version = cmd_args.args.first().unwrap().clone();
            use_version(version);
        },
        _ => {
            cli::help();
            std::process::exit(1)
        }
    };

}
