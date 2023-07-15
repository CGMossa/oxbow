use extendr_api::prelude::*;
use oxbow::fasta::FastaReader;
use oxbow::fastq::FastqReader;
use oxbow::bam::BamReader;
use oxbow::cram::CramReader;
use oxbow::vcf::VcfReader;
use oxbow::bcf::BcfReader;

/// Return Arrow IPC format from a FASTA file.
/// @export
#[extendr]
fn read_fasta(path: &str, region: Option<&str>) -> Vec<u8> {
    let mut reader = FastaReader::new(path).unwrap();
    reader.records_to_ipc(region).unwrap()
}

/// Return Arrow IPC format from a FASTQ file.
/// @export
#[extendr]
fn read_fastq(path: &str) -> Vec<u8> {
    let mut reader = FastqReader::new(path).unwrap();
    reader.records_to_ipc().unwrap()
}

/// Return Arrow IPC format from a BAM file.
/// @export
#[extendr]
fn read_bam(path: &str, region: Option<&str>) -> Vec<u8> {
    let mut reader = BamReader::new(path).unwrap();
    reader.records_to_ipc(region).unwrap()
}

/// Return Arrow IPC format from a CRAM file.
/// @export
#[extendr]
fn read_cram(path: &str, fasta_path: &str, region: Option<&str>) -> Vec<u8> {
    let mut reader = CramReader::new(path, fasta_path).unwrap();
    reader.records_to_ipc(region).unwrap()
}

/// Return Arrow IPC format from a VCF file.
/// @export
#[extendr]
fn read_vcf(path: &str, region: Option<&str>) -> Vec<u8> {
    let mut reader = VcfReader::new(path).unwrap();
    reader.records_to_ipc(region).unwrap()
}

/// Return Arrow IPC format from a BCF file.
/// @export
#[extendr]
fn read_bcf(path: &str, region: Option<&str>) -> Vec<u8> {
    let mut reader = BcfReader::new(path).unwrap();
    reader.records_to_ipc(region).unwrap()
}

// Macro to generate exports.
// This ensures exported functions are registered with R.
// See corresponding C code in `entrypoint.c`.
extendr_module! {
    mod oxbow;
    fn read_fasta;
    fn read_fastq;
    fn read_bam;
    fn read_cram;
    fn read_vcf;
    fn read_bcf;
}
