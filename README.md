# Beziervice

__Service of Bézier's curves and surfaces__

## Descripción del problema y contexto

Las curvas de Bézier son un tipo de curvas empleadas en muchísimos ámbitos de informática gráfica debido a sus propiedades relacionadas con la forma y la facilidad de cómputo. Una de sus características principales es la utilización de puntos de control para crear la curva, lo que permite alterar la forma de la curva de forma sencilla y rápida.

Sin embargo, pese a su gran utilización estas curvas normalmente no son almacenadas para ser reutilizadas, si no que se almacena el resultado de la misma. Por ejemplo, en el estudio de animación donde trabajo, se utilizan curvas de Bézier para crear expresiones faciales, como la forma de las cejas o las arrugas de un personaje. Al finalizar el trabajo, se guarda el modelo resultante, pero no se almacena la curva, ni los puntos de control que la generan. Por tanto, si se quiere replicar la misma expresión mediante la misma curva en otro personaje, no es posible, se podrá emular, pero no se tendrá la misma. Existen muchos otros ejemplos donde se utilizan las curvas de Bézier en el mundo de la animación 3D, como las curvas que controlan la aceleración de las animaciones de objetos, ciertos tipos de filtros para imagen etc...

El hecho de que no sean reutilizables, hace que se repita mucho trabajo a veces y da lugar a resultados que no son exactamente iguales, lo cual puede entorpecer el proceso de producción.

Además de la informática gráfica, las curvas de Bézier se utilizan en otros ámbitos como la interpolación, la representación de gráficas de funciones o la aproximación de funciones.

### Historias de Usuario

- Como usuario, dado que trabajo a diario con curvas en 3D, quiero poder crear una curva de Bézier a partir de una serie de puntos de control y almacenarla para poder recuperarla y reutilizarla en el futuro.

- Como administrador, para poder dar un servicio reutilizable, debo poder identificar una curva de forma única para recuperarla y alterarla si es necesario.

- Como usuario, dado que empleo en mi trabajo funciones muy costosas computacionalmente, deseo poder aproximar una función de mi elección para obtener una serie de puntos que están muy proximos a la misma y así ahorrar capacidad de cómputo con el mínimo error posible.

- Como usuario, quiero poder emplear dos curvas de Bézier, las haya guardado antes o no en el sistema, para poder crear una superficie para mi modelo 3D y poder tenerlas a mi disposición cuando quiera.

### Planificación

Se ha realizado una planificación del proyecto, para ello se han creado una serie de hitos, los cuales definen productos entregables, o MVP:

- Generar una curva de grado n, a partir de n+1 puntos.
- Crear una serie de curvas de grado n con union continua, a partir de n*m - 1 puntos.
- Aproximar una función dada, utilizando una serie de curvas de grado 2 o 3.
- Crear la estructura necesaria para almacenar, modificar, leer y eliminar curvas bajo demanda de los usuarios.
- Crear la estructura necesaria para aproximar una función y almacenar, modificar, leer o eliminar su resultado bajo demanda de los usuarios.
- Crear superficies de Bézier a partir de dos curvas.

Estos hitos definen el desarrollo del proyecto desde el punto más básico posible, como crear una curva simple, hasta obtener funcionalidades derivadas de las mismas como la capacidad de aproximación y la creación de superficies.

Para la generación de las curvas se empleará una base de polinomios especial y un algoritmo de generación llamado el algoritmo de De Casteljau. Una vez se hayan implementado los algoritmos, se utilizarán para la posterior creación de superficies. Además, el algoritmo de De Casteljau será la base de otros algoritmos de interpolación de funciones.

Siguiendo este plan, los hitos agruparán una serie de productos minimamente viables, que harán que el servicio crezca de forma progresiva y lógica.

### Entidades, Objetos-valor y agregados

De las historias de usuario se pueden obtener las entidades **Usuario**, **Curvas**(de Bézier) y **Superficies**, pues serán entidades que no cambiarán en ningún momento del desarrollo del proyecto. Una curva será siempre una curva y es una entidad per se, por otro lado, los usuarios son las entidades básicas y objetos de la gran mayoría de servicios, por lo que es necesario representarlo en el proyecto. Estará representado por un nombre de usuario y una contraseña, como estructura básica, y esta será facilmente ampliable si es necesario con tokens de acceso para identificar el usuario durante un periodo de tiempo, propiedades sobre el mismo para representar derechos etc...

Las superficies estarán representadas por dos curvas, pues se generarán a partir de ellas y sus puntos.

Los objetos-valor que se derivan de las historias de usuario serían los puntos y el grado. Un punto estará constituido por un número de coordenadas, 2 o 3 dependiendo de si se quiere trabajar en el plano o en el espacio. El grado será un entero pequeño, por razones de cómputo, estará restringido a un entero menor que 10.

### Objetivo del proyecto

El objetivo del proyecto es actuar como servicio de creación y almacenamiento de curvas y superficies de Bézier, ya sean creadas para la modelización en 3D o para su uso científico. Se proporcionará servicio de representación de las curvas y superficies, así como servicio de interpolación de funciones.

Se pretende que el servicio actúe de la forma mas independiente posible, para que así pueda ser integrado en el amplio repertorio de ámbitos descritos previamente.

### Lógica de negocio

La lógica de negocio del servicio se basa en la creación de curvas y superficies y la creación de un formato de almacenamiento que permita que estas sean lo más reutilizables posibles. Por otro lado, se espera contar con tiempos de respuesta lo más rápidos posibles, que sean menores a 500ms, lo que se cuenta como tiempo real.

### Configuración de ramas para entregas

Para la convocatoria extraordinaria, dado que realizarán las entregas con los espacios de tiempo de la convocatoria ordinaria, se pretende crear __una rama por hito del proyecto__ con el objetivo de que cada hito quede representado de la forma más fiel posible y que su evaluación sea más cómoda (también para organización personal, todo sea dicho). Así pues, la rama máster tendrá el desarrollo del proyecto, y se creará una rama por hito del proyecto, denominadas __hito0, hito1, hito2 etc__.

### Configuración del entorno git

La configuración del entorno de git se ha separado en otro documento para no alargar en exceso el documento readme y se puede consultar [aqui](./docs/hito0/configuracion-entorno-git.md).
