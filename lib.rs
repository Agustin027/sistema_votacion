/*Cosas a consultar
- use chrono::{TimeZone, Utc}; // cuando quiero usar chrono me revienta todo :(

-lo que yo queria hacer con el chrono era recibir una fecha en formato dd/mm/yyyy y
  convertirla a timestamp para poder compararla con el timestamp actual y asi saber
  si la eleccion esta activa o no.
  Lo que voy a hacer por ahora es pasar la fecha en formato timestamp directamente.

-El votante puede votar en varias elecciones? o solo en una? cuando deberia registrarse en una eleccion? cuando se registra en el sistema? o con una funcion aparte?

-Hay alguna manera mejor de verificar todas las condiciones de los usuarios? (ejemplo: que no se registre el admin como usuario normal) por que lo estoy haciendo con ifs y panics y queda feo

-Como deberia ser el registro de votos? deberia ser un struct aparte? o deberia ser un metodo de la eleccion?

*/

#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod sistema_votacion {
    use core::panic;

    use ink::env;
    use ink::prelude::collections::BTreeMap;
    use ink::prelude::string::String;
    use ink::prelude::vec::Vec;
    #[ink(storage)]
    pub struct SistemaVotacion {
        admin: Admin,
        elecciones: Vec<Eleccion>,
        usuarios: Vec<Usuario>,
    }

    impl SistemaVotacion {
        //----------------------Constructor y manejo de eleccion-------------------------------------------------------
        /// Constructor de la estructura SistemaVotacion que inicializa el admin y la primera eleccion del sistema con los datos ingresados
        #[ink(constructor)]
        pub fn new(cargo: String, fecha_inicio: u64, fecha_fin: u64) -> Self {
            let caller = Self::env().caller();
            let admin = Admin {
                id: caller,
                nombre: String::from("admin"),
                email: String::from("mail.com"),
                password: String::from("1234"),
            };
            let mut elecciones = Vec::new();
            let eleccion = Eleccion {
                id: 0,
                cargo,
                fecha_inicio,
                fecha_fin,
                candidatos: BTreeMap::new(),
                votantes: Vec::new(),
                estado: false,
            };
            elecciones.push(eleccion);
            let usuarios = Vec::new();
            Self {
                admin,
                elecciones,
                usuarios,
            }
        }

        #[ink(message)]
        /// Funcion para obtener el id del admin
        pub fn get_id_admin(&self) -> AccountId {
            self.admin.id
        }
        #[ink(message)]
        /// Funcion para settear un nuevo admin
        pub fn set_admin(&mut self, nombre: String, email: String, password: String) {
            self.admin = Admin {
                id: self.env().caller(),
                nombre,
                email,
                password,
            };
        }
        #[ink(message)]
        /// funcion para crear una nueva eleccion
        pub fn crear_eleccion(&mut self, cargo: String, fecha_inicio: u64, fecha_fin: u64) {
            let eleccion = Eleccion {
                id: self.elecciones.len() as u64,
                cargo,
                fecha_inicio,
                fecha_fin,
                candidatos: BTreeMap::new(),
                votantes: Vec::new(),
                estado: false,
            };
            self.elecciones.push(eleccion);
        }
        #[ink(message)]
        /// Funcion para cambiar el estado de una eleccion
        pub fn cambiar_estado_eleccion(&mut self, id: u64) {
            self.elecciones[id as usize].estado = !self.elecciones[id as usize].estado;
        }
        //----------------------Funciones de registro---------------------------------------------------------
        #[ink(message)]
        /// Funcion para registrar un usuario en el sistema con los datos ingresados,distingue entre votante y candidato y verifica que no se registre el admin como usuario normal
        pub fn registrar_usuario(&mut self, nombre: String, email: String, rol: RolUsuario) {
            let usuario = Usuario {
                id: self.env().caller(),
                nombre,
                email,
                rol,
            };
            if self.usuarios.iter().any(|u| u.id == usuario.id) || usuario.id == self.admin.id {
                if usuario.id == self.admin.id {
                    panic!("El admin no puede registrarse como un usuario normal");
                } else {
                    panic!("El usuario ya está registrado");
                }
            } else {
                self.usuarios.push(usuario);
            }
        }
        #[ink(message)]
        /// Funcion para registrar un votante en una eleccion
        pub fn registrar_votante_en_eleccion(&mut self, id_eleccion: u64) {
            let caller = self.env().caller();

            // Verificar que la elección exista
            if id_eleccion as usize >= self.elecciones.len() {
                panic!("La eleccion no existe");
            }

            // Obtener el usuario completo del vector de usuarios
            let usuario = self.usuarios.iter().find(|&u| u.id == caller).cloned();

            // Verificar que el usuario exista y sea un votante
            let usuario = match usuario {
                Some(u) if u.rol == RolUsuario::Votante => u,
                _ => panic!("El usuario no es un votante"),
            };

            // Verificar que el usuario no esté registrado como votante en la elección
            if self.elecciones[id_eleccion as usize]
                .votantes
                .iter()
                .any(|v| v.id == usuario.id)
            {
                panic!("El usuario ya está registrado como votante en esta elección");
            }

            // Registrar al usuario completo como votante en la elección
            self.elecciones[id_eleccion as usize].votantes.push(usuario);
        }

        #[ink(message)]
        /// Funcion para registrar un candidato en una eleccion
        pub fn registrar_candidato_en_eleccion(&mut self, id_eleccion: u64) {
            let caller = self.env().caller();

            // Verificar que la elección exista
            if id_eleccion as usize >= self.elecciones.len() {
                panic!("La eleccion no existe");
            }

            // Verificar que el usuario actual sea un candidato
            let mut usuario_es_candidato = false;
            for usuario in self.usuarios.iter() {
                if usuario.id == caller && usuario.rol == RolUsuario::Candidato {
                    usuario_es_candidato = true;
                    break;
                }
            }

            // Si el usuario no es un candidato, lanzar un error
            if !usuario_es_candidato {
                panic!("El usuario no es un candidato");
            }

            // Verificar que el usuario no esté registrado como candidato en la elección
            if self.elecciones[id_eleccion as usize]
                .candidatos
                .contains_key(&caller)
            {
                panic!("El usuario ya está registrado como candidato en esta elección");
            }

            // Registrar al usuario como candidato en la elección
            self.elecciones[id_eleccion as usize]
                .candidatos
                .insert(caller, 0);
        }

        //----------------------Funciones de votacion---------------------------------------------------------

        pub fn votar(&mut self) {
            let caller = self.env().caller();
            /*Cosas a verificar
            -Que el votante este registrado en la eleccion
            -Que la eleccion este activa
            -Que el votante no haya votado ya
            -Que el candidato exista
            -Que el votante sea un votante
            -Que el votante no sea el admin
            -Que las elecciones no esten cerradas
            -Que las elecciones esten abiertas

            podria hacer una funcion que verifique todas estas cosas y que devuelva un bool y un mensaje de error asi no es tanto quilombo
            */
        }
        //----------------------Funciones de conteo y resultados---------------------------------------------------------
        //Hacerlo despues de que termine la eleccion
        pub fn contar_votos() {
            //TODO
        }
        pub fn mostrar_resultados() {
            //TODO
        }
    }
    //----------------------Funciones de verificacion --------------------------------------------------------
    pub fn verificar_votante() {
        //TODO
    }
    pub fn verificar_candidato() {
        //TODO
    }
    //----------------------Structs de admin y eleccion---------------------------------------------------------
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    #[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
    #[derive(Debug, Clone, PartialEq, Eq)]
    struct Admin {
        id: AccountId,
        nombre: String,
        email: String,
        password: String,
    }
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    #[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
    #[derive(Debug, Clone, PartialEq, Eq)]
    struct Eleccion {
        id: u64,
        cargo: String,
        fecha_inicio: u64, // u64 por ser un timestamp
        fecha_fin: u64,
        candidatos: BTreeMap<AccountId, u64>,
        votantes: Vec<Usuario>,
        estado: bool,
    }

    //----------------------Structs de usuarios---------------------------------------------------------
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    #[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
    #[derive(Debug, Clone, PartialEq, Eq)]
    struct Candidato {
        afiliacion: String,
    }
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    #[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
    #[derive(Debug, Clone, PartialEq, Eq)]
    struct Votante {
        nose: String,
    }

    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    #[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub enum RolUsuario {
        Candidato,
        Votante,
    }

    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    #[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct Usuario {
        id: AccountId,
        nombre: String,
        email: String,
        rol: RolUsuario,
    }
}
