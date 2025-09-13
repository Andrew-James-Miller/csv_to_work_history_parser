# CSV to Work History Parser

A command-line tool that converts work history from CSV format into a formatted text output.

## Usage

```bash
csv_to_work_history_parser <input.csv> [output.txt]
```

If output path is not provided, the file will be created in the current directory with the name "formatted_work_history.txt"

### Input CSV Format

The expected CSV format is:
```text
Company,Job Title,Start Date,End Date,Address,Supervisor Name,Description,Reason
"Company Name",Position,MM/DD/YYYY,MM/DD/YYYY,"Address",Supervisor,"Description",Reason
```

### Output Format

The program generates a text file with entries formatted as:
```text
Work History N
Company: Company Name
Position: Job Title
Start Date: MM/YYYY
End Date: MM/YYYY
Location: City, State
Responsibilities: Description
```

### Examples

With specific output path:
```bash
csv_to_work_history_parser work_history.csv my_output.txt
```

Output to current directory:
```bash
csv_to_work_history_parser work_history.csv
```

## Building

Make sure you have Rust installed, then:

```bash
cargo build --release
```

The executable will be available in `target/release/csv_to_work_history_parser`

This software is licensed under the MIT License, which permits use, modification, and distribution, subject to the terms detailed in the LICENSE file.
See https://opensource.org/licenses/MIT for the full text of the MIT License.

(c) Copyright 2025 Andrew J. Miller. All rights reserved.