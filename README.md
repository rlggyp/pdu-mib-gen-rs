# PDU MIB Generator

This tool is designed to generate 32 MIB files, including both master and slave configurations, for PDUs (Power Distribution Units).

## Features

- Automatically generates 32 MIB files
- Includes both master and slave MIBs
- Simple Docker-based setup for easy use

## Prerequisites

- [Docker](https://docs.docker.com/engine/install/) installed on your system

## Getting Started

### 1. Build the Docker Image

To build the Docker image for the generator, run the following command:

```bash
docker build -t generator:pdu .
```

### 2. Generate MIB Files

Once the image is built, you can generate the MIB files using:

```bash
docker run --rm -v $(pwd):/opt generator:pdu <mib_file>
```

Replace `<mib_file>` with the path to your input MIB file.

All generated files will be saved in the current working directory.

## Notes

- Ensure your input MIB file is formatted correctly.
- The output includes 32 MIB files, each representing a PDU (both master and slave variants).
