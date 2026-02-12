/*

Just a file of ops to illustrate we can re-export into the component ops file.

Add ops if you want. I initially required luop and lurv (lucius operation and lucius return value)
for validation.

Operations need references to Artifacts as an argument. Return values should be able to be whatever.

Ideally don't do things like spin up threads in this. It also needs to be sync.

*/

use common::data_objs::{Artifact, LuciusContext};

pub struct InspectMagicResult {
    pub matched: bool,
    pub magic: [u8; 4],
}

pub fn inspect_magic(artifact: &Artifact) -> InspectMagicResult {
    let bytes = &artifact.bytes;

    let magic = bytes.get(0..4).unwrap_or(&[0, 0, 0, 0]);
    let matched = matches!(
        magic,
        [0x25, 0x50, 0x44, 0x46] | // %PDF
        [0x4D, 0x5A, _, _] // MZ
    );

    InspectMagicResult {
        matched,
        magic: [magic[0], magic[1], magic[2], magic[3]],
    }
}

pub struct EntropyResult {
    pub entropy: f64,
}

pub fn entropy_probe(artifact: &Artifact) -> EntropyResult {
    let entropy = compute_entropy(&artifact.bytes);
    EntropyResult { entropy }
}

fn compute_entropy(_bytes: &[u8]) -> f64 {
    // Placeholder for actual entropy calculation
    0.0
}

pub struct FormatClassResult {
    pub format: &'static str,
}

pub fn classify_format(artifact: &Artifact) -> FormatClassResult {
    let format = match artifact.bytes.get(0..2) {
        Some([0x4D, 0x5A]) => "pe",
        Some([0x25, 0x50]) => "pdf",
        _ => "unknown",
    };

    FormatClassResult { format }
}
