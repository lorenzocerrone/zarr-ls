# zarr-ls
Simple CLI to explore interactively a Zarr file

DISCAIMER! This is a toy project to learn Rust. 

## Build 
```bash
cargo build --release
```

## Usage
```bash
zarr-ls <zarr-file or directory> 
```

## Roadmap
- [x] Working prototype
- [ ] Handle different types of zarr storages (in particular S3)
- [ ] Add more options to the CLI (e.g. chose if using the interactive mode or not)
- [ ] Improve the output (e.g. add colors, layout, etc.)
- [ ] Improve the API 
- [ ] Add tests
- [ ] Add documentation
- [ ] Add CD 