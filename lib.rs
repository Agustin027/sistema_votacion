/*Cosas a consultar
- use chrono::{TimeZone, Utc}; // cuando quiero usar chrono me revienta todo :(
-El votante puede votar en varias elecciones? o solo en una? cuando deberia registrarse en una eleccion? cuando se registra en el sistema? o con una funcion aparte?
-Hay alguna manera mejor de verificar todas las condiciones de los usuarios? (ejemplo: que no se registre el admin como usuario normal) por que lo estoy haciendo con ifs y panics y queda feo
-Como deberia ser el registro de votos? deberia ser un struct aparte? o deberia ser un metodo de la eleccion?
//lo que yo queria hacer con el chrono era recibir una fecha en formato dd/mm/yyyy y convertirla a timestamp para poder compararla con el timestamp actual y asi saber si la eleccion esta activa o no
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
        /// Constructor
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
        pub fn get_id_admin(&self) -> AccountId {
            self.admin.id
        }
        #[ink(message)]
        pub fn set_admin(&mut self, nombre: String, email: String, password: String) {
            self.admin = Admin {
                id: self.env().caller(),
                nombre,
                email,
                password,
            };
        }
        #[ink(message)]
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
        pub fn cambiar_estado_eleccion(&mut self, id: u64) {
            self.elecciones[id as usize].estado = !self.elecciones[id as usize].estado;
        }
        //----------------------Funciones de registro---------------------------------------------------------
        #[ink(message)]
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

        pub fn registrar_votante_en_eleccion(&mut self, id_eleccion: u64) {}
        pub fn registrar_candidato_en_eleccion(&mut self, id_eleccion: u64) {}

        //----------------------Funciones de votacion---------------------------------------------------------

        pub fn votar() {
            //TODO
        }
        pub fn contar_votos() {
            //TODO
        }
        pub fn mostrar_resultados() {
            //TODO
        }
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
        candidatos: BTreeMap<u64, Usuario>,
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
