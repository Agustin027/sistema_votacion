/*

*/

#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod reporte {
    use ink::prelude::collections::BTreeMap;
    use ink::prelude::vec::Vec;
    use sistema_votacion::{Error, SistemaVotacionRef, Usuario};

    #[ink(storage)]
    pub struct Reporte {
        sistema_votacion: SistemaVotacionRef,
    }

    impl Reporte {
        #[ink(constructor)]
        pub fn new(sistema_votacion: SistemaVotacionRef) -> Self {
            Self { sistema_votacion }
        }

        fn generar_reporte_registro_votantes_priv(
            &self,
            id: u64,
        ) -> Result<ReporteRegistroVotantes, Error> {
            //verifica que la elección exista
            if id > self.sistema_votacion.get_tamanio_elecciones()? {
                return Err(Error::EleccionNoExiste);
            }

            //traigo los votantes registrados en la elección desde el contrato sistema_votacion
            let votantes_reg = self.sistema_votacion.get_votantes(id)?;

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
            let fecha_cierre = self.sistema_votacion.get_fecha_fin(id)?;
            let fecha_inicio = self.sistema_votacion.get_fecha_inicio(id)?;
            let fecha_actual = self.env().block_timestamp();

            //verifica que la elección ya haya cerrado
            if fecha_actual < fecha_cierre {
                return Err(Error::EleccionAbierta);
            }
            //verifica que la elección ya haya iniciado
            if fecha_actual < fecha_inicio {
                return Err(Error::EleccionNoActiva);
            }

            //traigo la cantidad de votos emitidos desde el contrato sistema_votacion y la cantidad de votantes registrados
            let cantidad_votos_emitidos =
                self.sistema_votacion.get_votantes_que_votaron(id)?.len() as u64;

            let cantidad_votantes = self.sistema_votacion.get_votantes(id)?.len() as u64;

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
            let fecha_cierre = self.sistema_votacion.get_fecha_fin(id)?;
            let fecha_inicio = self.sistema_votacion.get_fecha_inicio(id)?;
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
            let resultados_desordenados = self.sistema_votacion.get_candidatos(id)?;
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
        // no supe como simular la data que recibo del contrato sistema_votacion y no pude hacer los test
    }
}
