use deno_core::error::AnyError;
use deno_core::op_sync;
use deno_core::Extension;
use deno_core::OpState;
use deno_core::ZeroCopyBuf;
use serde::Deserialize;

#[no_mangle]
pub fn init() -> Extension {
  let mut builder = Extension::builder();

  builder.ops(vec![
      ("deno_example_plugin", op_sync(deno_example_plugin)),
  ]);
  
  builder.build()
}

#[derive(Debug, Deserialize)]
struct Args {
  val: String,
}

// sync plugin op.
fn deno_example_plugin(_state: &mut OpState, args: Args, _zero_copy: Option<ZeroCopyBuf>,) -> Result<String, AnyError> {
  
  println!("Hello from deno_example_plugin.");
  println!("i received an argument: {}", args.val);
  
  let return_val = String::from("denoExamplePluginReturnValue");
  println!("im returning the return value {}, to you!", return_val);
  
  Ok(return_val)
}

