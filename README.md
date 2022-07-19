# Beziervice

__Service of Bézier's curves and surfaces__

## Decripción del problema, objetivos y lógica de negocio

Tanto la descripción del problema, como los objetivos y lógica de negocio pueden ser consultados [aquí](./docs/hito0/objetivo-logica-negocio.md)

### Historias de Usuario

Con el objetivo de comenzar el desarrollo del proyecto, se han creado una serie de historias de usuario que conecten a los usuarios con el problema a resolver y la lógica de negocio del proyecto. De estas historias de usuario se han derivado una serie de issues, los cuales han sido agrupados en Milestones, además de deducirse una serie de indentidades y de objetos-valor. A continuación se listan las historias de usuario, los issues y milestones se han dejado fuera de este documento dado que se encuentran en el repositorio y en este documento pueden hacer la lectura pesada.

- Como usuario, dado que trabajo a diario con curvas en 3D, quiero poder crear una curva de Bézier a partir de una serie de puntos de control y almacenarla para poder recuperarla y reutilizarla en el futuro.

- Como administrador, para poder dar un servicio reutilizable, debo poder identificar una curva de forma única para recuperarla y alterarla si es necesario.

- Como usuario, dado que empleo en mi trabajo funciones muy costosas computacionalmente, deseo poder aproximar una función de mi elección para obtener una serie de puntos que están muy proximos a la misma y así ahorrar capacidad de cómputo con el mínimo error posible.

- Como usuario, quiero poder emplear dos curvas de Bézier, las haya guardado antes o no en el sistema, para poder crear una superficie para mi modelo 3D y poder tenerlas a mi disposición cuando quiera.

### Planificación

Se ha realizado una planificación del proyecto, para ello se han creado una serie de hitos, los cuales definen productos entregables, o MVP. Estos hitos definen el desarrollo del proyecto desde el punto más básico posible, como crear una curva simple, hasta obtener funcionalidades derivadas de las mismas como la capacidad de aproximación y la creación de superficies.

Para la generación de las curvas se empleará una base de polinomios especial y un algoritmo de generación llamado el algoritmo de De Casteljau. Una vez se hayan implementado los algoritmos, se utilizarán para la posterior creación de superficies. Además, el algoritmo de De Casteljau será la base de otros algoritmos de interpolación de funciones. Como se puede ver, sin haber terminado un MVP o hito, no es posible progresar en el siguiente, por lo que el proyecto se desarrolla de forma constructiva donde cada hito es esencial para el siguiente MVP.

Siguiendo este plan, los hitos agruparán una serie de productos minimamente viables, que harán que el servicio crezca de forma progresiva y lógica. Cada uno de los elementos de la siguiente lista, representa las capacidades que tendrá el producto en dicha fase de desarrollo.

- Generar una curva de grado n, a partir de n+1 puntos.
- Crear una serie de curvas de grado n con union continua, a partir de n*m - 1 puntos.
- Aproximar una función dada, utilizando una serie de curvas de grado 2 o 3.
- Crear la estructura necesaria para almacenar, modificar, leer y eliminar curvas bajo demanda de los usuarios.
- Crear la estructura necesaria para aproximar una función y almacenar, modificar, leer o eliminar su resultado bajo demanda de los usuarios.
- Crear superficies de Bézier a partir de dos curvas.

### Entidades, Objetos-valor y agregados

De las historias de usuario se pueden obtener las entidades **Usuario**, **Curvas**(de Bézier) y **Superficies**, pues serán entidades que no cambiarán en ningún momento del desarrollo del proyecto. Una curva será siempre una curva y es una entidad per se, por otro lado, los usuarios son las entidades básicas y objetos de la gran mayoría de servicios, por lo que es necesario representarlo en el proyecto. Estará representado por un nombre de usuario y una contraseña, como estructura básica, y esta será facilmente ampliable si es necesario con tokens de acceso para identificar el usuario durante un periodo de tiempo, propiedades sobre el mismo para representar derechos etc...

Las superficies estarán representadas por dos curvas, pues se generarán a partir de ellas y sus puntos.

Los objetos-valor que se derivan de las historias de usuario serían los puntos y el grado. Un punto estará constituido por un número de coordenadas, 2 o 3 dependiendo de si se quiere trabajar en el plano o en el espacio. El grado será un entero pequeño, por razones de cómputo, estará restringido a un entero menor que 10.

### Configuración de ramas para entregas

Para la convocatoria extraordinaria, dado que realizarán las entregas con los espacios de tiempo de la convocatoria ordinaria, se pretende crear __una rama por hito del proyecto__ con el objetivo de que cada hito quede representado de la forma más fiel posible y que su evaluación sea más cómoda (también para organización personal, todo sea dicho). Así pues, la rama máster tendrá el desarrollo del proyecto, y se creará una rama por hito del proyecto, denominadas __hito0, hito1, hito2 etc__.

### Configuración del entorno git

La configuración del entorno de git se ha separado en otro documento para no alargar en exceso el documento readme y se puede consultar [aqui](./docs/hito0/configuracion-entorno-git.md).
