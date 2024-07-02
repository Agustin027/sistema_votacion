/*To do
- HACER TESTS
- hacer reportes en otro contrato
 */

#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod sistema_votacion {
    use chrono::NaiveDate;
    use chrono::NaiveDateTime;
    use core::fmt;
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
            let aaaa = 0;
            let caller = Self::env().caller();
            SistemaVotacion::new_priv(caller)
        }

        fn new_priv(caller: AccountId) -> Self {
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
            let caller = self.env().caller();
            self.crear_eleccion_priv(caller, cargo, fecha_ini, fecha_f);
        }

        fn crear_eleccion_priv(
            &mut self,
            caller: AccountId,
            cargo: String,
            fecha_ini: Fecha,
            fecha_f: Fecha,
        ) -> Result<(), Error> {
            if self.admin.id != caller {
                return Err(Error::PermisoDenegado);
            }
            let fecha_inicio = fecha_ini.to_timestamp()?;
            let fecha_fin = fecha_f.to_timestamp()?;
            let eleccion = Eleccion {
                id: self.elecciones.len() as u64,
                cargo,
                fecha_inicio,
                fecha_fin,
                candidatos: BTreeMap::new(),
                votantes: Vec::new(),
                votantes_que_votaron: Vec::new(),
            };
            self.elecciones.push(eleccion);
            Ok(())
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
            let caller = self.env().caller();
            self.set_admin_priv(caller, nombre, email, password, nuevo_admin);
        }

        fn set_admin_priv(
            &mut self,
            caller: AccountId,
            nombre: String,
            email: String,
            password: String,
            nuevo_admin: AccountId,
        ) -> Result<(), Error> {
            if caller != self.admin.id {
                return Err(Error::PermisoDenegado);
            }
            self.admin = Admin {
                id: nuevo_admin,
                nombre,
                email,
                password,
            };
            Ok(())
        }

        fn eleccion_activa(&self, id: u64) -> Result<bool, Error> {
            let fecha_actual = self.env().block_timestamp();

            if id as usize >= self.elecciones.len() {
                return Err(Error::EleccionNoExiste);
            }

            let fecha_inicio = self.elecciones[id as usize].fecha_inicio;
            let fecha_fin = self.elecciones[id as usize].fecha_fin;

            Ok(fecha_actual >= fecha_inicio && fecha_actual <= fecha_fin)
        }

        fn eleccion_cerrada(&self, id: u64) -> Result<bool, Error> {
            let fecha_actual = self.env().block_timestamp();
            if id as usize >= self.elecciones.len() {
                return Err(Error::EleccionNoExiste);
            }

            let fecha_fin = self.elecciones[id as usize].fecha_fin;

            Ok(fecha_actual > fecha_fin)
        }

        fn eleccion_no_abierta(&self, id: u64) -> Result<bool, Error> {
            let fecha_actual = self.env().block_timestamp();
            if id as usize >= self.elecciones.len() {
                return Err(Error::EleccionNoExiste);
            }

            let fecha_inicio = self.elecciones[id as usize].fecha_inicio;

            Ok(fecha_actual < fecha_inicio)
        }

        /*

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
        pub fn mostrar_validaciones_fecha(&self, id: u64) -> Result<(bool, bool, bool), Error> {
            Ok((
                self.eleccion_activa(id)?,
                self.eleccion_cerrada(id)?,
                self.eleccion_no_abierta(id)?,
            ))
        }

        #[ink(message)]
        pub fn mostrar_fechas(&self, id: u64) -> (u64, u64, u64) {
            (
                self.elecciones[id as usize].fecha_inicio,
                self.elecciones[id as usize].fecha_fin,
                self.env().block_timestamp(),
            )
        }

        #[ink(message)]
        /// esto lo tengo que borrar despues, es solo para probar que se crean las elecciones
        pub fn get_elecciones(&self) -> Vec<Eleccion> {
            self.elecciones.clone()
        }

        //esto es para probar que funcionan las funciones de verificacion de fecha
        #[ink(message)]
        pub fn cambiar_fechas_eleccion(
            &mut self,
            id: u64,
            fecha_ini: Fecha,
            fecha_f: Fecha,
        ) -> Result<(), Error> {
            if self.admin.id != self.env().caller() {
                return Err(Error::PermisoDenegado);
            }
            self.elecciones[id as usize].fecha_inicio = fecha_ini.to_timestamp()?;
            self.elecciones[id as usize].fecha_fin = fecha_f.to_timestamp()?;
            Ok(())
        }
        */
        //----------------------Funciones de registro---------------------------------------------------------

        #[ink(message)]
        /// Funcion para registrar un usuario en el sistema con los datos ingresados,distingue entre votante y candidato y verifica que no se registre el admin como usuario normal
        pub fn registrar_usuario(&mut self, nombre: String, email: String, rol: RolUsuario) {
            let caller = self.env().caller();
            self.registrar_usuario_priv(caller, nombre, email, rol);
        }

        fn registrar_usuario_priv(
            &mut self,
            caller: AccountId,
            nombre: String,
            email: String,
            rol: RolUsuario,
        ) -> Result<(), Error> {
            let usuario = Usuario {
                id: caller,
                nombre,
                email,
                rol,
            };
            if self.usuarios.iter().any(|u| u.id == usuario.id) || usuario.id == self.admin.id {
                if usuario.id == self.admin.id {
                    return Err(Error::AdminNoPuedeRegistrarse);
                } else {
                    return Err(Error::UsuarioYaRegistrado);
                }
            }
            self.usuarios.push(usuario);
            Ok(())
        }
        #[ink(message)]
        /// Funcion para registrar un votante en una eleccion
        pub fn registrar_votante_en_eleccion(&mut self, id_eleccion: u64) {
            let caller = self.env().caller();
            self.registrar_votante_en_eleccion_priv(caller, id_eleccion);
        }
        pub fn registrar_votante_en_eleccion_priv(
            &mut self,
            caller: AccountId,
            id_eleccion: u64,
        ) -> Result<(), Error> {
            if !self.eleccion_no_abierta(id_eleccion)? {
                return Err(Error::EleccionAbierta);
            }

            // Verificar que la elección exista
            if id_eleccion as usize >= self.elecciones.len() {
                return Err(Error::EleccionNoExiste);
            }

            // Obtener el usuario completo del vector de usuarios
            let usuario = self.usuarios.iter().find(|&u| u.id == caller).cloned();

            // Verificar que el usuario exista y sea un votante
            let usuario = match usuario {
                Some(u) if u.rol == RolUsuario::Votante => u,
                _ => return Err(Error::UsuarioNoVotante),
            };

            // Verificar que el usuario no esté registrado como votante en la elección
            if self.elecciones[id_eleccion as usize]
                .votantes
                .iter()
                .any(|v| v.id == usuario.id)
            {
                return Err(Error::UsuarioYaRegistrado);
            }

            // Registrar al usuario completo como votante en la elección
            self.elecciones[id_eleccion as usize].votantes.push(usuario);
            Ok(())
        }

        #[ink(message)]
        /// Funcion para registrar un candidato en una eleccion
        pub fn registrar_candidato_en_eleccion(&mut self, id_eleccion: u64) {
            let caller = self.env().caller();
            self.registrar_candidato_en_eleccion_priv(caller, id_eleccion);
        }
        fn registrar_candidato_en_eleccion_priv(
            &mut self,
            caller: AccountId,
            id_eleccion: u64,
        ) -> Result<(), Error> {
            if !self.eleccion_no_abierta(id_eleccion)? {
                return Err(Error::EleccionAbierta);
            }

            // Verificar que la elección exista
            if id_eleccion as usize >= self.elecciones.len() {
                return Err(Error::EleccionNoExiste);
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
                return Err(Error::UsuarioNoCandidato);
            }

            // Verificar que el usuario no esté registrado como candidato en la elección
            if self.elecciones[id_eleccion as usize]
                .candidatos
                .contains_key(&caller)
            {
                return Err(Error::UsuarioYaRegistrado);
            }

            // Registrar al usuario como candidato en la elección
            self.elecciones[id_eleccion as usize]
                .candidatos
                .insert(caller, 0);
            Ok(())
        }

        //----------------------Funciones de votacion---------------------------------------------------------
        #[ink(message)]
        pub fn votar(&mut self, id_eleccion: u64, id_candidato: AccountId) {
            let caller = self.env().caller();
            self.votar_priv(caller, id_eleccion, id_candidato);
        }
        fn votar_priv(
            &mut self,
            caller: AccountId,
            id_eleccion: u64,
            id_candidato: AccountId,
        ) -> Result<(), Error> {
            if !self.eleccion_activa(id_eleccion)? {
                return Err(Error::EleccionNoActiva);
            }

            let votante = self.usuarios.iter().find(|&u| u.id == caller).cloned();
            let votante = match votante {
                Some(u) if u.rol == RolUsuario::Votante => u,
                _ => return Err(Error::UsuarioNoVotante),
            };

            if caller == self.admin.id {
                return Err(Error::AdminNoPuedeVotar);
            }

            self.elecciones[id_eleccion as usize].votar_en_eleccion(id_candidato, votante);
            Ok(())
        }
        //----------------------Funciones de conteo y resultados---------------------------------------------------------

        #[ink(message)]
        /// Funcion para mostrar los resultados de una eleccion
        pub fn mostrar_resultados(
            &self,
            id_eleccion: u64,
        ) -> Result<BTreeMap<AccountId, u64>, Error> {
            if !self.eleccion_cerrada(id_eleccion)? {
                return Err(Error::EleccionAbierta);
            }

            let resultados = self
                .elecciones
                .get(id_eleccion as usize)
                .ok_or(Error::EleccionNoExiste)?
                .candidatos
                .clone();
            Ok(resultados)
        }
    }
    //----------------------Funciones de eleccion---------------------------------------------------------
    impl Eleccion {
        fn votar_en_eleccion(
            &mut self,
            id_candidato: AccountId,
            votante: Usuario,
        ) -> Result<(), Error> {
            // Incrementar el conteo de votos del candidato
            if let Some(votos) = self.candidatos.get_mut(&id_candidato) {
                if self.votantes_que_votaron.contains(&votante) {
                    return Err(Error::UsuarioYaRegistrado);
                }

                // Intentar incrementar el conteo de votos, manejando el posible overflow
                *votos = votos.checked_add(1).ok_or(Error::Overflow)?;

                self.votantes_que_votaron.push(votante);
            } else {
                return Err(Error::CandidatoNoExiste);
            }
            Ok(())
        }
    }

    //----------------------Funciones de fecha---------------------------------------------------------

    impl Fecha {
        pub fn to_timestamp(&self) -> Result<u64, Error> {
            let date = NaiveDate::from_ymd_opt(self.anio, self.mes, self.dias)
                .ok_or(Error::FechaInvalida)?; // Manejo del error para una fecha inválida

            // Arranca a las 00:00:00 del día
            let datetime = date.and_hms(0, 0, 0);

            let timestamp_secs = datetime.timestamp() as u64;

            // Pasar a milisegundos, manejando posible overflow
            timestamp_secs.checked_mul(1000).ok_or(Error::Overflow)
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
        fecha_inicio: u64,
        fecha_fin: u64,
        candidatos: BTreeMap<AccountId, u64>,
        votantes: Vec<Usuario>,
        votantes_que_votaron: Vec<Usuario>,
    }

    //----------------------Structs de usuarios---------------------------------------------------------

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
    //----------------------Tests y errores---------------------------------------------------------
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    #[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub enum Error {
        PermisoDenegado,
        EleccionNoExiste,
        EleccionAbierta,
        EleccionNoActiva,
        UsuarioNoVotante,
        UsuarioYaRegistrado,
        AdminNoPuedeRegistrarse,
        AdminNoPuedeVotar,
        CandidatoNoExiste,
        UsuarioNoCandidato,
        FechaInvalida,
        Overflow,
    }

    impl core::fmt::Display for Error {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            match *self {
                Error::PermisoDenegado => write!(f, "Permiso denegado"),
                Error::EleccionNoExiste => write!(f, "La elección no existe"),
                Error::EleccionAbierta => write!(f, "La elección está abierta"),
                Error::EleccionNoActiva => write!(f, "La elección no está activa"),
                Error::UsuarioNoVotante => write!(f, "El usuario no es votante"),
                Error::UsuarioYaRegistrado => write!(f, "El usuario ya está registrado"),
                Error::AdminNoPuedeRegistrarse => write!(f, "El admin no puede registrarse"),
                Error::AdminNoPuedeVotar => write!(f, "El admin no puede votar"),
                Error::CandidatoNoExiste => write!(f, "El candidato no existe"),
                Error::UsuarioNoCandidato => write!(f, "El usuario no es candidato"),
                Error::FechaInvalida => write!(f, "Fecha inválida"),
                Error::Overflow => write!(f, "Overflow"),
            }
        }
    }

    #[cfg(test)]
    mod test {
        use super::*;
        use ink::primitives::AccountId;
        use ink_env::test::set_block_timestamp;
        #[test]
        fn test_fecha_to_timestamp() {
            let mut fecha = Fecha {
                dias: 1,
                mes: 1,
                anio: 2021,
            };
            assert_eq!(fecha.to_timestamp().unwrap(), 1609459200000);
            fecha.dias = 32;
            assert_eq!(fecha.to_timestamp().unwrap_err(), Error::FechaInvalida);
        }
        #[test]
        fn test_set_admin_priv() {
            let mut sistema = SistemaVotacion::new_priv(AccountId::from([0x01; 32]));
            assert!(sistema
                .set_admin_priv(
                    AccountId::from([0x01; 32]),
                    "Agustin".to_string(),
                    " ".to_string(),
                    " ".to_string(),
                    AccountId::from([0x02; 32])
                )
                .is_ok());
            assert!(sistema
                .set_admin_priv(
                    AccountId::from([0x01; 32]),
                    "Agustin".to_string(),
                    " ".to_string(),
                    " ".to_string(),
                    AccountId::from([0x03; 32])
                )
                .is_err());
        }

        #[ink::test]
        fn test_eleccion_activa() {
            let mut sistema = SistemaVotacion::new_priv(AccountId::from([0x01; 32]));
            sistema.crear_eleccion_priv(
                AccountId::from([0x01; 32]),
                "cargo".to_string(),
                Fecha {
                    dias: 1,
                    mes: 7,
                    anio: 2024,
                },
                Fecha {
                    dias: 3,
                    mes: 7,
                    anio: 2024,
                },
            );

            ink_env::test::set_block_timestamp::<ink_env::DefaultEnvironment>(1719878400000);

            assert!(sistema.eleccion_activa(0).is_ok());
            assert_eq!(sistema.eleccion_activa(0).unwrap(), true);

            ink_env::test::set_block_timestamp::<ink_env::DefaultEnvironment>(1800000000000);

            assert!(sistema.eleccion_activa(0).is_ok());
            assert_eq!(sistema.eleccion_activa(0).unwrap(), false);
            assert!(sistema.eleccion_activa(1).is_err());
        }
        #[ink::test]
        fn test_eleccion_cerrada() {
            let mut sistema = SistemaVotacion::new_priv(AccountId::from([0x01; 32]));
            sistema.crear_eleccion_priv(
                AccountId::from([0x01; 32]),
                "cargo".to_string(),
                Fecha {
                    dias: 1,
                    mes: 1,
                    anio: 2023,
                },
                Fecha {
                    dias: 1,
                    mes: 2,
                    anio: 2023,
                },
            );

            ink_env::test::set_block_timestamp::<ink_env::DefaultEnvironment>(1719878400000);

            assert!(sistema.eleccion_cerrada(0).is_ok());
            assert_eq!(sistema.eleccion_cerrada(0).unwrap(), true);

            ink_env::test::set_block_timestamp::<ink_env::DefaultEnvironment>(17198784000);

            assert!(sistema.eleccion_cerrada(0).is_ok());
            assert_eq!(sistema.eleccion_cerrada(0).unwrap(), false);
            assert!(sistema.eleccion_cerrada(1).is_err());
        }
        #[ink::test]
        fn test_eleccion_no_abierta() {
            let mut sistema = SistemaVotacion::new_priv(AccountId::from([0x01; 32]));
            sistema.crear_eleccion_priv(
                AccountId::from([0x01; 32]),
                "cargo".to_string(),
                Fecha {
                    dias: 1,
                    mes: 1,
                    anio: 2023,
                },
                Fecha {
                    dias: 1,
                    mes: 2,
                    anio: 2023,
                },
            );

            ink_env::test::set_block_timestamp::<ink_env::DefaultEnvironment>(1719878400000);

            assert!(sistema.eleccion_no_abierta(0).is_ok());
            assert_eq!(sistema.eleccion_no_abierta(0).unwrap(), false);

            ink_env::test::set_block_timestamp::<ink_env::DefaultEnvironment>(17198784000);

            assert!(sistema.eleccion_no_abierta(0).is_ok());
            assert_eq!(sistema.eleccion_no_abierta(0).unwrap(), true);
            assert!(sistema.eleccion_no_abierta(1).is_err());
        }
        #[ink::test]
        fn test_crear_eleccion_priv() {
            let mut sistema = SistemaVotacion::new_priv(AccountId::from([0x01; 32]));
            assert!(sistema
                .crear_eleccion_priv(
                    AccountId::from([0x01; 32]),
                    "cargo".to_string(),
                    Fecha {
                        dias: 1,
                        mes: 1,
                        anio: 2021,
                    },
                    Fecha {
                        dias: 1,
                        mes: 1,
                        anio: 2022,
                    }
                )
                .is_ok());
            assert!(sistema
                .crear_eleccion_priv(
                    AccountId::from([0x02; 32]),
                    "cargo".to_string(),
                    Fecha {
                        dias: 1,
                        mes: 1,
                        anio: 2021,
                    },
                    Fecha {
                        dias: 1,
                        mes: 1,
                        anio: 2022,
                    }
                )
                .is_err());
        }

        #[ink::test]
        fn test_registar_usuario_priv() {
            let mut sistema = SistemaVotacion::new_priv(AccountId::from([0x01; 32]));

            assert!(sistema
                .registrar_usuario_priv(
                    AccountId::from([0x02; 32]),
                    "Agustin".to_string(),
                    " ".to_string(),
                    RolUsuario::Votante
                )
                .is_ok());
            assert!(sistema
                .registrar_usuario_priv(
                    AccountId::from([0x02; 32]),
                    "Agustin".to_string(),
                    " ".to_string(),
                    RolUsuario::Votante
                )
                .is_err());
            assert!(sistema
                .registrar_usuario_priv(
                    AccountId::from([0x01; 32]),
                    "Agustin".to_string(),
                    " ".to_string(),
                    RolUsuario::Votante
                )
                .is_err());
        }
        #[ink::test]
        fn test_registrar_votante_en_eleccion_priv() {
            let mut sistema = SistemaVotacion::new_priv(AccountId::from([0x01; 32]));
            sistema.registrar_usuario_priv(
                AccountId::from([0x02; 32]),
                "Agustin".to_string(),
                " ".to_string(),
                RolUsuario::Votante,
            );
            sistema
                .crear_eleccion_priv(
                    AccountId::from([0x01; 32]),
                    "cargo".to_string(),
                    Fecha {
                        dias: 1,
                        mes: 1,
                        anio: 2021,
                    },
                    Fecha {
                        dias: 1,
                        mes: 1,
                        anio: 2022,
                    },
                )
                .unwrap();

            assert!(sistema
                .registrar_votante_en_eleccion_priv(AccountId::from([0x02; 32]), 0)
                .is_ok());
            assert!(sistema
                .registrar_votante_en_eleccion_priv(AccountId::from([0x02; 32]), 0)
                .is_err());
            assert!(sistema
                .registrar_votante_en_eleccion_priv(AccountId::from([0x01; 32]), 0)
                .is_err());
            assert!(sistema
                .registrar_votante_en_eleccion_priv(AccountId::from([0x02; 32]), 1)
                .is_err(),);
        }
        #[ink::test]
        fn test_registrar_candidato_en_eleccion_priv() {
            let mut sistema = SistemaVotacion::new_priv(AccountId::from([0x01; 32]));
            sistema.registrar_usuario_priv(
                AccountId::from([0x02; 32]),
                "Agustin".to_string(),
                " ".to_string(),
                RolUsuario::Candidato,
            );
            sistema
                .crear_eleccion_priv(
                    AccountId::from([0x01; 32]),
                    "cargo".to_string(),
                    Fecha {
                        dias: 1,
                        mes: 1,
                        anio: 2021,
                    },
                    Fecha {
                        dias: 1,
                        mes: 1,
                        anio: 2022,
                    },
                )
                .unwrap();

            assert!(sistema
                .registrar_candidato_en_eleccion_priv(AccountId::from([0x02; 32]), 0)
                .is_ok());
            assert!(sistema
                .registrar_candidato_en_eleccion_priv(AccountId::from([0x02; 32]), 0)
                .is_err());
            assert!(sistema
                .registrar_candidato_en_eleccion_priv(AccountId::from([0x01; 32]), 0)
                .is_err());
            assert!(sistema
                .registrar_candidato_en_eleccion_priv(AccountId::from([0x02; 32]), 1)
                .is_err());
        }
        #[ink::test]
        fn test_votar_priv() {
            let mut sistema = SistemaVotacion::new_priv(AccountId::from([0x01; 32]));
            // Crear una elección
            sistema.crear_eleccion_priv(
                AccountId::from([0x01; 32]),
                "cargo".to_string(),
                Fecha {
                    dias: 1,
                    mes: 1,
                    anio: 2024,
                },
                Fecha {
                    dias: 1,
                    mes: 1,
                    anio: 2025,
                },
            );
            // registrar usario candidato
            sistema.registrar_usuario_priv(
                AccountId::from([0x02; 32]),
                "Agustin".to_string(),
                " ".to_string(),
                RolUsuario::Candidato,
            );
            // registrar usario votante
            sistema.registrar_usuario_priv(
                AccountId::from([0x03; 32]),
                "Agustin".to_string(),
                " ".to_string(),
                RolUsuario::Votante,
            );

            sistema.registrar_usuario_priv(
                AccountId::from([0x04; 32]),
                "Agustin".to_string(),
                " ".to_string(),
                RolUsuario::Votante,
            );

            // registrar candidato en eleccion
            sistema
                .registrar_candidato_en_eleccion_priv(AccountId::from([0x02; 32]), 0)
                .unwrap();
            // registrar votante en eleccion
            sistema
                .registrar_votante_en_eleccion_priv(AccountId::from([0x03; 32]), 0)
                .unwrap();
            sistema
                .registrar_votante_en_eleccion_priv(AccountId::from([0x04; 32]), 0)
                .unwrap();
            ink_env::test::set_block_timestamp::<ink_env::DefaultEnvironment>(1719878400000);
            // Votar
            sistema.votar_priv(AccountId::from([0x03; 32]), 0, AccountId::from([0x02; 32]));
            assert!(sistema
                .votar_priv(AccountId::from([0x02; 32]), 0, AccountId::from([0x02; 32]))
                .is_err());
            assert!(sistema
                .votar_priv(AccountId::from([0x01; 32]), 0, AccountId::from([0x02; 32]))
                .is_err());
            assert!(sistema
                .votar_priv(AccountId::from([0x04; 32]), 1, AccountId::from([0x03; 32]))
                .is_err());
        }
        #[ink::test]
        fn test_votar_en_eleccion() {}
        #[test]
        fn test_display_formatting() {
            use std::fmt::Write;

            // Define casos de prueba para cada variante de Error junto con el mensaje esperado
            let test_cases = [
                (Error::PermisoDenegado, "Permiso denegado"),
                (Error::EleccionNoExiste, "La elección no existe"),
                (Error::EleccionAbierta, "La elección está abierta"),
                (Error::EleccionNoActiva, "La elección no está activa"),
                (Error::UsuarioNoVotante, "El usuario no es votante"),
                (Error::UsuarioYaRegistrado, "El usuario ya está registrado"),
                (
                    Error::AdminNoPuedeRegistrarse,
                    "El admin no puede registrarse",
                ),
                (Error::AdminNoPuedeVotar, "El admin no puede votar"),
                (Error::CandidatoNoExiste, "El candidato no existe"),
                (Error::UsuarioNoCandidato, "El usuario no es candidato"),
                (Error::FechaInvalida, "Fecha inválida"),
                (Error::Overflow, "Overflow"),
            ];

            // Itera sobre cada caso de prueba
            for (error, expected_message) in test_cases.iter() {
                // Crea un formatter para almacenar el resultado del formateo
                let mut formatted = String::new();

                // Formatea el error utilizando el trait Display
                write!(formatted, "{}", error.clone()).expect("Formatting should succeed");

                // Verifica que el mensaje formateado coincida con el mensaje esperado
                assert_eq!(formatted, *expected_message);
            }
        }
    }

    #[cfg(all(test, feature = "e2e-tests"))]
    mod e2e_tests {
        use super::*;
        use ink::primitives::AccountId;
        use ink_e2e::ContractsBackend;

        #[ink_e2e::test]
        async fn test_votacion<Client: E2EBackend>(mut client: Client) -> E2EResult<()> {
            let mut constructor = SistemaVotacion::new();
            let contract = client
                .instantiate("SistemaVotacion", &ink_e2e::bob(), &mut constructor)
                .submit()
                .await
                .expect("Failed to instantiate contract");
            let call_builder = contract.call_builder::<SistemaVotacion>();

            let fehcai = Fecha {
                dias: 7,
                mes: 6,
                anio: 2024,
            };
            let fehcaf = Fecha {
                dias: 7,
                mes: 6,
                anio: 2025,
            };
            call_builder
                .crear_eleccion("cargo".to_string(), fehcai, fehcaf)
                .exec()
                .await?;
            Ok(())
        }
    }
}
