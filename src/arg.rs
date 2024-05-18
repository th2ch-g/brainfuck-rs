use clap::Parser;

#[derive(Parser, Debug, Clone)]
#[clap(version, about, arg_required_else_help = true)]
pub struct MainArg {
    /// Path to brainfuck file (.bf)
    pub bffile: String,
}

pub fn arg() -> MainArg {
    MainArg::parse()
}
