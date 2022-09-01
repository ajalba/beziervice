# Beziervice

__Service of Bézier's curves and surfaces__

Proyecto dedicado a la construcción de curvas de Bezier en rust de forma rápida, eficaz y reutilizable.

En estos momentos el proyecto tiene configurada la integración continua y se encuentra en la fase previa a convertirse en un microservicio de curvas de Bézier, es posible crear curvas en este momento y también evaluarlar y aproximar funciones.

## Integración continua, configuración en diferentes sistemas

Se ha configurado la integración continua en los sistemas Travis y CircleCI.

### Intregración continua: Travis

Para poder emplear Travis es necesario darse de alta en la plataforma empleando la cuenta personal de github y en ella simplemente autorizar a travis a obtener datos de nuestro repositorio. Una vez hecho esto basta con seleccionar el repositorio donde se quiere implementar la integración continua. Travis accederá por defecto al fichero __.travis.yml__ que se coloque en la raíz del repositorio, mediante el cual se configura la integración continua.

```yml
language: rust
rust:
  - stable
cache: cargo
install: 
  - make build
script:
  - make test
```

El fichero __.travis.yml__ toma como base el [ejemplo que travis ofrece](https://docs.travis-ci.com/user/languages/rust/), toma la imagen de rust estable para ahorrar tiempo en travis, y activa la caché de cargo para no recompilar dependencias, emplea make para ejecutar los tests.

### Integración continua: circleCI

Este sistema de integración continua facilita el uso de imágenes Docker para el testeo de aplicaciones, es por esto por lo que ha sido seleccionado como sistema adicional. Además, empleo la imagen de docker del hito anterior para realizar la integración continua dentro de la misma.

Para emplear circleCI se sigue un proceso similar al de travis, es necesario darse de alta en su plataforma web, autorizar al sistema a acceder a nuestro github, y seleccionar el repositorio, una vez hecho esto se nos ofrece comenzar con una versión de prueba en la ruta __.circleci/config.yml__. Es necesario modificar el archivo para la integración continua, mi configuración es la siguiente:

```
version: 2.1

jobs:
    test:
        docker:
            - image: abeljosesanchez/beziervice
        steps:
            - checkout
            - run: make test

workflows:
    test_docker:
        jobs:
            - test
```

Es una version reducida de los ejemplos que [circleci ofrece](https://circleci.com/docs/sample-config), donde se parte de un solo __Job__; test, al cual se le especifica que se quiere usar docker, la imagen que se quiere emplear. En los __steps__ primero se indica mediante __checkout__ que se descargue el repositorio base donde se tiene activada la integración continua y el último paso es ejecutar los tests. Por último el workflow indica que se debe ejecutar el __Job__ test.

## Imagen elegida como base para tests

La imagen elegida como base para los tests se encuentra en el fichero Dockerfile, si se quiere saber sobre el por qué de su elección se puede acceder [aquí](./docs/hito3/documentacion-dockerfile.md).

## Actualización automática del contenedor: Dockerhub y Github Container Registry

Para la actualización automática del contenedor se han empleado dockerhub y GHCR, la configuración y puesta en marcha de Dockerhub se encuentra [aquí](./docs/hito3/documentacion-dockerhub.md), y se tiene la documentación y puesta en marcha de Github Container Registry en el siguiente [enlace](./docs/hito3/documentacion-ghcr.md).

## Decripción del problema, objetivos y lógica de negocio

Tanto la descripción del problema, como los objetivos y lógica de negocio pueden ser consultados [aquí](./docs/hito0/objetivo-logica-negocio.md)

### Historias de usuario, planificación y entidades

Tanto las historias de usuario del proyecto, como los issues derivados de las mismas, la planificación del proyecto y sus productos minimamente viables representados como milestones en github y las entidades y objetos-valor se pueden consultar [aquí](./docs/hito1/planificacion-us-entidades.md)

### Herramientas de test elegidas

Para los tests del proyecto se han elegido una biblioteca de aserciones, en este caso ha sido [pretty_assertions](https://github.com/colin-kiegel/rust-pretty-assertions), se emplea el marco de test de Rust y el gestor de tareas es GNU make, si se quiere leer más en profundidad sobre las elecciones tomadas, la justificación se encuentra [aqui](./docs/hito2/justificacion-herramientas.md).

### Configuración de ramas para entregas

Para la convocatoria extraordinaria, dado que realizarán las entregas con los espacios de tiempo de la convocatoria ordinaria, se pretende crear __una rama por hito del proyecto__ con el objetivo de que cada hito quede representado de la forma más fiel posible y que su evaluación sea más cómoda (también para organización personal, todo sea dicho). Así pues, la rama máster tendrá el desarrollo del proyecto, y se creará una rama por hito del proyecto, denominadas __hito0, hito1, hito2 etc__.

### Configuración del entorno git

La configuración del entorno de git se ha separado en otro documento para no alargar en exceso el documento readme y se puede consultar [aqui](./docs/hito0/configuracion-entorno-git.md).
