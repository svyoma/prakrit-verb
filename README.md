# Prakrit Verb Conjugation CLI

A fast, cross-platform command-line tool for generating Prakrit verb conjugations across multiple dialects, tenses, and voices.

> Ported from Python to Rust for improved performance and ease of distribution.

## Features

- **4 Tenses/Moods**: Present, Past, Future, Imperative
- **3 Dialects**: Maharashtri, Shauraseni, Magadhi
- **2 Voices**: Active, Passive
- **2 Encodings**: Harvard-Kyoto (HK) and SLP1
- **Multiple Output Formats**: Table (human-readable), JSON, CSV
- **Batch Processing**: Process hundreds of verbs with flexible combination options
- **Interactive Mode**: REPL for quick conjugation lookups

## Installation

### Pre-built Binaries

Download the latest release for your platform from the [Releases](../../releases) page.

### Building from Source

#### Prerequisites

- Rust 1.70+ ([install from rustup.rs](https://rustup.rs))
- On Windows: Visual Studio Build Tools with "C++ build tools" workload

#### Build Commands

```bash
# Clone the repository
git clone https://github.com/svyoma/prakrit-verb-cli.git
cd prakrit-verb-cli

# Build release version (recommended)
cargo build --release

# The binary will be at: target/release/prakrit-verb-cli
```

## Quick Start

```bash
# Conjugate a verb (defaults: present tense, Maharashtri dialect, active voice)
prakrit-verb-cli conjugate gam

# Specify tense and dialect
prakrit-verb-cli conjugate gam --tense future --dialect magadhi

# Output as JSON
prakrit-verb-cli conjugate gam --format json

# Start interactive mode
prakrit-verb-cli interactive
```

## Usage

### Single Verb Conjugation

```bash
# Basic usage
prakrit-verb-cli conjugate <VERB>

# With options
prakrit-verb-cli conjugate gam --tense present --dialect shauraseni --voice passive

# Save output to file
prakrit-verb-cli conjugate gam --format json --output result.json
```

#### Options for `conjugate`

| Option | Short | Values | Default | Description |
|--------|-------|--------|---------|-------------|
| `--tense` | `-t` | `present`, `past`, `future`, `imperative` | `present` | Tense/mood to conjugate |
| `--dialect` | `-d` | `maharastri`, `shauraseni`, `magadhi` | `maharastri` | Prakrit dialect |
| `--voice` | | `active`, `passive` | `active` | Grammatical voice |
| `--format` | `-f` | `table`, `json`, `csv` | `table` | Output format |
| `--encoding` | `-e` | `hk`, `slp1` | `slp1` | Output encoding |
| `--output` | `-o` | `PATH` | stdout | Output file path |

### Batch Processing

Process multiple verbs from a text file with flexible combination options.

#### Input File Format

Create a text file with one verb root per line:

```text
# verbs.txt - Comments start with #
gam
bhU
kar
nI
pucch
```

#### Basic Batch Commands

```bash
# Process with defaults (present, maharastri, active)
prakrit-verb-cli batch -i verbs.txt -o results.json

# Output as CSV
prakrit-verb-cli batch -i verbs.txt -o results.csv --format csv
```

#### Selecting Specific Combinations

```bash
# Multiple tenses
prakrit-verb-cli batch -i verbs.txt -o out.json \
  --tenses present --tenses future

# Multiple dialects
prakrit-verb-cli batch -i verbs.txt -o out.json \
  --dialects maharastri --dialects magadhi

# Multiple voices
prakrit-verb-cli batch -i verbs.txt -o out.json \
  --voices active --voices passive

# Combined: 2 tenses x 2 dialects x 2 voices = 8 conjugations per verb
prakrit-verb-cli batch -i verbs.txt -o out.json \
  --tenses present --tenses past \
  --dialects maharastri --dialects shauraseni \
  --voices active --voices passive
```

#### "All" Flags

```bash
# All tenses (present, past, future, imperative)
prakrit-verb-cli batch -i verbs.txt -o out.json --all-tenses

# All dialects (maharastri, shauraseni, magadhi)
prakrit-verb-cli batch -i verbs.txt -o out.json --all-dialects

# All voices (active, passive)
prakrit-verb-cli batch -i verbs.txt -o out.json --all-voices

# Everything: 4 tenses x 3 dialects x 2 voices = 24 conjugations per verb
prakrit-verb-cli batch -i verbs.txt -o out.json --all
```

#### Options for `batch`

| Option | Short | Values | Default | Description |
|--------|-------|--------|---------|-------------|
| `--input` | `-i` | `PATH` | required | Input file with verb roots |
| `--output` | `-o` | `PATH` | required | Output file path |
| `--format` | `-f` | `json`, `csv` | `json` | Output format |
| `--encoding` | | `hk`, `slp1` | `slp1` | Output encoding |
| `--tenses` | | `present`, `past`, `future`, `imperative` | `present` | Tenses to generate (repeatable) |
| `--dialects` | | `maharastri`, `shauraseni`, `magadhi` | `maharastri` | Dialects to generate (repeatable) |
| `--voices` | | `active`, `passive` | `active` | Voices to generate (repeatable) |
| `--all-tenses` | | | `false` | Generate all 4 tenses |
| `--all-dialects` | | | `false` | Generate all 3 dialects |
| `--all-voices` | | | `false` | Generate both voices |
| `--all` | | | `false` | Generate all combinations |

### Interactive Mode

```bash
prakrit-verb-cli interactive
```

In interactive mode, enter verbs with optional parameters:

```
> gam
> gam present maharastri active
> bhU future magadhi passive
> help
> quit
```

## Output Formats

### Table (Default)

Human-readable format for terminal display:

```
Verb Root: gam
Tense: present
Mood: indicative
Voice: active
Dialect: maharastri

Person               Singular                                 Plural
----------------------------------------------------------------------------------------------------
Third Person         gama_i, gamae, game_i                    gamanti, gamante, gama_ire...
Second Person        gamasi, gamase, gamesi                   gamaha, gama_itthA...
First Person         gamami, gamAmi, gamemi                   gamamo, gamAmo, gamemo...
```

### JSON

```json
{
  "verb_root": "gam",
  "tense": "present",
  "mood": "indicative",
  "voice": "active",
  "dialect": "maharastri",
  "forms": {
    "third_singular": ["gama_i", "gamae", "game_i"],
    "third_plural": ["gamanti", "gamante", "gama_ire"],
    "second_singular": ["gamasi", "gamase", "gamesi"],
    "second_plural": ["gamaha", "gama_itthA"],
    "first_singular": ["gamami", "gamAmi", "gamemi"],
    "first_plural": ["gamamo", "gamAmo", "gamemo", "gamimo"]
  }
}
```

### CSV

```csv
verb_root,tense,mood,voice,dialect,person,number,forms
gam,present,indicative,active,maharastri,third,singular,"gama_i, gamae, game_i"
gam,present,indicative,active,maharastri,third,plural,"gamanti, gamante, gama_ire"
```

## Encoding Notes

The tool supports two romanization schemes:

- **SLP1** (default): A lossless ASCII encoding for Sanskrit/Prakrit
- **Harvard-Kyoto (HK)**: A widely-used ASCII encoding

For the characters used in Prakrit verb conjugation, both encodings are nearly identical. The main conventions:

| Character | SLP1 | HK |
|-----------|------|-----|
| Long A | A | A |
| Long I | I | I |
| Long U | U | U |
| Anusvara | M | M |

## Project Structure

```
prakrit-verb-cli/
├── Cargo.toml           # Rust package manifest
├── README.md
├── examples/
│   └── verbs.txt        # Sample verb list
└── src/
    ├── main.rs          # Entry point, CLI orchestration
    ├── lib.rs           # Library exports
    ├── cli.rs           # Clap argument definitions
    ├── models.rs        # Data structures
    ├── error.rs         # Error types
    ├── encoding.rs      # HK/SLP1 conversion
    ├── conjugation/
    │   ├── mod.rs       # Conjugation dispatcher
    │   ├── present.rs   # Present tense logic
    │   ├── past.rs      # Past tense logic
    │   ├── future.rs    # Future tense logic
    │   └── affixes.rs   # Dialect-specific affix tables
    └── io/
        ├── mod.rs       # I/O module
        ├── batch.rs     # Batch processing
        ├── csv_writer.rs
        └── json_writer.rs
```

## Contributing

Contributions are welcome! Please feel free to submit issues and pull requests.

## License

MIT License - see [LICENSE](LICENSE) file for details.

## Credits

- Original Python implementation by **Vyom A. Shah** ([@svyoma](https://github.com/svyoma))
- Rust port preserves all conjugation logic from the original

## References

- Pischel, R. (1900). *Grammatik der Prakrit-Sprachen*
- Woolner, A.C. (1928). *Introduction to Prakrit*
