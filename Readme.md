
Install *nix:

```
sudo wget -c https://github.com/nstreamio/swim-create/releases/latest/download/swim-create-x86_64-unknown-linux-gnu.tar.gz | sudo tar -xz -C /usr/local/bin
```
or download manually and run:
```
sudo tar -f swim-create-x86_64-unknown-linux-gnu.tar.gz -xz -C /usr/local/bin
```

Releasing a new version:
```
git tag VERSION (e.g. v0.1.0)
git push origin VERSION
```