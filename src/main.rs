use clap::{Parser, Subcommand};
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use serde_with::FromInto;
use serde_with::{serde, serde_as, As};
use std::fs::File;

// Thank you @anden3 for helping with this <3
#[derive(Deserialize, Serialize, Debug)]
struct OptionalFloat(pub String);

impl From<OptionalFloat> for Option<f64> {
    fn from(val: OptionalFloat) -> Self {
        match val.0.trim() {
            "Error" | "NA" => None,
            s => Some(s.parse::<f64>().unwrap()),
        }
    }
}

#[derive(Deserialize, Serialize)]
struct OptionalU64(pub String);

impl From<OptionalU64> for Option<u64> {
    fn from(val: OptionalU64) -> Self {
        match val.0.trim() {
            "Error" | "NA" => None,
            s => Some(s.parse::<u64>().unwrap()),
        }
    }
}

#[derive(Deserialize, Serialize)]
struct OptionalBool(pub String);

impl From<OptionalBool> for Option<bool> {
    fn from(val: OptionalBool) -> Self {
        match val.0.trim() {
            "1" => Some(true),
            "0" => Some(false),
            "Error" | "NA" | _ => None,
        }
    }
}

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
#[clap(propagate_version = true)]
struct Args {
    /// Path to FrameView CSV file
    #[clap(short = 'i', long = "in", value_name = "PATH", value_hint = clap::ValueHint::DirPath)]
    in_file: std::path::PathBuf,

    #[clap(subcommand)]
    command: Commands,
}

// TODO: Average frame time
// TODO: Average temp(s)
// TODO: Overview command (A nice print out of the averages and stats)
#[derive(Subcommand, Debug)]
enum Commands {
    /// Outputs the average FPS
    AverageFPS,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    match &args.command {
        Commands::AverageFPS => average_fps(args).await,
    }
}

async fn average_fps(args: Args) {
    let mut entry_vec: Vec<FrameViewCSVEntry> = Vec::new();

    let file = File::open(&args.in_file.as_path()).expect("Unable to open file");

    let mut reader = csv::Reader::from_reader(file);

    for line in reader.deserialize() {
        let record: FrameViewCSVEntry = match line {
            Ok(l) => l,
            Err(e) => panic!("{}", e),
        };
        // println!("{:?}", record);

        entry_vec.push(record);
    }

    let mut vec_of_frame_times = Vec::new();

    for entry in entry_vec {
        match entry.ms_between_presents {
            None => {}
            Some(x) => {
                vec_of_frame_times.push(x);
            }
        }
    }

    let vec_of_fps: Vec<f64> = vec_of_frame_times.par_iter().map(|x| 1000f64 / x).collect();

    let average: f64 = vec_of_fps.par_iter().sum::<f64>() / vec_of_fps.len() as f64;

    println!("{:?}", average)
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde_as]
#[serde(rename_all = "camelCase")]
struct FrameViewCSVEntry {
    #[serde(rename = "Application")]
    pub application: String,
    #[serde(rename = "GPU")]
    pub gpu: String,
    #[serde(rename = "CPU")]
    pub cpu: String,
    #[serde(rename = "Resolution")]
    pub resolution: String,
    #[serde(rename = "Runtime")]
    pub runtime: String,
    #[serde(rename = "AllowsTearing", with = "As::<FromInto<OptionalBool>>")]
    pub allows_tearing: Option<bool>,
    #[serde(rename = "ProcessID", with = "As::<FromInto<OptionalU64>>")]
    pub process_id: Option<u64>,
    #[serde(rename = "SwapChainAddress")]
    pub swap_chain_address: String,

    #[serde(rename = "SyncInterval", with = "As::<FromInto<OptionalFloat>>")]
    pub sync_interval: Option<f64>,
    #[serde(rename = "PresentFlags", with = "As::<FromInto<OptionalU64>>")]
    pub present_flags: Option<u64>,
    #[serde(rename = "PresentMode")]
    pub present_mode: String,
    #[serde(rename = "Dropped", with = "As::<FromInto<OptionalBool>>")]
    pub dropped: Option<bool>,

