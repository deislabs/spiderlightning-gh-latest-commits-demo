# Slight demo application: Latest commits in GitHub repos

This sample application displays the five most recent commits from a set of GitHub repositories. The application gets the set of repositories from a table in a Postgresql database.

The application is written using [Slight]() and uses the [SQL and HTTP Client capabilities released in version 0.3](https://deislabs.io/posts/annoucing-slight-v0_3/).

[PicoCSS](https://picocss.com/) is used for styling the HTML.

> *IMPORTANT* This application is a demo app and is not intended to be run in a production environment. It is intended for demonstrating the features and capabilities of Slight only. Use at your own risk.

## Run the application on your development computer

You can run the Slight demo application directly on your development computer.

Before you begin, you'll need:

- [Slight installed](https://github.com/deislabs/spiderlightning#installation)
- [Access to a postgresql database](https://www.postgresql.org/)

Clone the sample repo and navigate to the root directory:

```console
git clone https://github.com/deislabs/spiderlightning-gh-latest-commits-demo.git
cd spiderlightning-gh-latest-commits-demo
```

Set the `DB_URL` environment variable to your postgresql database connection string, then build and run the application:

```console
export DB_URL="YOUR_CONNECTION_STRING"
cargo build --target wasm32-wasi 
slight -c slightfile.toml run target/wasm32-wasi/debug/gh-latest-commits-demo.wasm
```

Navigate to `localhost:3000/init_db` to initialize the database for the sample application.

Navigate to `localhost:3000/show_feeds` to show the latest commits for the default sample repos added during database initialization.

Add or remove entries in the *repos* table and refresh `localhost:3000/show_feeds` to see your changes. The newest entries in *repos* should be at the top.

> *NOTE:* When adding entries to the *repo* table, use the format `gh-user/repo`. For example, the repository at `https://github.com/deislabs/containerd-wasm-shims` would have the entry `deislabs/containerd-wasm-shims`.

## Run the application in Kubernetes

You can run the Slight demo application directly on your development computer.

Before you begin, you'll need:

- [Slight installed](https://github.com/deislabs/spiderlightning#installation)
- [Access to a postgresql database](https://www.postgresql.org/)
- [Docker Desktop installed](https://www.docker.com/products/docker-desktop/)
- Access to a Kubernetes cluster that can run WebAssembly modules, such as [k3d](https://k3d.io/) or [Azure Kubernetes Service](https://learn.microsoft.com/azure/aks/use-wasi-node-pools?wt.mc_id=azurelearn_inproduct_oss_wasm)
- Access to a container registry, such as [Dockerhub](https://hub.docker.com/), [ORAS registry](https://github.com/oras-project/distribution/pkgs/container/registry), or [Azure Container Registry](https://learn.microsoft.com/azure/container-registry/container-registry-get-started-azure-cli?wt.mc_id=azurelearn_inproduct_oss_wasm)

Clone the sample repo and navigate to the root directory:

```console
git clone URL
cd gh-latest-commits-demo
```

Build an image for the sample and push it to your registry:

```console
docker buildx build --platform=wasi/wasm -t REGISTRY_URL/gh-latest-commits-demo .
docker push REGISTRY_URL/gh-latest-commits-demo:latest
```

Update `gh-latest-commits-demo.yaml` with registry URL:

```yml
    spec:
      runtimeClassName: wasmtime-slight
      containers:
        - name: gh-latest-commits-demo
          image: REGISTRY_URL/gh-latest-commits-demo:latest
```

Create a secret with your database connection URL:

```console
kubectl create secret generic gh-latest-commits-demo-database --from-literal=connection-url='YOUR_CONNECTION_STRING'
```

Run the application in Kubernetes using `gh-latest-commits-demo.yaml`:

```console
kubectl apply -f gh-latest-commits-demo.yaml
```

Navigate to `DEMO_APP_IP/init_db` to initialize the database for the sample application.

Navigate to `DEMO_APP_IP/show_feeds` to show the latest commits for the default sample repos added during database initialization.

Add or remove entries in the *repos* table and refresh `DEMO_APP_IP/show_feeds` to see your changes. The newest entries in *repos* should be at the top.

> *NOTE:* When adding entries to the *repo* table, use the format `gh-user/repo`. For example, the repository at `https://github.com/deislabs/containerd-wasm-shims` would have the entry `deislabs/containerd-wasm-shims`.

