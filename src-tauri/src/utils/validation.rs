// Módulo 11 con pesos [3, 7, 13, 17, 19, 23, 29, 37, 41, 43, 47, 53, 59, 67, 71]
// Algoritmo oficial de validación de Dígito de Verificación (DV) de la DIAN para Colombia.

/// Calcula el Dígito de Verificación (DV) oficial de la DIAN para un NIT (base numérica).
pub fn calculate_dv(base: &str) -> u8 {
    let weights = [3, 7, 13, 17, 19, 23, 29, 37, 41, 43, 47, 53, 59, 67, 71];
    let mut sum = 0;

    // Iterar sobre los caracteres en orden inverso (de derecha a izquierda)
    for (i, c) in base.chars().rev().enumerate() {
        if let Some(digit) = c.to_digit(10) {
            let weight = weights[i % weights.len()];
            sum += digit * weight;
        }
    }

    let remainder = sum % 11;
    if remainder <= 1 {
        remainder as u8
    } else {
        (11 - remainder) as u8
    }
}

/// Limpia y parsea un NIT para separar la parte base y el Dígito de Verificación (si está provisto).
/// Retorna `(base, Option<dv>)`.
pub fn parse_nit(nit_str: &str) -> Result<(String, Option<u8>), String> {
    // Eliminar puntos, espacios y comas
    let cleaned: String = nit_str
        .chars()
        .filter(|c| *c != '.' && *c != ' ' && *c != ',')
        .collect();

    if cleaned.is_empty() {
        return Err("El NIT no puede estar vacío.".to_string());
    }

    // Dividir por guion si existe
    let parts: Vec<&str> = cleaned.split('-').collect();
    if parts.len() > 2 {
        return Err("El NIT no puede contener múltiples guiones.".to_string());
    }

    if parts.len() == 2 {
        let base = parts[0];
        let dv_str = parts[1];
        if base.is_empty() || !base.chars().all(|c| c.is_digit(10)) {
            return Err("La parte base del NIT debe contener únicamente dígitos numéricos.".to_string());
        }
        if dv_str.len() != 1 || !dv_str.chars().all(|c| c.is_digit(10)) {
            return Err("El dígito de verificación debe ser un único número de 0 a 9.".to_string());
        }
        let dv = dv_str.chars().next().unwrap().to_digit(10).unwrap() as u8;
        Ok((base.to_string(), Some(dv)))
    } else {
        // Sin guion: todo el string es la base numérica (Cédula o NIT de cualquier longitud)
        let digits = parts[0];
        if digits.is_empty() || !digits.chars().all(|c| c.is_digit(10)) {
            return Err("El NIT debe contener únicamente dígitos numéricos.".to_string());
        }

        // Se trata como un número base simple sin dígito de verificación
        Ok((digits.to_string(), None))
    }
}

/// Valida un NIT colombiano en base al algoritmo de Módulo 11 de la DIAN.
/// Si se provee un DV, verifica que coincida con el calculado.
pub fn validate_nit(nit_str: &str) -> Result<(), String> {
    let (base, dv) = parse_nit(nit_str)?;
    if let Some(provided_dv) = dv {
        let expected_dv = calculate_dv(&base);
        if provided_dv != expected_dv {
            return Err(format!(
                "Dígito de verificación incorrecto para el NIT {}. Esperado: {}, Ingresado: {}.",
                base, expected_dv, provided_dv
            ));
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_dv() {
        // Bancolombia NIT: 890.900.943 -> DV: 1
        assert_eq!(calculate_dv("890900943"), 1);
        // Banco de Bogotá NIT: 860.002.964 -> DV: 4
        assert_eq!(calculate_dv("860002964"), 4);
        // DIAN NIT: 800.197.268 -> DV: 4
        assert_eq!(calculate_dv("800197268"), 4);
    }

    #[test]
    fn test_parse_nit_with_hyphen() {
        let parsed = parse_nit("800.197.268-4").unwrap();
        assert_eq!(parsed, ("800197268".to_string(), Some(4)));

        let parsed = parse_nit("890900943-1").unwrap();
        assert_eq!(parsed, ("890900943".to_string(), Some(1)));
    }

    #[test]
    fn test_parse_nit_ten_digits() {
        let parsed = parse_nit("1066270400").unwrap();
        assert_eq!(parsed, ("1066270400".to_string(), None));

        let parsed_with_hyphen = parse_nit("1066270400-1").unwrap();
        assert_eq!(parsed_with_hyphen, ("1066270400".to_string(), Some(1)));
    }

    #[test]
    fn test_parse_nit_simple_number() {
        let parsed = parse_nit("800197268").unwrap();
        assert_eq!(parsed, ("800197268".to_string(), None));
    }

    #[test]
    fn test_validate_nit_success() {
        // Valid inputs
        assert!(validate_nit("800197268-4").is_ok());
        assert!(validate_nit("1066270400-1").is_ok());
        assert!(validate_nit("1066270400").is_ok());
        assert!(validate_nit("800197268").is_ok());
        assert!(validate_nit("890.900.943-1").is_ok());
        assert!(validate_nit("860002964-4").is_ok());
    }

    #[test]
    fn test_validate_nit_failure() {
        // Invalid DV
        assert!(validate_nit("800197268-8").is_err());
        assert!(validate_nit("1066270400-9").is_err());
        assert!(validate_nit("890900943-5").is_err());

        // Invalid format
        assert!(validate_nit("abc").is_err());
        assert!(validate_nit("800197268-").is_err());
        assert!(validate_nit("-4").is_err());
    }
}
