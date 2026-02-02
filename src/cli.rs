use clap::{Parser, Subcommand, ValueEnum};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(
    name = "prakrit-verb",
    version,
    author = "Vyom A. Shah",
    about = "Prakrit verb conjugation generator",
    long_about = "Generate verb conjugations for Maharastri, Shauraseni, and Magadhi Prakrit dialects.\n\nSupports Harvard-Kyoto (HK) and SLP1 encodings."
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Generate conjugation for a single verb
    Conjugate {
        /// Verb root in Harvard-Kyoto or SLP1 encoding
        verb: String,

        /// Tense to conjugate
        #[arg(short, long, value_enum, default_value = "present")]
        tense: TenseArg,

        /// Grammatical voice
        #[arg(long, value_enum, default_value = "active")]
        voice: VoiceArg,

        /// Prakrit dialect
        #[arg(short, long, value_enum, default_value = "maharastri")]
        dialect: DialectArg,

        /// Output format
        #[arg(short, long, value_enum, default_value = "table")]
        format: OutputFormat,

        /// Output encoding (hk or slp1)
        #[arg(short, long, value_enum, default_value = "slp1")]
        encoding: EncodingArg,

        /// Output file path (stdout if not specified)
        #[arg(short, long)]
        output: Option<PathBuf>,
    },

    /// Process a batch file of verb roots
    Batch {
        /// Input file containing verb roots (one per line)
        #[arg(short, long)]
        input: PathBuf,

        /// Output file path
        #[arg(short, long)]
        output: PathBuf,

        /// Output format
        #[arg(short, long, value_enum, default_value = "json")]
        format: BatchOutputFormat,

        /// Output encoding (hk or slp1)
        #[arg(long, value_enum, default_value = "slp1")]
        encoding: EncodingArg,

        /// Tenses to generate (can specify multiple: --tenses present --tenses past)
        #[arg(long = "tenses", value_enum)]
        tenses: Vec<TenseArg>,

        /// Voices to generate (can specify multiple: --voices active --voices passive)
        #[arg(long = "voices", value_enum)]
        voices: Vec<VoiceArg>,

        /// Dialects to generate (can specify multiple: --dialects maharastri --dialects magadhi)
        #[arg(long = "dialects", value_enum)]
        dialects: Vec<DialectArg>,

        /// Generate all tenses (present, past, future, imperative)
        #[arg(long, default_value = "false")]
        all_tenses: bool,

        /// Generate all dialects (maharastri, shauraseni, magadhi)
        #[arg(long, default_value = "false")]
        all_dialects: bool,

        /// Generate all voices (active, passive)
        #[arg(long, default_value = "false")]
        all_voices: bool,

        /// Generate all combinations (all tenses × all dialects × all voices)
        #[arg(long, default_value = "false")]
        all: bool,
    },

    /// Start interactive mode
    Interactive,
}

#[derive(ValueEnum, Clone, Debug, Copy)]
pub enum TenseArg {
    Present,
    Past,
    Future,
    Imperative,
}

impl From<TenseArg> for crate::models::Tense {
    fn from(arg: TenseArg) -> Self {
        match arg {
            TenseArg::Present | TenseArg::Imperative => crate::models::Tense::Present,
            TenseArg::Past => crate::models::Tense::Past,
            TenseArg::Future => crate::models::Tense::Future,
        }
    }
}

impl TenseArg {
    pub fn is_imperative(&self) -> bool {
        matches!(self, TenseArg::Imperative)
    }
}

#[derive(ValueEnum, Clone, Debug, Copy)]
pub enum VoiceArg {
    Active,
    Passive,
}

impl From<VoiceArg> for crate::models::Voice {
    fn from(arg: VoiceArg) -> Self {
        match arg {
            VoiceArg::Active => crate::models::Voice::Active,
            VoiceArg::Passive => crate::models::Voice::Passive,
        }
    }
}

#[derive(ValueEnum, Clone, Debug, Copy)]
pub enum DialectArg {
    Maharastri,
    Shauraseni,
    Magadhi,
}

impl From<DialectArg> for crate::models::Dialect {
    fn from(arg: DialectArg) -> Self {
        match arg {
            DialectArg::Maharastri => crate::models::Dialect::Maharastri,
            DialectArg::Shauraseni => crate::models::Dialect::Shauraseni,
            DialectArg::Magadhi => crate::models::Dialect::Magadhi,
        }
    }
}

#[derive(ValueEnum, Clone, Debug, Copy)]
pub enum EncodingArg {
    Hk,
    Slp1,
}

impl From<EncodingArg> for crate::models::Encoding {
    fn from(arg: EncodingArg) -> Self {
        match arg {
            EncodingArg::Hk => crate::models::Encoding::HK,
            EncodingArg::Slp1 => crate::models::Encoding::SLP1,
        }
    }
}

#[derive(ValueEnum, Clone, Debug, Copy)]
pub enum OutputFormat {
    /// Human-readable table
    Table,
    /// JSON output
    Json,
    /// CSV output
    Csv,
}

#[derive(ValueEnum, Clone, Debug, Copy)]
pub enum BatchOutputFormat {
    /// JSON output
    Json,
    /// CSV output
    Csv,
}
