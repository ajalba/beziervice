# Beziervice

__Service of Bézier's curves and surfaces__

## Justificación de las herramientas para tests elegidas

**Biblioteca de aserciones:**

Uno de los motivos por los que se ha seleccionado rust para este proyecto, ha sido el hecho de querer aprender el lenguaje de programación mediante un proyecto real. Rust incluye una biblioteca de asserciones TDD integrada en su ecosistema, cuyo uso esta muy extendendido en cada proyecto que he podido encontrar de tamaño mediano o grande. Esto no quita que se deban de investigar opciones de bibliotecas de aserciones implementadas con el objetivo de mejorar la propia de Rust.

Durante la investigación se han encontrado:

* [totems](https://crates.io/crates/totems): una biblioteca ligera de aserciones TDD, no se diferencia demasiado de la biblioteca estándar de rust, aunque aporta algo de flexibilidad al declarar aserciones, pero prácticamente no tiene usuarios, además las mejoras que provee no son significativas. Otro de los puntos que han hecho descartar la opción ha sido el hecho de que lleva 3 años sin recibir ninguna actualización.

* [more_asserts](https://docs.rs/more-asserts/latest/more_asserts/): otra biblioteca de aserciones que añade debug en los mensajes y flexibilidad en los tipos de aserciones.

* [pretty_assertions](https://github.com/colin-kiegel/rust-pretty-assertions): biblioteca que añade colores y amplifica los mensajes de fallo de las aserciones. Esta biblioteca recibió su última actualización en abril y es la opción más seguida y con más colaboración encontrada.

Finalmente me he decantado por **pretty_assertions** dado que sus mensajes de debug con colores en un formato que recuerda a las diff de plataformas como github hace que sea sencillo entender el fallo que ha hecho que el test no pase. Este formato ha sido hasta el momento de gran ayuda dado el tipo de programa que lidia con valores numéricos y fórmulas donde su formato facilita el debug.

**Marco de pruebas:**

Para el marco de pruebas me he decantado por el marco de tests de rust, un sistema TDD que viene integrado en el ecosistema y que además es el mas popular. No parece haber muchas alternativas, no obstante también he considerado el sistema [spectral](https://docs.rs/spectral/latest/spectral/), que es un framework de tests que incluye su propia biblioteca influenciada por Google Truth y frameworks de tests destinados a que sus tests sean lo mas legibles posible. Esta opción ha sido descartada por su poco mantenimiento y su uso de macros, que pueden dar lugar a tiempos de compilación más altos. Como otra opción se encuentra [galvanic-test](https://github.com/mindsbackyard/galvanic-test) que tiene el mismo problema de mantenimiento. Esta biblioteca permite agrupar tests y establecer una organización propia, aunque también requiere de macros para su biblioteca de aserciones que pueden realentizar tiempos de compilación.

El marco de tests integrado de rust se presenta como la opción más real y mejor optimizada para este proyecto.

**Task manager**

Se ha considerado cargo, el task manager oficial de rust, pero dado que es posible que algunas tareas requieran la ejecución de varios comandos y llamadas encadenadas, he elegido usar GNU Make para poder encapsular las llamadas en un solo argumento de make. Además es una herramienta sencilla de utilizar y que normalmente se encuentra instalada en la gran mayoría de sistemas linux. En ella se encuentran de momento las tareas troncales de cualquier proyecto de Rust, __build, test y run__ las cuales pueden ser invocadas mediante __make 'tarea'__.


## Decripción del problema, objetivos y lógica de negocio

Tanto la descripción del problema, como los objetivos y lógica de negocio pueden ser consultados [aquí](./docs/hito0/objetivo-logica-negocio.md)

### Historias de usuario, planificación y entidades

Tanto las historias de usuario del proyecto, como los issues derivados de las mismas, la planificación del proyecto y sus productos minimamente viables representados como milestones en github y las entidades y objetos-valor se pueden consultar [aquí](./docs/hito1/planificacion-us-entidades.md)

### Configuración de ramas para entregas

Para la convocatoria extraordinaria, dado que realizarán las entregas con los espacios de tiempo de la convocatoria ordinaria, se pretende crear __una rama por hito del proyecto__ con el objetivo de que cada hito quede representado de la forma más fiel posible y que su evaluación sea más cómoda (también para organización personal, todo sea dicho). Así pues, la rama máster tendrá el desarrollo del proyecto, y se creará una rama por hito del proyecto, denominadas __hito0, hito1, hito2 etc__.

### Configuración del entorno git

La configuración del entorno de git se ha separado en otro documento para no alargar en exceso el documento readme y se puede consultar [aqui](./docs/hito0/configuracion-entorno-git.md).
