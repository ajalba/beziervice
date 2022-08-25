## Github container registry

Dado que GitHub es una herramienta que se ha empleado a lo largo de todo el curso y en la que se confía, es lógico usar GitHub Container Registry que se puede integrar con mucha facilidad.
Para ello ha sido necesario crear un token de acceso personal para github secrets y registrarse con él en docker, esto se ha hecho siguiendo la documentación oficial de github sobre token de acceso personal [(PAT)](https://docs.github.com/es/packages/working-with-a-github-packages-registry/working-with-the-container-registry).

Una vez realizado esto, se ha introducido el token como __github secrets__ de la misma forma que en la sección de [Docker Hub](./dockerhub_config.md). Con esta configuración se ha creado un workflow parecido al workflow anterior, pero con facilidades de variables de entorno debido a estar trabajando con GitHub. La imagen resultante es un paquete del repositorio y se puede ver en la sección __packages__ del repositorio.

Finalmente el workflow resultante es:

```
name: ghcr beziervice

on:
  push:
    branches: ['main']

env:
  REGISTRY: ghcr.io
  IMAGE_NAME: ${{ github.repository }}

jobs:
  build-and-push-image:
    runs-on: ubuntu-latest
    permissions:
      contents: read
      packages: write

    steps:
      - name: Checkout repository
        uses: actions/checkout@v3

      - name: Log in to the Container registry
        uses: docker/login-action@f054a8b539a109f9f41c372932f1ae047eff08c9
        with:
          registry: ${{ env.REGISTRY }}
          username: ${{ github.actor }}
          password: ${{ secrets.GITHUB_TOKEN }}

      - name: Extract metadata (tags, labels) for Docker
        id: meta
        uses: docker/metadata-action@98669ae865ea3cffbcbaa878cf57c20bbf1c6c38
        with:
          images: ${{ env.REGISTRY }}/${{ env.IMAGE_NAME }}

      - name: Build and push Docker image
        uses: docker/build-push-action@ad44023a93711e3deb337508980b4b5e9bcdc5dc
        with:
          context: .
          push: true
          tags: ${{ steps.meta.outputs.tags }}
          labels: ${{ steps.meta.outputs.labels }}
```