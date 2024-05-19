# Orion
Orion aims to be a fast, powerful and scalable language.

## Try with docker
If you have [Docker] installed, you can try out Orion with:

```bash
docker pull arnabrollin/orion && docker run -it arnabrollin/orion
```
Then in the docker shell:
```bash
orion --help # sanity check
```

## Installation
For now, you can only build Orion from source or download the pre-built executable.

### Building from source
1. To build from source, you need to have [Rust] installed.
2. Clone this repository.
3. Run:
```bash
cargo run --release -- --help # sanity check
```
in the repository directory.

### Downloading the pre-built executable.
1. Go to the [Releases] page.
2. Find the executable named after your target-triplet (which can be obtained by running `gcc -dumpmachine`).
3. Click on the executable to download it.
4. Locate the executable on your machine.
5. 
```bash
/path/to/executable --help # sanity check
```

[Docker]:https://www.docker.com
[Rust]:https://www.rust-lang.org
[Releases]:https://github.com/Solarcode-org/orion/releases/latest