use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, sqlx::FromRow, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Proveedor {
    pub id: String,
    pub num_doc: String,
    pub tipo_doc: String,
    pub email: Option<String>,
    pub contacto: Option<String>,
    pub status: String,
    pub pais: Option<String>,
    pub nombre: String,
    pub apellido: Option<String>,
    pub telefono1: Option<String>,
    pub telefono2: Option<String>,
    pub direccion1: Option<String>,
    pub ciudad: Option<String>,
    pub departamento: Option<String>,
    pub resp_fisc: Option<String>,
    pub tax_scheme: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct CreateProveedorInput {
    pub num_doc: String,
    pub tipo_doc: String,
    pub nombre: String,
    pub apellido: Option<String>,
    pub telefono1: Option<String>,
    pub telefono2: Option<String>,
    pub email: Option<String>,
    pub contacto: Option<String>,
    pub direccion1: Option<String>,
    pub ciudad: Option<String>,
    pub departamento: Option<String>,
    pub resp_fisc: Option<String>,
    pub tax_scheme: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct UpdateProveedorInput {
    pub nombre: String,
    pub apellido: Option<String>,
    pub telefono1: Option<String>,
    pub telefono2: Option<String>,
    pub email: Option<String>,
    pub contacto: Option<String>,
    pub direccion1: Option<String>,
    pub ciudad: Option<String>,
    pub departamento: Option<String>,
    pub status: Option<String>,
    pub resp_fisc: Option<String>,
    pub tax_scheme: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct DeleteResult {
    pub success: bool,
    pub action: String,
    pub reason: String,
    pub message: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_proveedor_serialization() {
        let p = Proveedor {
            id: "123".to_string(),
            num_doc: "900123".to_string(),
            tipo_doc: "N".to_string(),
            email: Some("test@test.com".to_string()),
            contacto: None,
            status: "A".to_string(),
            pais: Some("CO".to_string()),
            nombre: "EMPRESA SAS".to_string(),
            apellido: None,
            telefono1: Some("1234".to_string()),
            telefono2: None,
            direccion1: None,
            ciudad: None,
            departamento: None,
            resp_fisc: Some("O-99,".to_string()),
            tax_scheme: Some("ZZ,".to_string()),
        };

        let serialized = serde_json::to_string(&p).unwrap();
        assert!(serialized.contains(r#""numDoc":"900123""#));
        assert!(serialized.contains(r#""tipoDoc":"N""#));
        assert!(serialized.contains(r#""email":"test@test.com""#));
        assert!(serialized.contains(r#""contacto":null"#));

        let deserialized: Proveedor = serde_json::from_str(&serialized).unwrap();
        assert_eq!(deserialized, p);
    }
}
