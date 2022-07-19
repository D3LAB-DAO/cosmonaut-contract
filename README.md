## How to Run


1. Build docker image and create volume for common packages
```shell
./init.sh $cargo_registry_mount_path
```
2. Create volume for user packages

3. Edit contract path for dependency of packages
```shell
packages/*/Cargo.toml
```
4. Edit parameters in run.sh

5. Run target lesson-chapter binary
```shell
./run.sh $lesson_number $chaper_number
```
