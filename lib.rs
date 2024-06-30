/*To do
- HACER TESTS
- meter manejo de errores
 todo lo del caller lo hago desde metodos publicos, si paso de ahi lo testeo en privado, los end to end los hago con el caller
 terminar este contrato con 85% de cobertura

 */

/*Cosas a consultar


*/
#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod sistema_votacion {
    use chrono::NaiveDate;
    use chrono::NaiveDateTime;
    use core::panic;
    use ink::env;
    use ink::prelude::collections::BTreeMap;
    use ink::prelude::string::String;
    use ink::prelude::vec::Vec;
    use ink_env::account_id;
    use ink_env::caller;
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
        pub fn new() -> Self {
            SistemaVotacion::new_priv()
        }

        fn new_priv() -> Self {
            let caller = Self::env().caller();
            let admin = Admin {
                id: caller,
                nombre: String::from("admin"),
                email: String::from("mail.com"),
                password: String::from("admin"),
            };

            Self {
                admin,
                elecciones: Vec::new(),
                usuarios: Vec::new(),
            }
        }
        #[ink(message)]
        /// funcion para crear una nueva eleccion
        pub fn crear_eleccion(&mut self, cargo: String, fecha_ini: Fecha, fecha_f: Fecha) {
            self.crear_eleccion_priv(cargo, fecha_ini, fecha_f);
        }

        fn crear_eleccion_priv(&mut self, cargo: String, fecha_ini: Fecha, fecha_f: Fecha) {
            let fecha_inicio = fecha_ini.to_timestamp();
            let fecha_fin = fecha_f.to_timestamp();
            let eleccion = Eleccion {
                id: self.elecciones.len() as u64,
                cargo,
                fecha_inicio,
                fecha_fin,
                candidatos: BTreeMap::new(),
                votantes: Vec::new(),
                votantes_que_votaron: Vec::new(),
                estado: false,
            };
            self.elecciones.push(eleccion);
        }

        #[ink(message)]
        pub fn get_fecha_fin(&self, id: u64) -> u64 {
            SistemaVotacion::get_fecha_fin_priv(self, id)
        }

        fn get_fecha_fin_priv(&self, id: u64) -> u64 {
            self.elecciones[id as usize].fecha_fin
        }

        #[ink(message)]
        pub fn get_fecha_inicio(&self, id: u64) -> u64 {
            SistemaVotacion::get_fecha_inicio_priv(self, id)
        }

        fn get_fecha_inicio_priv(&self, id: u64) -> u64 {
            //////////////////////
            self.elecciones[id as usize].fecha_inicio
        }

        #[ink(message)]
        /// Funcion para obtener el id del admin
        pub fn get_id_admin(&self) -> AccountId {
            SistemaVotacion::get_id_admin_priv(&self)
        }

        fn get_id_admin_priv(&self) -> AccountId {
            ///////////////////
            self.admin.id
        }

        #[ink(message)]
        /// Funcion para settear un nuevo admin
        pub fn set_admin(
            &mut self,
            nombre: String,
            email: String,
            password: String,
            nuevo_admin: AccountId,
        ) {
            SistemaVotacion::set_admin_priv(self, nombre, email, password, nuevo_admin)
        }

        fn set_admin_priv(
            &mut self,
            nombre: String,
            email: String,
            password: String,
            nuevo_admin: AccountId,
        ) {
            let caller = self.env().caller();
            if caller != self.admin.id {
                panic!("No tiene permisos para realizar esta accion");
            }
            self.admin = Admin {
                id: nuevo_admin,
                nombre,
                email,
                password,
            };
        }

        #[ink(message)]
        /// Funcion para cambiar el estado de una eleccion
        pub fn cambiar_estado_eleccion(&mut self, id: u64) {
            self.elecciones[id as usize].estado = !self.elecciones[id as usize].estado;
        }

        #[ink(message)]
        /// esto lo tengo que borrar despues, es solo para probar que se crean las elecciones
        pub fn get_elecciones(&self) -> Vec<Eleccion> {
            self.elecciones.clone()
        }

        //----------------------Funciones de registro---------------------------------------------------------

        #[ink(message)]
        /// Funcion para registrar un usuario en el sistema con los datos ingresados,distingue entre votante y candidato y verifica que no se registre el admin como usuario normal
        pub fn registrar_usuario(&mut self, nombre: String, email: String, rol: RolUsuario) {
            self.registrar_usuario_priv(nombre, email, rol);
        }

        fn registrar_usuario_priv(&mut self, nombre: String, email: String, rol: RolUsuario) {
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
            self.registrar_votante_en_eleccion_priv(id_eleccion);
        }
        pub fn registrar_votante_en_eleccion_priv(&mut self, id_eleccion: u64) {
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
            self.registrar_candidato_en_eleccion_priv(id_eleccion);
        }
        fn registrar_candidato_en_eleccion_priv(&mut self, id_eleccion: u64) {
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
        #[ink(message)]
        pub fn votar(&mut self, id_eleccion: u64, id_candidato: AccountId) {
            self.votar_priv(id_eleccion, id_candidato);
        }
        fn votar_priv(&mut self, id_eleccion: u64, id_candidato: AccountId) {
            let caller = self.env().caller();

            let votante = self.usuarios.iter().find(|&u| u.id == caller).cloned();
            let votante = match votante {
                Some(u) if u.rol == RolUsuario::Votante => u,
                _ => panic!("El usuario no es un votante"),
            };

            if caller == self.admin.id {
                panic!("El admin no puede votar");
            }

            self.elecciones[id_eleccion as usize].votar_en_eleccion(id_candidato, votante);
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
    //----------------------Funciones de eleccion---------------------------------------------------------
    impl Eleccion {
        fn votar_en_eleccion(&mut self, id_candidato: AccountId, votante: Usuario) {
            // Verificar que la elección esté activa, esto despues lo voy a hacer con una funcion que verifique todas las cosas
            if !self.estado {
                panic!("La elección no está activa");
            }

            // Incrementar el conteo de votos del candidato
            if let Some(votos) = self.candidatos.get_mut(&id_candidato) {
                // Verificar que el votante no haya votado ya
                if self.votantes_que_votaron.contains(&votante) {
                    panic!("El votante ya ha votado");
                }
                *votos = votos.checked_add(1).expect("Vote count overflow");
                self.votantes_que_votaron.push(votante);
            } else {
                panic!("El candidato no existe");
            }
        }
    }
    //----------------------Funciones de verificacion --------------------------------------------------------

    //----------------------Funciones de fecha---------------------------------------------------------

    impl Fecha {
        pub fn to_timestamp(&self) -> u64 {
            let date = chrono::NaiveDate::from_ymd_opt(self.anio, self.mes, self.dias)
                .expect("Fecha inválida"); // este lo deberia cambiar ??, una fecha invalida no deberia ser un panic sino un error que se maneje en el contrato y se devuelva al usuario

            let datetime = date.and_hms(0, 0, 0);

            datetime.timestamp() as u64
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
    pub struct Eleccion {
        id: u64,
        cargo: String,
        fecha_inicio: u64, // u64 por ser un timestamp
        fecha_fin: u64,
        candidatos: BTreeMap<AccountId, u64>,
        votantes: Vec<Usuario>,
        votantes_que_votaron: Vec<Usuario>,
        //votantes_votaron: BTreeMap<AccountId, bool>, aca pense en almacenar los votantes en un btreemap con el AccountId y un bool que indique si voto o no, pero despues iba a ser un quilombo el reporte
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

    //----------------------Structs de fecha---------------------------------------------------------
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    #[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct Fecha {
        dias: u32,
        mes: u32,
        anio: i32,
    }
    //----------------------Tests---------------------------------------------------------
    /*#[cfg(test)]
    mod tests {
        use super::*;
        use ink_env::{call, test};
        #[ink::test]
        fn test_crear_sistema() {
            let accounts = env::test::default_accounts::<ink_env::DefaultEnvironment>();
            let caller = accounts.alice;
            ink_env::test::set_caller::<ink_env::DefaultEnvironment>(caller);
            let mut sistema = SistemaVotacion::new(
                String::from("cargo"),
                Fecha {
                    dias: 1,
                    mes: 1,
                    anio: 2021,
                },
                Fecha {
                    dias: 1,
                    mes: 1,
                    anio: 2021,
                },
            );
            println!("{:?}", sistema.get_id_admin());
            assert_eq!(sistema.get_id_admin(), caller);
        }

        #[ink::test]
        fn test_crear_eleccion() {
            let accounts = env::test::default_accounts::<ink_env::DefaultEnvironment>();
            let caller = accounts.alice;
            ink_env::test::set_caller::<ink_env::DefaultEnvironment>(caller);
            let mut sistema = SistemaVotacion::new(
                String::from("cargo"),
                Fecha {
                    dias: 1,
                    mes: 1,
                    anio: 2021,
                },
                Fecha {
                    dias: 1,
                    mes: 1,
                    anio: 2021,
                },
            );
            sistema.crear_eleccion(
                String::from("cargo"),
                Fecha {
                    dias: 1,
                    mes: 1,
                    anio: 2021,
                },
                Fecha {
                    dias: 1,
                    mes: 1,
                    anio: 2021,
                },
            );
            assert_eq!(sistema.get_id_admin(), caller);
            //cambio de alice a bob
            sistema.set_admin(
                "Agustin".to_string(),
                "Mail".to_string(),
                "*****".to_string(),
                accounts.bob,
            );
            let elecciones = sistema.get_elecciones();
            assert_eq!(elecciones.len(), 2);
            assert_eq!(sistema.get_id_admin(), accounts.bob);
        }

        #[ink::test]
        fn test_registrar_usuario() {
            let accounts = env::test::default_accounts::<ink_env::DefaultEnvironment>();
            let mut caller = accounts.alice;
            ink_env::test::set_caller::<ink_env::DefaultEnvironment>(caller);

            let mut sistema = SistemaVotacion::new(
                String::from("cargo"),
                Fecha {
                    dias: 1,
                    mes: 1,
                    anio: 2021,
                },
                Fecha {
                    dias: 1,
                    mes: 1,
                    anio: 2021,
                },
            );
            caller = accounts.bob;
            ink_env::test::set_caller::<ink_env::DefaultEnvironment>(caller);
            sistema.registrar_usuario(
                String::from("nombre"),
                String::from("mail"),
                RolUsuario::Votante,
            );
            let usuarios = sistema.usuarios;
            assert_eq!(usuarios.len(), 1);
        }
        #[ink::test]
        fn registrar_candidato() {
            let accounts = env::test::default_accounts::<ink_env::DefaultEnvironment>();
            let mut caller = accounts.alice;
            ink_env::test::set_caller::<ink_env::DefaultEnvironment>(caller);

            let mut sistema = SistemaVotacion::new(
                String::from("cargo"),
                Fecha {
                    dias: 1,
                    mes: 1,
                    anio: 2024,
                },
                Fecha {
                    dias: 1,
                    mes: 1,
                    anio: 2024,
                },
            );
            caller = accounts.bob;
            ink_env::test::set_caller::<ink_env::DefaultEnvironment>(caller);
            sistema.registrar_usuario(
                String::from("nombre"),
                String::from("mail"),
                RolUsuario::Candidato,
            );

            sistema.registrar_candidato_en_eleccion(0);
            assert_eq!(sistema.usuarios.len(), 1);
            assert_eq!(sistema.elecciones[0].candidatos.len(), 1);
        }
        #[ink::test]
        fn registrar_votante() {
            let accounts = env::test::default_accounts::<ink_env::DefaultEnvironment>();
            let mut caller = accounts.alice;
            ink_env::test::set_caller::<ink_env::DefaultEnvironment>(caller);

            let mut sistema = SistemaVotacion::new(
                String::from("cargo"),
                Fecha {
                    dias: 1,
                    mes: 1,
                    anio: 2024,
                },
                Fecha {
                    dias: 1,
                    mes: 1,
                    anio: 2024,
                },
            );
            caller = accounts.bob;
            ink_env::test::set_caller::<ink_env::DefaultEnvironment>(caller);
            sistema.registrar_usuario(
                String::from("nombre"),
                String::from("mail"),
                RolUsuario::Votante,
            );

            sistema.registrar_votante_en_eleccion(0);
            assert_eq!(sistema.usuarios.len(), 1);
            assert_eq!(sistema.elecciones[0].votantes.len(), 1);
        }
        #[ink::test]
        fn votar() {}
    } */
}
