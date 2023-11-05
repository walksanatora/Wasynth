use std::io::{Result, Write};

use wasm_ast::module::Module;

fn load_arg_source() -> Result<Vec<u8>> {
	let name = std::env::args().nth(1).expect("usage: wasm2luajit <file>");

	std::fs::read(name)
}

fn do_runtime(lock: &mut dyn Write) -> Result<()> {
	let runtime = codegen_lua53::RUNTIME;
	let numeric = codegen_lua53::NUMERIC;

	writeln!(lock, "local Integer = (function()")?;
	writeln!(lock, "{numeric}")?;
	writeln!(lock, "end)()")?;
	writeln!(lock, "local rt = (function()")?;
	writeln!(lock, "{runtime}")?;
	writeln!(lock, "end)()")
}
fn main() -> Result<()> {
	let data = load_arg_source()?;
	let wasm = Module::try_from_data(&data).unwrap();

	let lock = &mut std::io::stdout().lock();

	do_runtime(lock)?;
	codegen_lua53::from_module_untyped(&wasm, lock)
}
