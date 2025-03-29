# Run as container with LND

This guide assumes a headless Fedora Server 41 setup.

These instructions were last tested with `cdk-mintd v0.8.1`.

Prerequisites

* Make sure LND is installed and reachable

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

Note: After it's created, the container can be started and stopped with:

```
podman stop cdk-mintd
podman start cdk-mintd
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

The next step is to create a `systemd` service:

```
cd ~

# Create a service file
podman generate systemd --name cdk-mintd --files --new

# Copy the service file to the systemd directory
sudo cp container-cdk-mintd.service /etc/systemd/system/

# Enable and start the service
sudo systemctl daemon-reload
sudo systemctl enable container-cdk-mintd.service
sudo systemctl start container-cdk-mintd.service
```

With the new `container-cdk-mintd` service running, check that the mint is reachable from your host:

```
curl http://localhost:8085/v1/info
# JSON output
```

The next and final step is to expose the mint service through a reverse proxy. In this example we'll use `caddy`.

Edit the default `Caddyfile` at `/etc/caddy/Caddyfile` as follows:

* un-comment the `reverse-proxy` line and change it to `reverse-proxy localhost:8085`
* comment out the `file-server` line

Then restart `caddy`:

```
sudo systemctl restart caddy
```

Your mint should now be publicly accessible.

Once you've tested the mint is publicly reachable over HTTP, you can switch to HTTPS by editing the `Caddyfile` and replacing `http://` in the first un-commented line with `your-mint-domain.com`. Re-start `caddy` to generate and apply an HTTPS certificate.
