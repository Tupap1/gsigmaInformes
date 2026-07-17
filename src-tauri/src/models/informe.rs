use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, sqlx::FromRow, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct CompraAcumulada {
    pub pas: String,
    pub nombre: String,
    pub cantidad: f64,
    pub total: f64,
    pub costo_promedio: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ResumenCaja {
    pub base_caja: f64,
    pub ingresos: f64,
    pub ventas_contado: f64,
    pub ventas_credito: f64,
    pub compras: f64,
    pub egresos: f64,
    pub caja_efectivo: f64,
    pub caja_total: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone, sqlx::FromRow, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Producto {
    pub pas: String,
    pub pasnom: String,
}
