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
