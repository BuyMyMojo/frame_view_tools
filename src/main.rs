use clap::{Parser, Subcommand};
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use std::fs::File;

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

    let file = File::open(&args.in_file.as_path()).expect("msg");

    let mut reader = csv::Reader::from_reader(file);

    for line in reader.deserialize() {
        // Notice that we need to provide a type hint for automatic
        // deserialization.
        let record: FrameViewCSVEntry = match line {
            Ok(l) => l,
            Err(e) => panic!("{}", e),
        };
        // println!("{:?}", record);

        entry_vec.push(record);
    }

    let mut vec_of_frame_times = Vec::new();

    for entry in entry_vec {
        vec_of_frame_times.push(entry.ms_between_presents);
    }

    let vec_of_fps: Vec<f64> = vec_of_frame_times.par_iter().map(|x| 1000f64 / x).collect();

    let average: f64 = vec_of_fps.par_iter().sum::<f64>() / vec_of_fps.len() as f64;

    println!("{:?}", average)
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
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
    #[serde(rename = "AllowsTearing")]
    pub allows_tearing: i64,
    #[serde(rename = "ProcessID")]
    pub process_id: i64,
    #[serde(rename = "SwapChainAddress")]
    pub swap_chain_address: String,
    #[serde(rename = "SyncInterval")]
    pub sync_interval: i64,
    #[serde(rename = "PresentFlags")]
    pub present_flags: i64,
    #[serde(rename = "PresentMode")]
    pub present_mode: String,
    #[serde(rename = "Dropped")]
    pub dropped: String,
    #[serde(rename = "TimeInSeconds")]
    pub time_in_seconds: f64,
    #[serde(rename = "MsBetweenPresents")]
    pub ms_between_presents: f64,
    #[serde(rename = "MsBetweenDisplayChange")]
    pub ms_between_display_change: f64,
    #[serde(rename = "MsInPresentAPI")]
    pub ms_in_present_api: f64,
    #[serde(rename = "MsRenderPresentLatency")]
    pub ms_render_present_latency: f64,
    #[serde(rename = "MsUntilDisplayed")]
    pub ms_until_displayed: f64,
    #[serde(rename = "Render Queue Depth")]
    pub render_queue_depth: f64,
    #[serde(rename = "GPU0Clk(MHz)")]
    pub gpu0clk_mhz: u64,
    #[serde(rename = "GPU0MemClk(MHz)")]
    pub gpu0mem_clk_mhz: u64,
    #[serde(rename = "GPU0Util(%)")]
    pub gpu0util: u64,
    #[serde(rename = "GPU0Temp(C)")]
    pub gpu0temp_c: f64,
    #[serde(rename = "GPU1Clk(MHz)")]
    pub gpu1clk_mhz: String,
    #[serde(rename = "GPU1MemClk(MHz)")]
    pub gpu1mem_clk_mhz: String,
    #[serde(rename = "GPU1Util(%)")]
    pub gpu1util: String,
    #[serde(rename = "GPU1Temp(C)")]
    pub gpu1temp_c: String,
    #[serde(rename = "PCAT Power Total(W)")]
    pub pcat_power_total_w: String,
    #[serde(rename = "Perf/W Total(F/J) (PCAT)")]
    pub perf_w_total_f_j_pcat: String,
    #[serde(rename = "Perf/W Total(F/J) (API)")]
    pub perf_w_total_f_j_apit: String,
    #[serde(rename = "Perf/W GPUOnly(F/J) (API)")]
    pub perf_w_gpu_only_f_j_api: String,
    #[serde(rename = "Perf/W Total-USBC(F/J) (API)")]
    pub perf_w_total_usbc_f_j_api: String,
    #[serde(rename = "GPUOnlyPwr(W) (API)")]
    pub gpuonly_pwr_w_api: u64,
    #[serde(rename = "NV-Total-USBCPwr(W) (API)")]
    pub nv_total_usbcpwr_w_api: u64,
    #[serde(rename = "NV Pwr(W) (API)")]
    pub nv_pwr_w_api: u64,
    #[serde(rename = "AMDPwr(W) (API)")]
    pub amdpwr_w_api: String,
    #[serde(rename = "CPUClk(MHz)")]
    pub cpuclk_mhz: u64,
    #[serde(rename = "CPUUtil(%)")]
    pub cpuutil: u64,
    #[serde(rename = "CPU Package Temp(C)")]
    pub cpu_package_temp_c: String,
    #[serde(rename = "CPU Package Power(W)")]
    pub cpu_package_power_w: f64,
    #[serde(rename = "CPU TDP (W)")]
    pub cpu_tdp_w: String,
    #[serde(rename = "CPUCoreUtil%[ 0]")]
    pub cpucore_util_0: i64,
    #[serde(rename = "CPUCoreUtil%[ 1]")]
    pub cpucore_util_1: i64,
    #[serde(rename = "CPUCoreUtil%[ 2]")]
    pub cpucore_util_2: i64,
    #[serde(rename = "CPUCoreUtil%[ 3]")]
    pub cpucore_util_3: i64,
    #[serde(rename = "CPUCoreUtil%[ 4]")]
    pub cpucore_util_4: i64,
    #[serde(rename = "CPUCoreUtil%[ 5]")]
    pub cpucore_util_5: i64,
    #[serde(rename = "CPUCoreUtil%[ 6]")]
    pub cpucore_util_6: i64,
    #[serde(rename = "CPUCoreUtil%[ 7]")]
    pub cpucore_util_7: i64,
    #[serde(rename = "CPUCoreUtil%[ 8]")]
    pub cpucore_util_8: i64,
    #[serde(rename = "CPUCoreUtil%[ 9]")]
    pub cpucore_util_9: i64,
    #[serde(rename = "CPUCoreUtil%[10]")]
    pub cpucore_util_10: i64,
    #[serde(rename = "CPUCoreUtil%[11]")]
    pub cpucore_util_11: i64,
    #[serde(rename = "CPUCoreUtil%[12]")]
    pub cpucore_util_12: String,
    #[serde(rename = "CPUCoreUtil%[13]")]
    pub cpucore_util_13: String,
    #[serde(rename = "CPUCoreUtil%[14]")]
    pub cpucore_util_14: String,
    #[serde(rename = "CPUCoreUtil%[15]")]
    pub cpucore_util_15: String,
    #[serde(rename = "CPUCoreUtil%[16]")]
    pub cpucore_util_16: String,
    #[serde(rename = "CPUCoreUtil%[17]")]
    pub cpucore_util_17: String,
    #[serde(rename = "CPUCoreUtil%[18]")]
    pub cpucore_util_18: String,
    #[serde(rename = "CPUCoreUtil%[19]")]
    pub cpucore_util_19: String,
    #[serde(rename = "CPUCoreUtil%[20]")]
    pub cpucore_util_20: String,
    #[serde(rename = "CPUCoreUtil%[21]")]
    pub cpucore_util_21: String,
    #[serde(rename = "CPUCoreUtil%[22]")]
    pub cpucore_util_22: String,
    #[serde(rename = "CPUCoreUtil%[23]")]
    pub cpucore_util_23: String,
    #[serde(rename = "CPUCoreUtil%[24]")]
    pub cpucore_util_24: String,
    #[serde(rename = "CPUCoreUtil%[25]")]
    pub cpucore_util_25: String,
    #[serde(rename = "CPUCoreUtil%[26]")]
    pub cpucore_util_26: String,
    #[serde(rename = "CPUCoreUtil%[27]")]
    pub cpucore_util_27: String,
    #[serde(rename = "CPUCoreUtil%[28]")]
    pub cpucore_util_28: String,
    #[serde(rename = "CPUCoreUtil%[29]")]
    pub cpucore_util_29: String,
    #[serde(rename = "CPUCoreUtil%[30]")]
    pub cpucore_util_30: String,
    #[serde(rename = "CPUCoreUtil%[31]")]
    pub cpucore_util_31: String,
    #[serde(rename = "CPUCoreUtil%[32]")]
    pub cpucore_util_32: String,
    #[serde(rename = "CPUCoreUtil%[33]")]
    pub cpucore_util_33: String,
    #[serde(rename = "CPUCoreUtil%[34]")]
    pub cpucore_util_34: String,
    #[serde(rename = "CPUCoreUtil%[35]")]
    pub cpucore_util_35: String,
    #[serde(rename = "CPUCoreUtil%[36]")]
    pub cpucore_util_36: String,
    #[serde(rename = "CPUCoreUtil%[37]")]
    pub cpucore_util_37: String,
    #[serde(rename = "CPUCoreUtil%[38]")]
    pub cpucore_util_38: String,
    #[serde(rename = "CPUCoreUtil%[39]")]
    pub cpucore_util_39: String,
    #[serde(rename = "CPUCoreUtil%[40]")]
    pub cpucore_util_40: String,
    #[serde(rename = "CPUCoreUtil%[41]")]
    pub cpucore_util_41: String,
    #[serde(rename = "CPUCoreUtil%[42]")]
    pub cpucore_util_42: String,
    #[serde(rename = "CPUCoreUtil%[43]")]
    pub cpucore_util_43: String,
    #[serde(rename = "CPUCoreUtil%[44]")]
    pub cpucore_util_44: String,
    #[serde(rename = "CPUCoreUtil%[45]")]
    pub cpucore_util_45: String,
    #[serde(rename = "CPUCoreUtil%[46]")]
    pub cpucore_util_46: String,
    #[serde(rename = "CPUCoreUtil%[47]")]
    pub cpucore_util_47: String,
    #[serde(rename = "CPUCoreUtil%[48]")]
    pub cpucore_util_48: String,
    #[serde(rename = "CPUCoreUtil%[49]")]
    pub cpucore_util_49: String,
    #[serde(rename = "CPUCoreUtil%[50]")]
    pub cpucore_util_50: String,
    #[serde(rename = "CPUCoreUtil%[51]")]
    pub cpucore_util_51: String,
    #[serde(rename = "CPUCoreUtil%[52]")]
    pub cpucore_util_52: String,
    #[serde(rename = "CPUCoreUtil%[53]")]
    pub cpucore_util_53: String,
    #[serde(rename = "CPUCoreUtil%[54]")]
    pub cpucore_util_54: String,
    #[serde(rename = "CPUCoreUtil%[55]")]
    pub cpucore_util_55: String,
    #[serde(rename = "CPUCoreUtil%[56]")]
    pub cpucore_util_56: String,
    #[serde(rename = "CPUCoreUtil%[57]")]
    pub cpucore_util_57: String,
    #[serde(rename = "CPUCoreUtil%[58]")]
    pub cpucore_util_58: String,
    #[serde(rename = "CPUCoreUtil%[59]")]
    pub cpucore_util_59: String,
    #[serde(rename = "CPUCoreUtil%[60]")]
    pub cpucore_util_60: String,
    #[serde(rename = "CPUCoreUtil%[61]")]
    pub cpucore_util_61: String,
    #[serde(rename = "CPUCoreUtil%[62]")]
    pub cpucore_util_62: String,
    #[serde(rename = "CPUCoreUtil%[63]")]
    pub cpucore_util_63: String,
    #[serde(rename = "Current Battery Capacity(Wh)")]
    pub current_battery_capacity_wh: String,
    #[serde(rename = "Total Battery Capacity(Wh)")]
    pub total_battery_capacity_wh: String,
    #[serde(rename = "Battery Percentage")]
    pub battery_percentage: String,
    #[serde(rename = "Battery Drain Rate(W)")]
    pub battery_drain_rate_w: String,
}
