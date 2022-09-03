## Framework elegido para el microservicio

Actualmente en el ecosistema de rust existen 3 frameworks principales para implementar un servicio REST, entre muchos otros no incluidos por presentar una versión aun muy joven del framework o notar un poco mantenimiento sobre el mismo:

- [warp](https://crates.io/crates/warp): Framework web con el objetivo de hacer fácil el routing de servicios y disminuir al máximo las dependencias de crates externos, centrado en programación funcional, aunque aun joven y con demasiadas funcionalidades a implementar.

-[Axum](https://crates.io/crates/axum): Desarrollado por los creadores de otra biblioteca asíncrona de rust llamada Tokyo, este framework se centra en la modularidad y la ergonomía. Este framework fue descartado por la juventud del mismo y la falta de una documentación más extensa.

- [Actix-web](https://crates.io/crates/actix-web): conocido por ser el framework de rust más rápido en [TechEmpower](https://www.techempower.com/benchmarks/#section=data-r20), web que ofrece una clasificación de frameworks web en todos los lenguajes de programación basada en sus propios benchmarks. Este ha sido el framework elegido, a parte de su velocidad idónea para cálculo, por ser el framework más maduro basado en rust, en este proyecto se emplea su versión 4.0. Además las funcionalidades relacionadas con el routing y los tests son muy completas y utilizan metodologías similares a las utilizadas para los tests unitarios.

## Diseño general de la API y rutas

El diseño de las rutas se ha divido en dos servicios, uno dedicado a la interpolación y el otro dedicado a las curvas. Esto servicios se definen en el momento de la creación de la aplicación del servidor en la función main. Todo lo relacionado con la API se encuentra en una carpeta dedicada a la misma para favorecer la modularidad del proyecto. En esta carpeta se han dispuesto dos archivos, el archivo __handlers__ dedicado a realizar las operaciones relacionadas con cada petición, y el archivo [test](../../Beziervice/src/api/tests.rs), dedicado a los test, y donde se pueden ver de forma rápida las rutas programadas.

Respecto a las curvas de Bézier, se ha creado el servicio de creación de curvas, que funciona mediante una petición post a /create_curve, donde el cuerpo de la petición conforme los puntos de la curva en 3 vectores de números reales, uno por eje 3D. Mediante otra petición post a /evaluate_curve, se puede crear y evaluar una curva de Bézier. Por último se puede obtener una curva de Bézier básica haciendo una petición get a /.

En el otro servicio se pueden realizar peticiones get a /interpolate_curve/{grade}, donde el parámetro grade indica el grado del interpolante utilizado, en este momento 2 o 3. Esta petición devuelve una aproximación de la función seno.

## Logger y middleware

Se ha empleado el middleware de actix pues es el que mejor integración presenta y para los logs se ha utilizado el logger [simple_logger](https://crates.io/crates/simple_logger), que es el logger de mayor versión y soporte encontrado y que ofrece logs verbose y de facil lectura.

## Tests y relación con las historias de usuario

Los tests añadidos a las rutas del servicio web consiguen cerrar las siguientes historias de usuario.

- [[HU3] Como usuario, dado que empleo en mi trabajo funciones muy costosas computacionalmente, deseo poder aproximar una función de mi elección para obtener una serie de puntos que están muy proximos a la misma y así ahorrar capacidad de cómputo con el mínimo error posible.](https://github.com/ajalba/beziervice/issues/3)

- [[HU1] Como usuario, dado que trabajo a diario con curvas en 3D, quiero poder crear una curva de Bézier a partir de una serie de puntos de control y almacenarla para poder recuperarla y reutilizarla en el futuro.](https://github.com/ajalba/beziervice/issues/1)

Lo cual comprende la mitad de las historias de usuario pleadas en el proyecto, a falta de conexión con la base de datos y algoritmos, el objetivo principal del proyecto se ve representado por la lógica representada en el código.
