# Create `systemd` Service

This section assumes you already have a container called `cdk-mintd`.

It will show how to create a system service from the existing container. This will allow the container to be automatically started when the host starts up.

---

To create a system service:

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

The next and final step is to [expose the mint service through a reverse proxy](04_reverse_proxy.md).