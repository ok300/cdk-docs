# Run as container with LND

!!! note "Environment"
    This guide assumes a headless Fedora Server 41 machine. The exact commands may be different for your setup.

These instructions were last tested with `cdk-mintd v0.8.1`.

Prerequisites

* LND is reachable from the machine where you are installing the mint

---

Create a working directory that will be used by `cdk-mintd`:

```
mkdir ~/.cdk-mintd
cd ~/.cdk-mintd
```

Copy the following two files from your LND node into this working directory:

* `tls.cert`: typically found under `<lnd-data-dir>/tls.cert`
* `admin.macaroon`: typically found under `<lnd-data-dir>/data/chain/bitcoin/<network>/admin.macaroon`

Download the sample CDK mint configuration file and save it as `config.toml`:

```
curl -o config.toml https://raw.githubusercontent.com/cashubtc/cdk/refs/tags/v0.8.1/crates/cdk-mintd/example.config.toml
```

Note the `v0.8.1` release version in the URL. Change it according to the version you're using.

Edit the downloaded `config.toml` as follows:

* change `info.url` to the URL under which your mint will be accessible
* change `info.listen_host` to `0.0.0.0`
* change `info.mnemonic` to a new mnemonic
* edit the fields in `mint_info` with public details of your mint (name, description, etc)
* change `ln.ln_backend` to `lnd`
* un-comment the entire `lnd` section
* change `lnd.address` to `https://ip:port` with the IP and port where your LND node exposes its gRPC interface
* change `lnd.macaroon_file` to `/root/.cdk-mintd/admin.macaroon`
* change `lnd.cert_file` to `/root/.cdk-mintd/tls.cert`

Your mint working directory should now contain:

```
ls ~/.cdk-mintd
config.toml  admin.macaroon  tls.cert
```

Create and run a `cdk-mintd` container that also:

* mounts your local mint working directory, so it's available from within the container
* binds port `8085` (the `info.listen_port` defined in `config.toml`), so that it's reachable from outside the container, but only on your host's loopback interface

```
podman run -d --name cdk-mintd \
    -p 127.0.0.1:8085:8085 \
    -v ~/.cdk-mintd:/root/.cdk-mintd:Z \
    docker.io/thesimplekid/cdk-mintd:latest
```

With the container running, check that the mint service is reachable from your host:

```
curl http://localhost:8085/v1/info
# JSON output
```

After you confirmed it's working, stop the container:

```
podman stop cdk-mintd
```

The next step is to [create a system service](03_systemd_service.md).