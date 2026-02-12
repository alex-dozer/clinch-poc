use common::data_objs::Artifact;

pub mod lstran_mint;
pub mod lstran_ops;
pub mod ops_file;

fn main() {
    let artifact = Artifact {
        bytes: vec![0x25, 0x50, 0x44, 0x46],
        text: Some(String::from("Hello, world!")),
        meta: std::collections::HashMap::new(),
    };
    let ctx = lstran_mint::run_lstran_pipeline(&artifact);

    println!("Pipeline context: {:?}", ctx);
}
