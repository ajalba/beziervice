# Descripción del problema y contexto

Las curvas de Bézier son un tipo de curvas empleadas en muchísimos ámbitos de informática gráfica debido a sus propiedades relacionadas con la forma y la facilidad de cómputo. Una de sus características principales es la utilización de puntos de control para crear la curva, lo que permite alterar la forma de la curva de forma sencilla y rápida.

Sin embargo, pese a su gran utilización estas curvas normalmente no son almacenadas para ser reutilizadas, si no que se almacena el resultado de la misma. Por ejemplo, en el estudio de animación donde trabajo, se utilizan curvas de Bézier para crear expresiones faciales, como la forma de las cejas o las arrugas de un personaje. Al finalizar el trabajo, se guarda el modelo resultante, pero no se almacena la curva, ni los puntos de control que la generan. Por tanto, si se quiere replicar la misma expresión mediante la misma curva en otro personaje, no es posible, se podrá emular, pero no se tendrá la misma. Existen muchos otros ejemplos donde se utilizan las curvas de Bézier en el mundo de la animación 3D, como las curvas que controlan la aceleración de las animaciones de objetos, ciertos tipos de filtros para imagen etc...

El hecho de que no sean reutilizables, hace que se repita mucho trabajo a veces y da lugar a resultados que no son exactamente iguales, lo cual puede entorpecer el proceso de producción.

Además de la informática gráfica, las curvas de Bézier se utilizan en otros ámbitos como la interpolación, la representación de gráficas de funciones o la aproximación de funciones.

## Objetivo del proyecto

El objetivo del proyecto es actuar como servicio de creación y almacenamiento de curvas y superficies de Bézier, ya sean creadas para la modelización en 3D o para su uso científico. Se proporcionará servicio de representación de las curvas y superficies, así como servicio de interpolación de funciones.

Se pretende que el servicio actúe de la forma mas independiente posible, para que así pueda ser integrado en el amplio repertorio de ámbitos descritos previamente.

### Lógica de negocio

La lógica de negocio del servicio se basa en la creación de curvas y superficies y la creación de un formato de almacenamiento que permita que estas sean lo más reutilizables posibles. Por otro lado, se espera contar con tiempos de respuesta lo más rápidos posibles, que sean menores a 500ms, lo que se cuenta como tiempo real.
