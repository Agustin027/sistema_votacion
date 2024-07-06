/*

*/

#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod reporte {
    use ink::prelude::collections::BTreeMap;
    use ink::prelude::vec::Vec;
    use sistema_votacion::{Error, RolUsuario, SistemaVotacionRef, Usuario};

    #[ink(storage)]
    pub struct Reporte {
        #[cfg(not(test))]
        //campo que solo va a existir en ambiente de producción y no en ambiente de testing para poder mockearlo en los tests y que no me salga error de mismatch de tipos cuando intento instaciar un reporte
        sistema_votacion: SistemaVotacionRef,
    }

    impl Reporte {
        #[cfg(not(test))]
        #[ink(constructor)]
        pub fn new(sistema_votacion: SistemaVotacionRef) -> Self {
            Self { sistema_votacion }
        }

        //----------------------------------- Funciones Mock con valores de prueba------------------------------------------------------------
        #[cfg(test)]
        pub fn new() -> Self {
            Self {}
        }

        #[cfg(test)]
        fn get_votantes(&self, _id: u64) -> Result<Vec<Usuario>, Error> {
            let usuario1 = Usuario::new(
                AccountId::from([0x1; 32]),
                "Agustin".to_string(),
                "mail".to_string(),
                RolUsuario::Votante,
            );
            let usuario2 = Usuario::new(
                AccountId::from([0x2; 32]),
                "Fer".to_string(),
                "mail".to_string(),
                RolUsuario::Votante,
            );
            Ok(vec![usuario1, usuario2])
        }

        #[cfg(test)]
        fn get_tamanio_elecciones(&self) -> Result<u64, Error> {
            Ok(2)
        }

        #[cfg(test)]
        fn get_fecha_fin(&self, _id: u64) -> Result<u64, Error> {
            Ok(100000)
        }

        #[cfg(test)]
        fn get_fecha_inicio(&self, _id: u64) -> Result<u64, Error> {
            Ok(500)
        }

        #[cfg(test)]
        fn get_votantes_que_votaron(&self, _id: u64) -> Result<Vec<Usuario>, Error> {
            let usuario1 = Usuario::new(
                AccountId::from([0x1; 32]),
                "Agustin".to_string(),
                "mail".to_string(),
                RolUsuario::Votante,
            );
            let usuario2 = Usuario::new(
                AccountId::from([0x2; 32]),
                "Fer".to_string(),
                "mail".to_string(),
                RolUsuario::Votante,
            );

            Ok(vec![usuario1, usuario2])
        }

        #[cfg(test)]
        fn get_candidatos(&self, _id: u64) -> Result<BTreeMap<AccountId, u64>, Error> {
            let mut candidatos = BTreeMap::new();
            candidatos.insert(AccountId::from([0x1; 32]), 10);
            candidatos.insert(AccountId::from([0x2; 32]), 5);
            candidatos.insert(AccountId::from([0x3; 32]), 3);
            Ok(candidatos)
        }
        //------------------------------------------------------------------------------------------------------
        #[cfg(not(test))]
        fn get_votantes(&self, id: u64) -> Result<Vec<Usuario>, Error> {
            self.sistema_votacion.get_votantes(id)
        }

        #[cfg(not(test))]
        fn get_tamanio_elecciones(&self) -> Result<u64, Error> {
            self.sistema_votacion.get_tamanio_elecciones()
        }

        #[cfg(not(test))]
        fn get_fecha_fin(&self, id: u64) -> Result<u64, Error> {
            self.sistema_votacion.get_fecha_fin(id)
        }

        #[cfg(not(test))]
        fn get_fecha_inicio(&self, id: u64) -> Result<u64, Error> {
            self.sistema_votacion.get_fecha_inicio(id)
        }

        #[cfg(not(test))]
        fn get_votantes_que_votaron(&self, id: u64) -> Result<Vec<Usuario>, Error> {
            self.sistema_votacion.get_votantes_que_votaron(id)
        }

        #[cfg(not(test))]
        fn get_candidatos(&self, id: u64) -> Result<BTreeMap<AccountId, u64>, Error> {
            self.sistema_votacion.get_candidatos(id)
        }

        fn generar_reporte_registro_votantes_priv(
            &self,
            id: u64,
        ) -> Result<ReporteRegistroVotantes, Error> {
            //verifica que la elección exista
            if id >= self.get_tamanio_elecciones()? {
                return Err(Error::EleccionNoExiste);
            }

            //traigo los votantes registrados en la elección desde el contrato sistema_votacion
            let votantes_reg = self.get_votantes(id)?;

            //creo el reporte
            let reporte_registro_votantes = ReporteRegistroVotantes {
                nro_eleccion: id,
                votantes: votantes_reg,
            };
            Ok(reporte_registro_votantes)
        }

        #[ink(message)]
        // Genera un reporte de los votantes registrados en una elección
        pub fn generar_reporte_registro_votantes(
            &self,
            id: u64,
        ) -> Result<ReporteRegistroVotantes, Error> {
            self.generar_reporte_registro_votantes_priv(id)
        }

        fn generar_reporte_participacion_priv(
            &self,
            id: u64,
        ) -> Result<ReporteParticipacion, Error> {
            let fecha_cierre = self.get_fecha_fin(id)?;
            let fecha_inicio = self.get_fecha_inicio(id)?;
            let fecha_actual = self.env().block_timestamp();
            //verifica que la elección ya haya iniciado
            if fecha_actual < fecha_inicio {
                return Err(Error::EleccionNoActiva);
            }
            //verifica que la elección ya haya cerrado
            if fecha_actual < fecha_cierre {
                return Err(Error::EleccionAbierta);
            }

            //traigo la cantidad de votos emitidos desde el contrato sistema_votacion y la cantidad de votantes registrados
            let cantidad_votos_emitidos = self.get_votantes_que_votaron(id)?.len() as u64;

            let cantidad_votantes = self.get_votantes(id)?.len() as u64;

            //calculo el porcentaje de participación
            let mut porcentaje_participacion = cantidad_votos_emitidos
                .checked_mul(100)
                .ok_or(Error::Overflow)?;

            porcentaje_participacion = porcentaje_participacion
                .checked_div(cantidad_votantes)
                .ok_or(Error::Overflow)?;

            let reporte_participacion = ReporteParticipacion {
                nro_eleccion: id,
                cantidad_votos_emitidos,
                porcentaje_participacion,
            };

            Ok(reporte_participacion)
        }

        #[ink(message)]
        // Genera un reporte de la participación en una elección
        pub fn generar_reporte_participacion(
            &self,
            id: u64,
        ) -> Result<ReporteParticipacion, Error> {
            self.generar_reporte_participacion_priv(id)
        }

        fn generar_reporte_resultado_priv(&self, id: u64) -> Result<ReporteResultado, Error> {
            let fecha_cierre = self.get_fecha_fin(id)?;
            let fecha_inicio = self.get_fecha_inicio(id)?;
            let fecha_actual = self.env().block_timestamp();

            //verifica que la elección ya haya cerrado
            if fecha_actual < fecha_cierre {
                return Err(Error::EleccionAbierta);
            }
            //verifica que la elección ya haya iniciado
            if fecha_actual < fecha_inicio {
                return Err(Error::EleccionNoActiva);
            }

            //traigo los resultados de la elección desde el contrato sistema_votacion y los ordeno
            let resultados_desordenados = self.get_candidatos(id)?;
            let mut resultados_ordenados = resultados_desordenados.into_iter().collect::<Vec<_>>();
            resultados_ordenados.sort_by(|a, b| b.1.cmp(&a.1));

            let reporte_resultado = ReporteResultado {
                nro_eleccion: id,
                resultados_ordenados,
            };

            Ok(reporte_resultado)
        }

        #[ink(message)]
        // Genera un reporte de los resultados de una elección
        pub fn generar_reporte_resultado(&self, id: u64) -> Result<ReporteResultado, Error> {
            self.generar_reporte_resultado_priv(id)
        }
    }

    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    #[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct ReporteRegistroVotantes {
        nro_eleccion: u64,
        votantes: Vec<Usuario>,
    }

    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    #[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct ReporteParticipacion {
        nro_eleccion: u64,
        cantidad_votos_emitidos: u64,
        porcentaje_participacion: u64,
    }

    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    #[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct ReporteResultado {
        nro_eleccion: u64,
        resultados_ordenados: Vec<(AccountId, u64)>,
    }

    #[cfg(test)]
    mod tests {

        use super::*;

        #[ink::test]
        fn test_generar_reporte_registro_votantes() {
            let reporte = Reporte::new();

            assert!(reporte.generar_reporte_registro_votantes(1).is_ok());
            assert!(reporte.generar_reporte_registro_votantes(3).is_err());
            let reporte_registro_votantes = ReporteRegistroVotantes {
                nro_eleccion: 1,
                votantes: vec![
                    Usuario::new(
                        AccountId::from([0x1; 32]),
                        "Agustin".to_string(),
                        "mail".to_string(),
                        RolUsuario::Votante,
                    ),
                    Usuario::new(
                        AccountId::from([0x2; 32]),
                        "Fer".to_string(),
                        "mail".to_string(),
                        RolUsuario::Votante,
                    ),
                ],
            };
            assert_eq!(
                reporte.generar_reporte_registro_votantes(1).unwrap(),
                reporte_registro_votantes
            );
        }
        #[ink::test]
        fn test_generar_reporte_participacion() {
            let reporte = Reporte::new();

            ink_env::test::set_block_timestamp::<ink_env::DefaultEnvironment>(300);
            assert!(reporte.generar_reporte_participacion(1).is_err());
            ink_env::test::set_block_timestamp::<ink_env::DefaultEnvironment>(600);
            assert!(reporte.generar_reporte_participacion(1).is_err());
            ink_env::test::set_block_timestamp::<ink_env::DefaultEnvironment>(10000000);
            assert!(reporte.generar_reporte_participacion(2).is_ok());

            let reporte_participacion = ReporteParticipacion {
                nro_eleccion: 2,
                cantidad_votos_emitidos: 2,
                porcentaje_participacion: 100,
            };
            assert_eq!(
                reporte.generar_reporte_participacion(2).unwrap(),
                reporte_participacion
            );
        }
        #[ink::test]
        fn test_generar_reporte_resultado() {
            let reporte = Reporte::new();
            ink_env::test::set_block_timestamp::<ink_env::DefaultEnvironment>(300);
            assert!(reporte.generar_reporte_resultado(1).is_err());

            ink_env::test::set_block_timestamp::<ink_env::DefaultEnvironment>(600);
            assert!(reporte.generar_reporte_resultado(1).is_err());

            ink_env::test::set_block_timestamp::<ink_env::DefaultEnvironment>(10000000);
            assert!(reporte.generar_reporte_resultado(2).is_ok());
            let reporte_resultado = ReporteResultado {
                nro_eleccion: 2,
                resultados_ordenados: vec![
                    (AccountId::from([0x1; 32]), 10),
                    (AccountId::from([0x2; 32]), 5),
                    (AccountId::from([0x3; 32]), 3),
                ],
            };
            assert_eq!(
                reporte.generar_reporte_resultado(2).unwrap(),
                reporte_resultado
            );
        }
    }
}
