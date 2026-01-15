# Binary Search Tree (BST) in Rust 🦀

Este es un proyecto de entrenamiento realizado como preparación para un reto semanal de programación. El objetivo es dominar los conceptos de gestión de memoria, propiedad (*ownership*) y recursividad en Rust mediante la implementación de un Árbol Binario de Búsqueda.

## 🚀 Características

- **Inserción Ordenada**: Mantiene la propiedad de BST (izquierdos menores, derechos mayores).
- **Búsqueda Eficiente**: Implementación recursiva aprovechando el orden del árbol.
- **Recorrido In-Order**: Función que recolecta los elementos en un `Vec<i32>` de forma ordenada.
- **Pruebas con Datos Aleatorios**: Integración con la crate `rand` para verificar el comportamiento con grandes volúmenes de datos.

## 🧠 Conceptos de Rust Aplicados

* **Smart Pointers (`Box<T>`)**: Utilizados para permitir tipos recursivos, moviendo los nodos del *stack* al *heap*.
* **Manejo de Ausencia (`Option<T>`)**: Uso de `Some` y `None` para gestionar hijos de forma segura sin punteros nulos.
* **Pattern Matching**: Uso extensivo de `match` e `if let` para navegar por la estructura del árbol.
* **Borrow Checker**: Gestión de referencias mutables (`&mut self`) para la inserción y referencias inmutables para la búsqueda.

## 🛠️ Instalación y Uso

1. Asegúrate de tener Rust y Cargo instalados.
2. Clona el repositorio:
   ```bash
   git clone [https://github.com/tu_usuario/Rust-BST-Tree.git](https://github.com/tu_usuario/Rust-BST-Tree.git)
   cd Rust-BST-Tree
   ```
3. Ejecuta el proyecto.
   `cargo run`
