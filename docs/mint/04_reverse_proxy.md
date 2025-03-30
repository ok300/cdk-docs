# Setup Reverse Proxy

The mint service created in the previous section listens only on the loopback interface, meaning it is not reachable from outside.

This section will go over how to expose the mint service to the internet, by using a reverse proxy. In this example we'll use `caddy`.

This assumes you are installing the mint on a machine that is directly reachable from the internet using a domain or subdomain.

---

Edit the default `Caddyfile` at `/etc/caddy/Caddyfile` as follows:

* un-comment the `reverse-proxy` line and change it to `reverse-proxy localhost:8085`
* comment out the `file-server` line

Then restart `caddy`:

```
sudo systemctl restart caddy
```

Your mint should now be publicly accessible.

Once you've tested the mint is publicly reachable over HTTP, you can switch to HTTPS by editing the `Caddyfile` and replacing `http://` in the first un-commented line with `your-mint-domain.com`. Re-start `caddy` to generate and apply an HTTPS certificate.
