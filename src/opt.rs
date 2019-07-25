use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct Opt {
    pub pattern: String,
}

impl Opt {
    pub fn from_args() -> Self {
        StructOpt::from_args()
    }
}
