/*
En C++ o Java simplemente puedes crear puntero o referencia.
En Rust, el Box<T> es obligatorio para estructuras recursivas (árboles, listas enlazadas...)
por la parte física de la memoria
*/
struct Node{
    value: i32,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

//Constructor
impl Node{
    fn new(value:i32) -> Self{
        Node{
            value,
            left: None,
            right: None,}

    }

    fn insert(&mut self, valor: i32){
        match valor {
            //self.left y self.right son Option<Box<Node>>
            x if x == self.value => {
                println!("The value is already on the tree :)")
            },
            x if x < self.value => {
                match &mut self.left {
                    Some(left) => left.insert(valor),
                    None => {
                        let new = Node::new(valor);
                        self.left = Some(Box::new(new)); 
                    },
                }
            },
            _ => { //Caso derecha
                match &mut self.right {
                    Some(right) => right.insert(valor),
                    None => {
                        let new = Node::new(valor);
                        self.right = Some(Box::new(new)); 
                    },
                }            
            }, 
        }
    }

    fn search(&self, valor: i32) -> bool{
        //Como este es un arbol ordenado (izq menor, derecha mayor)
        match valor {
            //self.left y self.right son Option<Box<Node>>
            x if x == self.value => {
                true
            },
            x if x < self.value => {
                match &self.left {
                    Some(left) => left.search(valor),
                    None => false,
                }
            },
            _ => { //Caso derecha
                match &self.right {
                    Some(right) => right.search(valor),
                    None => false,
                }            
            }, 
        }
    }

    fn print_value(&self) -> i32{
        self.value
    }

    //No devuelve nada el método, porque se hace por el mut
    fn inOrder(&self, v: &mut Vec<i32>){
        //if let => "Si Option tiene algo dentro, préstame una referencia a ese algo para trabajar con él"
        //ref => Dame una referencia del nodo hijo (no me lo lleves)
        
        //IZQ
        if let Some(ref left_child) = self.left{
            left_child.inOrder(v)
        }
        
        //VALOR RAÍZ
        v.push(self.value);
        
        //DERECHA
        if let Some(ref right_child) = self.right{
            right_child.inOrder(v)
        }
    }
}

use rand::Rng; // Random

fn main(){
    //En Rust todo es inmutable.
    //mut node para cambiar node.left después de haber creado node.
    let mut rng = rand::thread_rng();
    let num = 3;
    let mut node = Node::new(num);
    let right = Node::new(5);
    let left = Node::new(1);

    // 1. Metemos el nodo en un Box (heap)
    // 2. Metemos el Box en un Some (Option)
    node.left = Some(Box::new(left));
    node.right = Some(Box::new(right));

    for _ in 0..100 {
        let num = rng.gen_range(1..1000);
        node.insert(num);
    }
    println!("value of Node is {}", node.print_value());
    println!("¿Existe el 5?: {}", node.search(5));
    println!("¿Existe el 20?: {}", node.search(20));

    let mut v = Vec::new();
    node.inOrder(&mut v);
    println!("InOrder: {:?}", v);
}