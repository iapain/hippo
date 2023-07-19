### Hippo - Generates and manage your git repo on server

***Not production ready*** Use at your own risk.

Hippo solve a common problem, how to manage your git repo on servers like github, bitbucket or gitlab.
It not only creates repo but also allows you to define various properties like CI/CD, variables, settings etc.

Here is a common example for repository manifest.

```yaml
apiVersion: hippo.tool/v1alpha1
kind: Repository
source: bitbucket
metadata:
  name: Learning Path
  description: My cool app
  homePage: https://myapp.com
spec:
  organization: forlagshuset
  slug: learning-path-service
  private: true
  mainBranch: main
  deployments:
    - name: Test
      envs:
        - key: NODE_ENV
          value: testing
  pipeline:
    enable: true
  envs:
    - key: MONGODB_URL
      value: mongodb://127.0.0.1:27017/testdb
      secure: true
    - key: CI_IMAGE_URL
      value: mycoolregistry.com/cool-service
    - key: CI_DEPLOYMENT_NAME
      value: deployment/cool-service
```

### Installation
You'll need rustc >= 1.65 to compile and run this tool

```
cargo run -- --file my-repo-manifest.yaml
```

### License
MIT License
