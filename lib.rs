/*To do
- HACER TESTS

 */

/*Cosas a consultar
No se si mi funcion de votar esta bien recibiendo el accounid del candidato,
por que no salen los candidatos en la lista de elecciones, solo salen las wallets propias.

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

        fn crear_eleccion_priv(
            &mut self,
            cargo: String,
            fecha_ini: Fecha,
            fecha_f: Fecha,
        ) -> Result<(), Error> {
            if self.admin.id != self.env().caller() {
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
            self.set_admin_priv(nombre, email, password, nuevo_admin);
        }

        fn set_admin_priv(
            &mut self,
            nombre: String,
            email: String,
            password: String,
            nuevo_admin: AccountId,
        ) -> Result<(), Error> {
            let caller = self.env().caller();
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

        //----------------------Funciones de registro---------------------------------------------------------

        #[ink(message)]
        /// Funcion para registrar un usuario en el sistema con los datos ingresados,distingue entre votante y candidato y verifica que no se registre el admin como usuario normal
        pub fn registrar_usuario(&mut self, nombre: String, email: String, rol: RolUsuario) {
            self.registrar_usuario_priv(nombre, email, rol);
        }

        fn registrar_usuario_priv(
            &mut self,
            nombre: String,
            email: String,
            rol: RolUsuario,
        ) -> Result<(), Error> {
            let usuario = Usuario {
                id: self.env().caller(),
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
            self.registrar_votante_en_eleccion_priv(id_eleccion);
        }
        pub fn registrar_votante_en_eleccion_priv(
            &mut self,
            id_eleccion: u64,
        ) -> Result<(), Error> {
            let caller = self.env().caller();

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
            self.registrar_candidato_en_eleccion_priv(id_eleccion);
        }
        fn registrar_candidato_en_eleccion_priv(&mut self, id_eleccion: u64) -> Result<(), Error> {
            let caller = self.env().caller();

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
            self.votar_priv(id_eleccion, id_candidato);
        }
        fn votar_priv(&mut self, id_eleccion: u64, id_candidato: AccountId) -> Result<(), Error> {
            let caller = self.env().caller();

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
        fn contar_votos(&self, id_eleccion: u64) -> Result<u64, Error> {
            if !self.eleccion_cerrada(id_eleccion)? {
                return Err(Error::EleccionAbierta);
            }

            let mut votos_totales: u64 = 0;
            for (_, votos) in self
                .elecciones
                .get(id_eleccion as usize)
                .ok_or(Error::EleccionNoExiste)?
                .candidatos
                .iter()
            {
                votos_totales = votos_totales.checked_add(*votos).ok_or(Error::Overflow)?;
            }
            Ok(votos_totales)
        }
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
                // Verificar que el votante no haya votado ya
                /*  esto lo tengo comentado por que quiero testear el sistema y alta paja crear muchas wallets para hacerlo CAMBIAR!!!!!!
                    if self.votantes_que_votaron.contains(&votante) {
                    panic!("El votante ya ha votado");
                }*/

                // Intentar incrementar el conteo de votos, manejando el posible overflow
                *votos = votos.checked_add(1).ok_or(Error::Overflow)?;

                self.votantes_que_votaron.push(votante);
            } else {
                return Err(Error::CandidatoNoExiste);
            }
            Ok(())
        }
    }
    //----------------------Funciones de verificacion --------------------------------------------------------

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