    #[serde(rename = "TimeInSeconds", with = "As::<FromInto<OptionalFloat>>")]
    pub time_in_seconds: Option<f64>,

    #[serde(rename = "MsBetweenPresents", with = "As::<FromInto<OptionalFloat>>")]
    pub ms_between_presents: Option<f64>,

    #[serde(
        rename = "MsBetweenDisplayChange",
        with = "As::<FromInto<OptionalFloat>>"
    )]
    pub ms_between_display_change: Option<f64>,

    #[serde(rename = "MsInPresentAPI", with = "As::<FromInto<OptionalFloat>>")]
    pub ms_in_present_api: Option<f64>,

    #[serde(
        rename = "MsRenderPresentLatency",
        with = "As::<FromInto<OptionalFloat>>"
    )]
    pub ms_render_present_latency: Option<f64>,

    #[serde(rename = "MsUntilDisplayed", with = "As::<FromInto<OptionalFloat>>")]
    pub ms_until_displayed: Option<f64>,

    #[serde(rename = "Render Queue Depth", with = "As::<FromInto<OptionalFloat>>")]
    pub render_queue_depth: Option<f64>,

    #[serde(rename = "GPU0Clk(MHz)", with = "As::<FromInto<OptionalFloat>>")]
    pub gpu0clk_mhz: Option<f64>,

    #[serde(rename = "GPU0MemClk(MHz)", with = "As::<FromInto<OptionalFloat>>")]
    pub gpu0mem_clk_mhz: Option<f64>,

    #[serde(rename = "GPU0Util(%)", with = "As::<FromInto<OptionalFloat>>")]
    pub gpu0util: Option<f64>,

    #[serde(rename = "GPU0Temp(C)", with = "As::<FromInto<OptionalFloat>>")]
    pub gpu0temp_c: Option<f64>,

    #[serde(rename = "GPU1Clk(MHz)", with = "As::<FromInto<OptionalFloat>>")]
    pub gpu1clk_mhz: Option<f64>,

    #[serde(rename = "GPU1MemClk(MHz)", with = "As::<FromInto<OptionalFloat>>")]
    pub gpu1mem_clk_mhz: Option<f64>,

    #[serde(rename = "GPU1Util(%)", with = "As::<FromInto<OptionalFloat>>")]
    pub gpu1util: Option<f64>,

    #[serde(rename = "GPU1Temp(C)", with = "As::<FromInto<OptionalFloat>>")]
    pub gpu1temp_c: Option<f64>,

    #[serde(rename = "PCAT Power Total(W)", with = "As::<FromInto<OptionalFloat>>")]
    pub pcat_power_total_w: Option<f64>,

    #[serde(
        rename = "Perf/W Total(F/J) (PCAT)",
        with = "As::<FromInto<OptionalFloat>>"
    )]
    pub perf_w_total_f_j_pcat: Option<f64>,

    #[serde(
        rename = "Perf/W Total(F/J) (API)",
        with = "As::<FromInto<OptionalFloat>>"
    )]
    pub perf_w_total_f_j_apit: Option<f64>,

    #[serde(
        rename = "Perf/W GPUOnly(F/J) (API)",
        with = "As::<FromInto<OptionalFloat>>"
    )]
    pub perf_w_gpu_only_f_j_api: Option<f64>,

    #[serde(
        rename = "Perf/W Total-USBC(F/J) (API)",
        with = "As::<FromInto<OptionalFloat>>"
    )]
    pub perf_w_total_usbc_f_j_api: Option<f64>,

    #[serde(rename = "GPUOnlyPwr(W) (API)", with = "As::<FromInto<OptionalFloat>>")]
    pub gpuonly_pwr_w_api: Option<f64>,

    #[serde(
        rename = "NV-Total-USBCPwr(W) (API)",
        with = "As::<FromInto<OptionalFloat>>"
    )]
    pub nv_total_usbcpwr_w_api: Option<f64>,

    #[serde(rename = "NV Pwr(W) (API)", with = "As::<FromInto<OptionalFloat>>")]
    pub nv_pwr_w_api: Option<f64>,

    #[serde(rename = "AMDPwr(W) (API)", with = "As::<FromInto<OptionalFloat>>")]
    pub amdpwr_w_api: Option<f64>,

    #[serde(rename = "CPUClk(MHz)", with = "As::<FromInto<OptionalFloat>>")]
    pub cpuclk_mhz: Option<f64>,

    #[serde(rename = "CPUUtil(%)", with = "As::<FromInto<OptionalFloat>>")]
    pub cpuutil: Option<f64>,

    #[serde(rename = "CPU Package Temp(C)", with = "As::<FromInto<OptionalFloat>>")]
    pub cpu_package_temp_c: Option<f64>,

    #[serde(
        rename = "CPU Package Power(W)",
        with = "As::<FromInto<OptionalFloat>>"
    )]
    pub cpu_package_power_w: Option<f64>,

    #[serde(rename = "CPU TDP (W)", with = "As::<FromInto<OptionalFloat>>")]
    pub cpu_tdp_w: Option<f64>,

    #[serde(rename = "CPUCoreUtil%[ 0]", with = "As::<FromInto<OptionalFloat>>")]
    pub cpucore_util_0: Option<f64>,

    #[serde(rename = "CPUCoreUtil%[ 1]", with = "As::<FromInto<OptionalFloat>>")]
    pub cpucore_util_1: Option<f64>,

    #[serde(rename = "CPUCoreUtil%[ 2]", with = "As::<FromInto<OptionalFloat>>")]
    pub cpucore_util_2: Option<f64>,

    #[serde(rename = "CPUCoreUtil%[ 3]", with = "As::<FromInto<OptionalFloat>>")]
    pub cpucore_util_3: Option<f64>,

    #[serde(rename = "CPUCoreUtil%[ 4]", with = "As::<FromInto<OptionalFloat>>")]
    pub cpucore_util_4: Option<f64>,

    #[serde(rename = "CPUCoreUtil%[ 5]", with = "As::<FromInto<OptionalFloat>>")]
    pub cpucore_util_5: Option<f64>,

    #[serde(rename = "CPUCoreUtil%[ 6]", with = "As::<FromInto<OptionalFloat>>")]
    pub cpucore_util_6: Option<f64>,

    #[serde(rename = "CPUCoreUtil%[ 7]", with = "As::<FromInto<OptionalFloat>>")]
    pub cpucore_util_7: Option<f64>,

    #[serde(rename = "CPUCoreUtil%[ 8]", with = "As::<FromInto<OptionalFloat>>")]
    pub cpucore_util_8: Option<f64>,

    #[serde(rename = "CPUCoreUtil%[ 9]", with = "As::<FromInto<OptionalFloat>>")]
    pub cpucore_util_9: Option<f64>,

    #[serde(rename = "CPUCoreUtil%[10]", with = "As::<FromInto<OptionalFloat>>")]
    pub cpucore_util_10: Option<f64>,

    #[serde(rename = "CPUCoreUtil%[11]", with = "As::<FromInto<OptionalFloat>>")]
    pub cpucore_util_11: Option<f64>,

    #[serde(rename = "CPUCoreUtil%[12]", with = "As::<FromInto<OptionalFloat>>")]
    pub cpucore_util_12: Option<f64>,

    #[serde(rename = "CPUCoreUtil%[13]", with = "As::<FromInto<OptionalFloat>>")]
    pub cpucore_util_13: Option<f64>,

    #[serde(rename = "CPUCoreUtil%[14]", with = "As::<FromInto<OptionalFloat>>")]
    pub cpucore_util_14: Option<f64>,

    #[serde(rename = "CPUCoreUtil%[15]", with = "As::<FromInto<OptionalFloat>>")]
    pub cpucore_util_15: Option<f64>,

    #[serde(rename = "CPUCoreUtil%[16]", with = "As::<FromInto<OptionalFloat>>")]
    pub cpucore_util_16: Option<f64>,

    #[serde(rename = "CPUCoreUtil%[17]", with = "As::<FromInto<OptionalFloat>>")]
    pub cpucore_util_17: Option<f64>,

    #[serde(rename = "CPUCoreUtil%[18]", with = "As::<FromInto<OptionalFloat>>")]
    pub cpucore_util_18: Option<f64>,

    #[serde(rename = "CPUCoreUtil%[19]", with = "As::<FromInto<OptionalFloat>>")]
    pub cpucore_util_19: Option<f64>,

    #[serde(rename = "CPUCoreUtil%[20]", with = "As::<FromInto<OptionalFloat>>")]
    pub cpucore_util_20: Option<f64>,

    #[serde(rename = "CPUCoreUtil%[21]", with = "As::<FromInto<OptionalFloat>>")]
    pub cpucore_util_21: Option<f64>,

    #[serde(rename = "CPUCoreUtil%[22]", with = "As::<FromInto<OptionalFloat>>")]
    pub cpucore_util_22: Option<f64>,

    #[serde(rename = "CPUCoreUtil%[23]", with = "As::<FromInto<OptionalFloat>>")]
    pub cpucore_util_23: Option<f64>,

    #[serde(rename = "CPUCoreUtil%[24]", with = "As::<FromInto<OptionalFloat>>")]
    pub cpucore_util_24: Option<f64>,

    #[serde(rename = "CPUCoreUtil%[25]", with = "As::<FromInto<OptionalFloat>>")]
    pub cpucore_util_25: Option<f64>,

    #[serde(rename = "CPUCoreUtil%[26]", with = "As::<FromInto<OptionalFloat>>")]
    pub cpucore_util_26: Option<f64>,

    #[serde(rename = "CPUCoreUtil%[27]", with = "As::<FromInto<OptionalFloat>>")]
    pub cpucore_util_27: Option<f64>,

    #[serde(rename = "CPUCoreUtil%[28]", with = "As::<FromInto<OptionalFloat>>")]
    pub cpucore_util_28: Option<f64>,

    #[serde(rename = "CPUCoreUtil%[29]", with = "As::<FromInto<OptionalFloat>>")]
    pub cpucore_util_29: Option<f64>,

    #[serde(rename = "CPUCoreUtil%[30]", with = "As::<FromInto<OptionalFloat>>")]
    pub cpucore_util_30: Option<f64>,

    #[serde(rename = "CPUCoreUtil%[31]", with = "As::<FromInto<OptionalFloat>>")]
    pub cpucore_util_31: Option<f64>,

    #[serde(rename = "CPUCoreUtil%[32]", with = "As::<FromInto<OptionalFloat>>")]
    pub cpucore_util_32: Option<f64>,

    #[serde(rename = "CPUCoreUtil%[33]", with = "As::<FromInto<OptionalFloat>>")]
    pub cpucore_util_33: Option<f64>,

    #[serde(rename = "CPUCoreUtil%[34]", with = "As::<FromInto<OptionalFloat>>")]
    pub cpucore_util_34: Option<f64>,

    #[serde(rename = "CPUCoreUtil%[35]", with = "As::<FromInto<OptionalFloat>>")]
    pub cpucore_util_35: Option<f64>,

    #[serde(rename = "CPUCoreUtil%[36]", with = "As::<FromInto<OptionalFloat>>")]
    pub cpucore_util_36: Option<f64>,

    #[serde(rename = "CPUCoreUtil%[37]", with = "As::<FromInto<OptionalFloat>>")]
    pub cpucore_util_37: Option<f64>,

    #[serde(rename = "CPUCoreUtil%[38]", with = "As::<FromInto<OptionalFloat>>")]
    pub cpucore_util_38: Option<f64>,

    #[serde(rename = "CPUCoreUtil%[39]", with = "As::<FromInto<OptionalFloat>>")]
    pub cpucore_util_39: Option<f64>,

    #[serde(rename = "CPUCoreUtil%[40]", with = "As::<FromInto<OptionalFloat>>")]
    pub cpucore_util_40: Option<f64>,

    #[serde(rename = "CPUCoreUtil%[41]", with = "As::<FromInto<OptionalFloat>>")]
    pub cpucore_util_41: Option<f64>,

    #[serde(rename = "CPUCoreUtil%[42]", with = "As::<FromInto<OptionalFloat>>")]
    pub cpucore_util_42: Option<f64>,

    #[serde(rename = "CPUCoreUtil%[43]", with = "As::<FromInto<OptionalFloat>>")]
    pub cpucore_util_43: Option<f64>,

    #[serde(rename = "CPUCoreUtil%[44]", with = "As::<FromInto<OptionalFloat>>")]
    pub cpucore_util_44: Option<f64>,

    #[serde(rename = "CPUCoreUtil%[45]", with = "As::<FromInto<OptionalFloat>>")]
    pub cpucore_util_45: Option<f64>,

    #[serde(rename = "CPUCoreUtil%[46]", with = "As::<FromInto<OptionalFloat>>")]
    pub cpucore_util_46: Option<f64>,

    #[serde(rename = "CPUCoreUtil%[47]", with = "As::<FromInto<OptionalFloat>>")]
    pub cpucore_util_47: Option<f64>,

    #[serde(rename = "CPUCoreUtil%[48]", with = "As::<FromInto<OptionalFloat>>")]
    pub cpucore_util_48: Option<f64>,

    #[serde(rename = "CPUCoreUtil%[49]", with = "As::<FromInto<OptionalFloat>>")]
    pub cpucore_util_49: Option<f64>,

    #[serde(rename = "CPUCoreUtil%[50]", with = "As::<FromInto<OptionalFloat>>")]
    pub cpucore_util_50: Option<f64>,

    #[serde(rename = "CPUCoreUtil%[51]", with = "As::<FromInto<OptionalFloat>>")]
    pub cpucore_util_51: Option<f64>,

    #[serde(rename = "CPUCoreUtil%[52]", with = "As::<FromInto<OptionalFloat>>")]
    pub cpucore_util_52: Option<f64>,

    #[serde(rename = "CPUCoreUtil%[53]", with = "As::<FromInto<OptionalFloat>>")]
    pub cpucore_util_53: Option<f64>,

    #[serde(rename = "CPUCoreUtil%[54]", with = "As::<FromInto<OptionalFloat>>")]
    pub cpucore_util_54: Option<f64>,

    #[serde(rename = "CPUCoreUtil%[55]", with = "As::<FromInto<OptionalFloat>>")]
    pub cpucore_util_55: Option<f64>,

    #[serde(rename = "CPUCoreUtil%[56]", with = "As::<FromInto<OptionalFloat>>")]
    pub cpucore_util_56: Option<f64>,

    #[serde(rename = "CPUCoreUtil%[57]", with = "As::<FromInto<OptionalFloat>>")]
    pub cpucore_util_57: Option<f64>,

    #[serde(rename = "CPUCoreUtil%[58]", with = "As::<FromInto<OptionalFloat>>")]
    pub cpucore_util_58: Option<f64>,

    #[serde(rename = "CPUCoreUtil%[59]", with = "As::<FromInto<OptionalFloat>>")]
    pub cpucore_util_59: Option<f64>,

    #[serde(rename = "CPUCoreUtil%[60]", with = "As::<FromInto<OptionalFloat>>")]
    pub cpucore_util_60: Option<f64>,

    #[serde(rename = "CPUCoreUtil%[61]", with = "As::<FromInto<OptionalFloat>>")]
    pub cpucore_util_61: Option<f64>,

    #[serde(rename = "CPUCoreUtil%[62]", with = "As::<FromInto<OptionalFloat>>")]
    pub cpucore_util_62: Option<f64>,

    #[serde(rename = "CPUCoreUtil%[63]", with = "As::<FromInto<OptionalFloat>>")]
    pub cpucore_util_63: Option<f64>,

    #[serde(
        rename = "Current Battery Capacity(Wh)",
        with = "As::<FromInto<OptionalFloat>>"
    )]
    pub current_battery_capacity_wh: Option<f64>,

    #[serde(
        rename = "Total Battery Capacity(Wh)",
        with = "As::<FromInto<OptionalFloat>>"
    )]
    pub total_battery_capacity_wh: Option<f64>,

    #[serde(rename = "Battery Percentage", with = "As::<FromInto<OptionalFloat>>")]
    pub battery_percentage: Option<f64>,

    #[serde(
        rename = "Battery Drain Rate(W)",
        with = "As::<FromInto<OptionalFloat>>"
    )]
    pub battery_drain_rate_w: Option<f64>,
}
