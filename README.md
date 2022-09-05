# Beziervice

__Service of Bézier's curves and surfaces__

Proyecto dedicado a la construcción de curvas de Bezier en rust de forma rápida, eficaz y reutilizable.

Tras tener los tests y la integración continua configuradas. El proyecto ha visto la luz como servicio web, donde se han configurado una serie de rutas destinadas a cumplir con las historias de usuario y que ofrecen interacción con curvas de Bézier para fomentar el buen uso de las mismas y optimizar el trabajo de otras personas con ellas.

En estos momentos es posible disponer de una base de datos integrada al proyecto, donde el servicio web es capaz de realizar un CRUD de curvas de Bézier. Dicho CRUD consiste en crear, leer(read), actualizar(update) y eliminar(delete) curvas de la base de datos. Las rutas se explican con más detalle en su sección.

## Uso en su forma actual con base de datos

La base de datos se encuentra alojada en una imagen docker que se lanza mediante docker-compose, es posible saber más en el fichero [docker-compose](./docker-compose.yml).
Es necesario lanzar docker-compose. Una vez hecho esto se debe incluir en la carpeta __Beziervice__ un archivo .env para configurar la variable de entorno __DATABASE_URL=postgres://user:password@host/dbname__ siguiendo esta plantilla. El último paso de la configuración sería ejecutar en la carpeta __Beziervice__, el comando __diesel setup__ para levantar la base de datos.

Una vez hecho esto, es suficiente con ejecutar make run y disponer así de un servicio web de curvas de Bézier.

## Servicio Beziervice

### Framework elegido para el microservicio

Actualmente en el ecosistema de rust existen 3 frameworks principales para implementar un servicio REST, entre muchos otros no incluidos por presentar una versión aun muy joven del framework o notar un poco mantenimiento sobre el mismo:

- [warp](https://crates.io/crates/warp): Framework web con el objetivo de hacer fácil el routing de servicios y disminuir al máximo las dependencias de crates externos, centrado en programación funcional, aunque aun joven y con demasiadas funcionalidades a implementar.

-[Axum](https://crates.io/crates/axum): Desarrollado por los creadores de otra biblioteca asíncrona de rust llamada Tokyo, este framework se centra en la modularidad y la ergonomía. Este framework fue descartado por la juventud del mismo y la falta de una documentación más extensa.

- [Actix-web](https://crates.io/crates/actix-web): conocido por ser el framework de rust más rápido en [TechEmpower](https://www.techempower.com/benchmarks/#section=data-r20), web que ofrece una clasificación de frameworks web en todos los lenguajes de programación basada en sus propios benchmarks. Este ha sido el framework elegido, a parte de su velocidad idónea para cálculo, por ser el framework más maduro basado en rust, en este proyecto se emplea su versión 4.0. Además las funcionalidades relacionadas con el routing y los tests son muy completas y utilizan metodologías similares a las utilizadas para los tests unitarios.

### Diseño general de la API y rutas

El diseño de las rutas se ha divido en dos servicios, uno dedicado a la interpolación y el otro dedicado a las curvas. Esto servicios se definen en el momento de la creación de la aplicación del servidor en la función main. Todo lo relacionado con la API se encuentra en una carpeta dedicada a la misma para favorecer la modularidad del proyecto. En esta carpeta se han dispuesto dos archivos, el archivo __handlers__ dedicado a realizar las operaciones relacionadas con cada petición, y el archivo [test](../../Beziervice/src/api/tests.rs), dedicado a los test, y donde se pueden ver de forma rápida las rutas programadas.

Respecto a las curvas de Bézier, se ha creado el servicio de creación de curvas, que funciona mediante una petición post a /create_curve, donde el cuerpo de la petición conforme los puntos de la curva en 3 vectores de números reales, uno por eje 3D. Mediante otra petición post a /evaluate_curve, se puede crear y evaluar una curva de Bézier. Por último se puede obtener una curva de Bézier básica haciendo una petición get a /.

En el otro servicio se pueden realizar peticiones get a /interpolate_curve/{grade}, donde el parámetro grade indica el grado del interpolante utilizado, en este momento 2 o 3. Esta petición devuelve una aproximación de la función seno.

Finalmente para las rutas del CRUD he elegido la ruta __get_function/{id}__ para obtener la curva de Bézier de la base de datos indicada por el id. Para crear una curva se ha seleccionado la ruta __create_curve__ que mediante una peticion post con el json que aparece en el ejemplo creará una curva y devolverá los dato de la misma en formato json. Para actualizar una curva se requiere del mismo Json pero añadiendo el campo __id__ y realizando una petición put, como indica el protocolo HTPP para actualizaciones. Finalmente para eliminar una curva se ha decidido emplear la petición de tipo delete, a la ruta __/delete_curve__, con un Json como cuerpo cuyo único campo es el id de la curva. La curva eliminada se devuelve como cuerpo de la respuesta. 

``` json
{
    "name": "IT WORKS",
    "points_x": [0.0, 1.0, 2.0],
    "points_y": [1.0, 1.0, 2.0],
    "points_z": [0.0, 1.0, 3.0]
}
```

### Logger y middleware

Se ha empleado el middleware de actix pues es el que mejor integración presenta y para los logs se ha utilizado el logger [simple_logger](https://crates.io/crates/simple_logger), que es el logger de mayor versión y soporte encontrado y que ofrece logs verbose y de facil lectura.

### Tests y relación con las historias de usuario

Los tests añadidos a las rutas del servicio web consiguen cerrar las siguientes historias de usuario.

- [[HU3] Como usuario, dado que empleo en mi trabajo funciones muy costosas computacionalmente, deseo poder aproximar una función de mi elección para obtener una serie de puntos que están muy proximos a la misma y así ahorrar capacidad de cómputo con el mínimo error posible.](https://github.com/ajalba/beziervice/issues/3)

- [[HU1] Como usuario, dado que trabajo a diario con curvas en 3D, quiero poder crear una curva de Bézier a partir de una serie de puntos de control y almacenarla para poder recuperarla y reutilizarla en el futuro.](https://github.com/ajalba/beziervice/issues/1)

- [[HU2] Como administrador, para poder dar un servicio reutilizable, debo poder identificar una curva de forma única para recuperarla y alterarla si es necesario.](https://github.com/ajalba/beziervice/issues/2)

Lo cual comprende 3 de las 4 historias de usuario planteadas para el proyecto, por lo que a falta de los algoritmos de superficies, se han cumplido una cantidad considerable de objetivos del proyecto, además de sentar una base y estructura adecuadas para su propio escalado.

## Integración continua, configuración en diferentes sistemas

Se ha configurado la integración continua en los sistemas Travis y CircleCI. Es posible encontrar la documentación detallada sobre estos sistemas de CI [aquí](./docs/hito4/documentacion-ci.md).

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
