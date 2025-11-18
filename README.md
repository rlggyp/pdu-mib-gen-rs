# PDU MIB Generator

This tool is designed to generate 32 MIB files, including both master and slave configurations, for PDUs (Power Distribution Units).

## Features

- Automatically generates 32 MIB files
- Includes both master and slave MIBs
- Simple Docker-based setup for easy use

## Prerequisites

- [Docker](https://docs.docker.com/engine/install/) installed on your system

## Getting Started

You have two options to run the generator container:

### Option 1: Build the Docker Image Manually

Build the Docker image locally with:

```bash
docker build -t pdu-mib-gen:latest .
```

Generate the MIB files using:

```bash
docker run --rm -v $(pwd):/opt pdu-mib-gen:latest <mib_file>
```

### Option 2: Use the Prebuilt Image

Pull the prebuilt image from Docker Hub:

```bash
docker pull rlggyp/pdu-mib-gen:latest
```

Generate the MIB files using:

```bash
docker run --rm -v $(pwd):/opt rlggyp/pdu-mib-gen:latest <mib_file>
```

Replace `<mib_file>` with the path to your input MIB file.

All generated files will be saved in the current working directory.

## Notes

- Ensure your input MIB file is formatted correctly.
- The output includes 32 MIB files, each representing a PDU (both master and slave variants).
