/*
    -Hacer Test
    -En los reportes verificar que la eleccion este cerrada
*/

#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod reporte {
    use ink::prelude::collections::BTreeMap;
    use ink::prelude::string::String;
    use ink::prelude::vec::Vec;
    use sistema_votacion::{Eleccion, SistemaVotacionRef, Usuario};

    #[ink(storage)]
    pub struct Reporte {
        sistema_votacion: SistemaVotacionRef,
    }

    impl Reporte {
        #[ink(constructor)]
        pub fn new(sistema_votacion: SistemaVotacionRef) -> Self {
            Self { sistema_votacion }
        }

        #[ink(message)]
        pub fn reportar(&self, id: u64) -> BTreeMap<AccountId, u64> {
            let ret = self.sistema_votacion.mostrar_resultados(id).unwrap();
            ret
        }

        #[ink(message)]
        pub fn generar_reporte_registro_votantes(&self, id: u64) -> ReporteRegistroVotantes {
            let votantes_reg = self.sistema_votacion.get_votantes(id);
            let reporte_registro_votantes = ReporteRegistroVotantes {
                nro_eleccion: id,
                votantes: votantes_reg,
            };
            reporte_registro_votantes
        }

        #[ink(message)]
        pub fn generar_reporte_participacion(&self, id: u64) -> ReporteParticipacion {
            // TO Do agregar verificacion de eleccion cerrada
            let cantidad_votos_emitidos =
                self.sistema_votacion.get_votantes_que_votaron(id).len() as u64;
            let cantidad_votantes = self.sistema_votacion.get_votantes(id).len() as u64;

            //despues manejar el error bien y no unwrap
            let porcentaje_participacion = cantidad_votos_emitidos.checked_mul(100).unwrap_or(0);

            porcentaje_participacion
                .checked_div(cantidad_votantes)
                .unwrap_or(0);

            let reporte_participacion = ReporteParticipacion {
                nro_eleccion: id,
                cantidad_votos_emitidos,
                porcentaje_participacion,
            };
            reporte_participacion
        }

        #[ink(message)]
        pub fn generar_reporte_resultado(&self, id: u64) -> ReporteResultado {
            // TO Do agregar verificacion de eleccion cerrada
            let resultados_desordenados = self.sistema_votacion.mostrar_resultados(id).unwrap();
            let mut resultados_ordenados = resultados_desordenados.into_iter().collect::<Vec<_>>();
            resultados_ordenados.sort_by(|a, b| b.1.cmp(&a.1));

            let reporte_resultado = ReporteResultado {
                nro_eleccion: id,
                resultados_ordenados,
            };

            reporte_resultado
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
    pub struct ReporteResultado {
        nro_eleccion: u64,
        resultados_ordenados: Vec<(AccountId, u64)>,
    }
}
