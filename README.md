# Time Series Data Aggregator

A multi-threaded tool for combining and processing time series data files.

## Quick Start

```bash
# installation (assuming rust and cargo is installed)
cargo install ts-merger
# Basic usage with default .csv extension
ts-merger /path/to/data

# Custom file extension
ts-merger /path/to/data .txt
```

## Example

### Input Files

```
data/
├── temperature_1.csv
├── temperature_2.csv
└── pressure_1.csv

# temperature_1.csv
id, timestamp, value
A1, 2024-01-01 00:00:00, 20.5
A2, 2024-01-01 00:01:00, 21.0
A3, 2024-01-01 00:02:00, 20.8

# temperature_2.csv
id, timestamp, value
A1, 2024-01-01 00:00:00, 19.5
A2, 2024-01-01 00:01:00, 20.0
A3, 2024-01-01 00:02:00, 19.8

# pressure_1.csv
id, timestamp, value
B1, 2024-01-01 00:00:00, 1013.2
B2, 2024-01-01 00:01:00, 1013.4
B3, 2024-01-01 00:02:00, 1013.1
```

### Output Files

```
data/
├── temperature.csv
└── pressure.csv

# temperature.csv (sum of _1 and _2 files)
id, timestamp, value
A1, 2024-01-01 00:00:00, 40.0
A2, 2024-01-01 00:01:00, 41.0
A3, 2024-01-01 00:02:00, 40.6

# pressure.csv (renamed from pressure_1.csv)
id, timestamp, value
B1, 2024-01-01 00:00:00, 1013.2
B2, 2024-01-01 00:01:00, 1013.4
B3, 2024-01-01 00:02:00, 1013.1
```

## Features

- Multi-threaded processing
- Customizable output file extension
- Progress indicator
- Automatic cleanup of processed files

## License

AGPL-3.0
